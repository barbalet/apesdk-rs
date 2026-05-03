#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/c-engine-trace-probe"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"
COMPILER="${CC:-gcc}"
CFLAGS_TEXT="${CFLAGS:--O2}"
DEFINES_TEXT="${DEFINES:--DCOMMAND_LINE_EXPLICIT -DAUTOMATED}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

read -r -a CFLAGS_ARGS <<< "$CFLAGS_TEXT"
read -r -a DEFINES_ARGS <<< "$DEFINES_TEXT"
DEFINES_ARGS+=("-DFULL_DATE=\"$FULL_DATE\"")

BUILD_DIR="$OUT_DIR/objects"
mkdir -p "$BUILD_DIR"

PROBE="$OUT_DIR/c_engine_trace_probe.c"
cat > "$PROBE" <<'PROBE_C'
#include <stdio.h>

#include "toolkit/toolkit.h"
#include "sim/sim.h"
#include "entity/entity.h"
#include "universe/universe.h"

#ifndef FIXED_RANDOM_SIM
#define FIXED_RANDOM_SIM 0x5261f726
#endif

void food_values(n_int loc_x, n_int loc_y, n_int *grass, n_int *trees, n_int *bush);

n_int draw_error(n_constant_string error_text, n_constant_string location, n_int line_number)
{
    printf("ERROR: %s @ %s %ld\n", (const n_string)error_text, location, line_number);
    return -1;
}

static const n_byte2 terrain_locations[][2] =
{
    {0, 0},
    {64, 64},
    {3200, 9600},
    {12345, 23456},
    {16384, 16384},
    {28000, 4000},
    {32704, 32704},
    {24576, 8192}
};

static n_int terrain_biology_value(n_int loc_x, n_int loc_y, n_int kind)
{
    return land_operator_interpolated(loc_x, loc_y, (n_byte *)&operators[kind - VARIABLE_BIOLOGY_AREA]);
}

static n_int terrain_highres_tide_bit(n_int high_x, n_int high_y)
{
    n_byte4 *highres_tide = land_highres_tide();
    n_int index = ((high_x & (HI_RES_MAP_DIMENSION - 1)) +
                   ((high_y & (HI_RES_MAP_DIMENSION - 1)) << HI_RES_MAP_BITS));
    return (highres_tide[index >> 5] >> (index & 31)) & 1;
}

static void terrain_food_classification(n_int loc_x, n_int loc_y, n_byte *food_type, n_int *max_energy)
{
    n_int height = land_location(APESPACE_TO_MAPSPACE(loc_x), APESPACE_TO_MAPSPACE(loc_y));

    *food_type = FOOD_VEGETABLE;
    *max_energy = BEING_DEAD;

    if (height > TIDE_MAX)
    {
        n_int grass = 0, trees = 0, bush = 0;
        food_values(loc_x, loc_y, &grass, &trees, &bush);
        if ((grass > bush) && (grass > trees))
        {
            *max_energy = ENERGY_GRASS;
        }
        else if (bush > trees)
        {
            *max_energy = ENERGY_BUSH;
        }
        else
        {
            *food_type = FOOD_FRUIT;
            *max_energy = ENERGY_FRUIT;
        }
    }
    else
    {
        n_int seaweed = terrain_biology_value(loc_x, loc_y, VARIABLE_BIOLOGY_SEAWEED);
        n_int rockpool = terrain_biology_value(loc_x, loc_y, VARIABLE_BIOLOGY_ROCKPOOL);
        n_int beach = terrain_biology_value(loc_x, loc_y, VARIABLE_BIOLOGY_BEACH);
        beach += LAND_DITHER(seaweed, rockpool, beach);

        if ((seaweed > rockpool) && (seaweed > beach))
        {
            *food_type = FOOD_SEAWEED;
            *max_energy = ENERGY_SEAWEED;
        }
        else if (rockpool > beach)
        {
            *food_type = FOOD_SHELLFISH;
            *max_energy = ENERGY_SHELLFISH;
        }
    }
}

static void emit_terrain_sample(const char *label, n_int sample, n_int loc_x, n_int loc_y)
{
    n_byte *topography = land_topography();
    n_byte *highdef = land_topography_highdef();
    n_int map_x = APESPACE_TO_MAPSPACE(loc_x);
    n_int map_y = APESPACE_TO_MAPSPACE(loc_y);
    n_int high_x = map_x << 3;
    n_int high_y = map_y << 3;
    n_int map_index = (map_x & (MAP_DIMENSION - 1)) + ((map_y & (MAP_DIMENSION - 1)) << MAP_BITS);
    n_int high_index = (high_x & (HI_RES_MAP_DIMENSION - 1)) +
                       ((high_y & (HI_RES_MAP_DIMENSION - 1)) << HI_RES_MAP_BITS);
    n_int grass = 0, trees = 0, bush = 0;
    n_int seaweed = terrain_biology_value(loc_x, loc_y, VARIABLE_BIOLOGY_SEAWEED);
    n_int rockpool = terrain_biology_value(loc_x, loc_y, VARIABLE_BIOLOGY_ROCKPOOL);
    n_int beach = terrain_biology_value(loc_x, loc_y, VARIABLE_BIOLOGY_BEACH);
    n_byte food_type = FOOD_VEGETABLE;
    n_int max_energy = BEING_DEAD;
    n_int height = land_location(map_x, map_y);
    n_int terrain_class = height <= land_tide_level() ? 0 : (height <= TIDE_MAX ? 1 : 2);

    food_values(loc_x, loc_y, &grass, &trees, &bush);
    beach += LAND_DITHER(seaweed, rockpool, beach);
    terrain_food_classification(loc_x, loc_y, &food_type, &max_energy);

    printf(
        "TERRAIN snapshot=%s sample=%ld loc=%ld:%ld map=%ld:%ld class=%ld "
        "height=%ld topo=%u high=%u hightide=%ld weather=%d pressure=%ld "
        "food=%ld:%ld:%ld intertidal=%ld:%ld:%ld source=%u:%ld\n",
        label,
        sample,
        loc_x,
        loc_y,
        map_x,
        map_y,
        terrain_class,
        height,
        topography[map_index],
        highdef[high_index],
        terrain_highres_tide_bit(high_x, high_y),
        weather_seven_values(loc_x, loc_y),
        weather_pressure(map_x, map_y),
        grass,
        trees,
        bush,
        seaweed,
        rockpool,
        beach,
        food_type,
        max_energy);
}

static void emit_terrain_samples(const char *label)
{
    n_int sample = 0;
    while (sample < (n_int)(sizeof(terrain_locations) / sizeof(terrain_locations[0])))
    {
        emit_terrain_sample(label, sample, terrain_locations[sample][0], terrain_locations[sample][1]);
        sample++;
    }
}

static void emit_snapshot(const char *label)
{
    n_byte2 *genetics = land_genetics();
    n_byte *topography = land_topography();
    n_byte *highdef = land_topography_highdef();
    n_byte4 *highres_tide = land_highres_tide();
    n_int grass = 0, trees = 0, bush = 0;
    simulated_group *group = sim_group();
    simulated_being *being = 0L;

    food_values(0, 0, &grass, &trees, &bush);
    if ((group != 0L) && (group->num > 0))
    {
        being = group->select;
        if (being == 0L)
        {
            being = &(group->beings[0]);
        }
    }

    printf(
        "TRACE snapshot=%s date=%lu time=%lu genetics=%u:%u tide=%u "
        "topo=%u:%u:%u high=%u:%u hightide=%lu weather=%d pressure=%ld food=%ld:%ld:%ld "
        "population=%lu selected=%u name=%u:%u energy=%ld loc=%ld:%ld facing=%u velocity=%u:%u honor=%u "
        "height=%ld mass=%ld posture=%u awake=%u drives=%u:%u:%u:%u brain=%u:%u:%u "
        "probe0=%u:%u:%u:%u:%u:%u social0=%u:%u:%u:%u:%u:%u episodic0=%u:%u:%u:%u "
        "territory0=%u:%u conception=%lu family=%u:%u:%u:%u:%u:%u immune=%u:%u:%u:%u:%u:%u "
        "inventory=%u:%u shout=%u preference=%u\n",
        label,
        (unsigned long)land_date(),
        (unsigned long)land_time(),
        genetics[0],
        genetics[1],
        land_tide_level(),
        topography[0],
        topography[MAP_DIMENSION + 1],
        topography[(MAP_DIMENSION / 2) + ((MAP_DIMENSION / 2) * MAP_DIMENSION)],
        highdef[0],
        highdef[(HI_RES_MAP_DIMENSION / 2) + ((HI_RES_MAP_DIMENSION / 2) * HI_RES_MAP_DIMENSION)],
        (unsigned long)highres_tide[0],
        weather_seven_values(0, 0),
        weather_pressure(0, 0),
        grass,
        trees,
        bush,
        (unsigned long)((group != 0L) ? group->num : 0),
        (being != 0L),
        (being != 0L) ? being->constant.name[0] : 0,
        (being != 0L) ? being->constant.name[1] : 0,
        (being != 0L) ? being_energy(being) : 0,
        (being != 0L) ? being_location_x(being) : 0,
        (being != 0L) ? being_location_y(being) : 0,
        (being != 0L) ? being_facing(being) : 0,
        (being != 0L) ? being->delta.velocity[0] : 0,
        (being != 0L) ? being->delta.velocity[1] : 0,
        (being != 0L) ? being->delta.honor : 0,
        (being != 0L) ? being_height(being) : 0,
        (being != 0L) ? being->delta.mass : 0,
        (being != 0L) ? being_posture(being) : 0,
        (being != 0L) ? being->delta.awake : 0,
        (being != 0L) ? being->changes.drives[0] : 0,
        (being != 0L) ? being->changes.drives[1] : 0,
        (being != 0L) ? being->changes.drives[2] : 0,
        (being != 0L) ? being->changes.drives[3] : 0,
        (being != 0L) ? being->braindata.braincode_register[0] : 0,
        (being != 0L) ? being->braindata.braincode_register[1] : 0,
        (being != 0L) ? being->braindata.braincode_register[2] : 0,
        (being != 0L) ? being->braindata.brainprobe[0].type : 0,
        (being != 0L) ? being->braindata.brainprobe[0].position : 0,
        (being != 0L) ? being->braindata.brainprobe[0].address : 0,
        (being != 0L) ? being->braindata.brainprobe[0].frequency : 0,
        (being != 0L) ? being->braindata.brainprobe[0].offset : 0,
        (being != 0L) ? being->braindata.brainprobe[0].state : 0,
        (being != 0L) ? being->events.social[0].first_name[0] : 0,
        (being != 0L) ? being->events.social[0].family_name[0] : 0,
        (being != 0L) ? being->events.social[0].familiarity : 0,
        (being != 0L) ? being->events.social[0].friend_foe : 0,
        (being != 0L) ? being->events.social[0].attraction : 0,
        (being != 0L) ? being->events.social[0].relationship : 0,
        (being != 0L) ? being->events.episodic[0].event : 0,
        (being != 0L) ? being->events.episodic[0].food : 0,
        (being != 0L) ? being->events.episodic[0].affect : 0,
        (being != 0L) ? being->events.episodic[0].arg : 0,
        (being != 0L) ? being->events.territory[0].name : 0,
        (being != 0L) ? being->events.territory[0].familiarity : 0,
        (being != 0L) ? being->changes.date_of_conception : 0,
        (being != 0L) ? being->changes.father_name[0] : 0,
        (being != 0L) ? being->changes.father_name[1] : 0,
        (being != 0L) ? being->changes.mother_name[0] : 0,
        (being != 0L) ? being->changes.mother_name[1] : 0,
        (being != 0L) ? being->changes.child_generation_min : 0,
        (being != 0L) ? being->changes.child_generation_max : 0,
        (being != 0L) ? being->immune_system.antigens[0] : 0,
        (being != 0L) ? being->immune_system.shape_antigen[0] : 0,
        (being != 0L) ? being->immune_system.antibodies[0] : 0,
        (being != 0L) ? being->immune_system.shape_antibody[0] : 0,
        (being != 0L) ? being->immune_system.random_seed[0] : 0,
        (being != 0L) ? being->immune_system.random_seed[1] : 0,
        (being != 0L) ? being->changes.inventory[0] : 0,
        (being != 0L) ? being->changes.inventory[1] : 0,
        (being != 0L) ? being->changes.shout[0] : 0,
        (being != 0L) ? being->changes.learned_preference[0] : 0);
}

static void emit_selected_minute(const char *label, n_uint minute)
{
    simulated_group *group = sim_group();
    simulated_being *being = 0L;
    if ((group != 0L) && (group->num > 0))
    {
        being = group->select;
        if (being == 0L)
        {
            being = &(group->beings[0]);
        }
    }

    printf(
        "SELECTED-MINUTE label=%s minute=%lu date=%lu time=%lu population=%lu "
        "selected=%u energy=%ld loc=%ld:%ld facing=%u speed=%u:%u honor=%u "
        "mass=%ld awake=%u state=%u drives=%u:%u:%u:%u brain=%u:%u:%u "
        "social0=%u:%u episodic0=%u:%u immune=%u:%u:%u:%u preference=%u\n",
        label,
        (unsigned long)minute,
        (unsigned long)land_date(),
        (unsigned long)land_time(),
        (unsigned long)((group != 0L) ? group->num : 0),
        (being != 0L),
        (being != 0L) ? being_energy(being) : 0,
        (being != 0L) ? being_location_x(being) : 0,
        (being != 0L) ? being_location_y(being) : 0,
        (being != 0L) ? being_facing(being) : 0,
        (being != 0L) ? being->delta.velocity[0] : 0,
        (being != 0L) ? being->delta.velocity[1] : 0,
        (being != 0L) ? being->delta.honor : 0,
        (being != 0L) ? being->delta.mass : 0,
        (being != 0L) ? being->delta.awake : 0,
        (being != 0L) ? being->delta.macro_state : 0,
        (being != 0L) ? being->changes.drives[0] : 0,
        (being != 0L) ? being->changes.drives[1] : 0,
        (being != 0L) ? being->changes.drives[2] : 0,
        (being != 0L) ? being->changes.drives[3] : 0,
        (being != 0L) ? being->braindata.braincode_register[0] : 0,
        (being != 0L) ? being->braindata.braincode_register[1] : 0,
        (being != 0L) ? being->braindata.braincode_register[2] : 0,
        (being != 0L) ? being->events.social[0].familiarity : 0,
        (being != 0L) ? being->events.social[0].attraction : 0,
        (being != 0L) ? being->events.episodic[0].event : 0,
        (being != 0L) ? being->events.episodic[0].food : 0,
        (being != 0L) ? being->immune_system.antigens[0] : 0,
        (being != 0L) ? being->immune_system.shape_antigen[0] : 0,
        (being != 0L) ? being->immune_system.random_seed[0] : 0,
        (being != 0L) ? being->immune_system.random_seed[1] : 0,
        (being != 0L) ? being->changes.learned_preference[0] : 0);
}

int main(void)
{
    n_uint minute = 0;
    sim_init(KIND_START_UP, FIXED_RANDOM_SIM, MAP_AREA, 0);
    emit_snapshot("startup");
    emit_terrain_samples("startup");
    sim_cycle();
    emit_snapshot("after_cycle_1");
    emit_terrain_samples("after_cycle_1");
    while (minute < TIME_DAY_MINUTES)
    {
        sim_cycle();
        minute++;
        if (((minute % 60) == 0) || (minute > (TIME_DAY_MINUTES - 16)))
        {
            emit_selected_minute("after_cycle_1_to_after_day", minute);
        }
    }
    emit_snapshot("after_day");
    emit_terrain_samples("after_day");
    sim_close();
    return 0;
}
PROBE_C

cd "$ROOT"

sources=(
    toolkit/*.c
    script/*.c
    render/*.c
    sim/*.c
    entity/*.c
    universe/*.c
)

objects=()
for source in "${sources[@]}"; do
    source_path="./$source"
    object_path="$BUILD_DIR/${source//\//_}.o"
    "$COMPILER" "${CFLAGS_ARGS[@]}" "${DEFINES_ARGS[@]}" -c "$source_path" -o "$object_path" -w
    objects+=("$object_path")
done

"$COMPILER" "${CFLAGS_ARGS[@]}" "${DEFINES_ARGS[@]}" -I"$ROOT" -c "$PROBE" -o "$BUILD_DIR/c_engine_trace_probe.o" -w
objects+=("$BUILD_DIR/c_engine_trace_probe.o")

"$COMPILER" "${CFLAGS_ARGS[@]}" "${DEFINES_ARGS[@]}" -I"$ROOT" -o "$OUT_DIR/c_engine_trace_probe" "${objects[@]}" -lz -lm -lpthread
"$OUT_DIR/c_engine_trace_probe" > "$OUT_DIR/c_engine_trace.trace"

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'compiler=%s\n' "$COMPILER"
    "$COMPILER" --version | head -n 1
    printf 'trace=%s\n' "$OUT_DIR/c_engine_trace.trace"
    shasum -a 256 "$OUT_DIR/c_engine_trace.trace"
} > "$OUT_DIR/c_engine_trace_manifest.txt"

echo "c-engine-trace-probe=$OUT_DIR"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
C_ROOT="$ROOT/c"
OUT_DIR="${1:-"$ROOT/target/native-raw-binary-oracle"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"
COMPILER="${CC:-gcc}"
CFLAGS_TEXT="${CFLAGS:--O2}"
DEFINES_TEXT="${DEFINES:--DCOMMAND_LINE_EXPLICIT -DAUTOMATED}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

BUILD_DIR="$OUT_DIR/objects"
ARTIFACT_DIR="$OUT_DIR/artifacts"
mkdir -p "$BUILD_DIR" "$ARTIFACT_DIR"

HARNESS="$OUT_DIR/native_raw_oracle.c"
cat > "$HARNESS" <<'C'
#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <stdlib.h>

#include "toolkit/toolkit.h"
#include "entity/entity.h"
#include "sim/sim.h"
#include "universe/universe.h"

n_int draw_error( n_constant_string error_text, n_constant_string location, n_int line_number )
{
    printf( "ERROR: %s @ %s %ld\n", ( const n_string ) error_text, location, line_number );
    return -1;
}

static void write_n_file( const char *path, n_file *file )
{
    FILE *out;
    if ( file == 0L || file->data == 0L )
    {
        fprintf( stderr, "no transfer data for %s\n", path );
        exit( 2 );
    }
    out = fopen( path, "wb" );
    if ( out == 0L )
    {
        perror( path );
        exit( 3 );
    }
    fwrite( file->data, 1, file->location, out );
    fclose( out );
    io_file_free( &file );
}

static void write_summary( FILE *summary, const char *scenario )
{
    simulated_group *group = sim_group();
    simulated_being *being = group->num > 0 ? &( group->beings[0] ) : 0L;

    unsigned loc0 = 0, loc1 = 0, facing = 0, speed = 0, energy = 0, state = 0;
    unsigned brain0 = 0, brain1 = 0, brain2 = 0, brain3 = 0, brain4 = 0, brain5 = 0;
    unsigned territory0 = 0;
    unsigned antigen0 = 0, shape_antigen0 = 0, antibody0 = 0, shape_antibody0 = 0;
    unsigned inventory0 = 0, inventory1 = 0, shout0 = 0;

    if ( being != 0L )
    {
        loc0 = ( unsigned )being->delta.location[0];
        loc1 = ( unsigned )being->delta.location[1];
        facing = ( unsigned )being->delta.direction_facing;
        speed = ( unsigned )being->delta.velocity[0];
        energy = ( unsigned )being->delta.stored_energy;
        state = ( unsigned )being->delta.macro_state;
        brain0 = ( unsigned )being->braindata.brain_state[0];
        brain1 = ( unsigned )being->braindata.brain_state[1];
        brain2 = ( unsigned )being->braindata.brain_state[2];
        brain3 = ( unsigned )being->braindata.brain_state[3];
        brain4 = ( unsigned )being->braindata.brain_state[4];
        brain5 = ( unsigned )being->braindata.brain_state[5];
        territory0 = ( unsigned )being->events.territory[0].familiarity;
        antigen0 = ( unsigned )being->immune_system.antigens[0];
        shape_antigen0 = ( unsigned )being->immune_system.shape_antigen[0];
        antibody0 = ( unsigned )being->immune_system.antibodies[0];
        shape_antibody0 = ( unsigned )being->immune_system.shape_antibody[0];
        inventory0 = ( unsigned )being->changes.inventory[0];
        inventory1 = ( unsigned )being->changes.inventory[1];
        shout0 = ( unsigned )being->changes.shout[0];
    }

    fprintf( summary,
             "RAW scenario=%s population=%u first_loc=%u:%u first_facing=%u first_speed=%u first_energy=%u first_state=%u first_brain=%u:%u:%u:%u:%u:%u first_territory0=%u first_immune=%u:%u:%u:%u first_inventory=%u:%u first_shout=%u\n",
             scenario,
             ( unsigned )group->num,
             loc0,
             loc1,
             facing,
             speed,
             energy,
             state,
             brain0,
             brain1,
             brain2,
             brain3,
             brain4,
             brain5,
             territory0,
             antigen0,
             shape_antigen0,
             antibody0,
             shape_antibody0,
             inventory0,
             inventory1,
             shout0 );
}

int main( int argc, char **argv )
{
    char path[4096];
    char summary_path[4096];
    const char *out_dir = argc > 1 ? argv[1] : ".";
    n_uint seed = 0x5261f726;
    n_int cycle = 0;
    FILE *summary;

    snprintf( summary_path, sizeof( summary_path ), "%s/native_raw_binary_values.trace", out_dir );
    summary = fopen( summary_path, "w" );
    if ( summary == 0L )
    {
        perror( summary_path );
        exit( 4 );
    }

    snprintf( path, sizeof( path ), "%s/raw_empty_startup.native", out_dir );
    sim_init( KIND_START_UP, seed, MAP_AREA, 0 );
    write_summary( summary, "raw_empty_startup" );
    write_n_file( path, tranfer_out() );

    snprintf( path, sizeof( path ), "%s/raw_reset_startup.native", out_dir );
    sim_init( KIND_NEW_SIMULATION, seed, MAP_AREA, 0 );
    write_summary( summary, "raw_reset_startup" );
    write_n_file( path, tranfer_out() );

    snprintf( path, sizeof( path ), "%s/raw_after_one_cycle.native", out_dir );
    sim_cycle();
    write_summary( summary, "raw_after_one_cycle" );
    write_n_file( path, tranfer_out() );

    snprintf( path, sizeof( path ), "%s/raw_social_heavy.native", out_dir );
    for ( cycle = 0; cycle < 120; cycle++ )
    {
        sim_cycle();
    }
    write_summary( summary, "raw_social_heavy" );
    write_n_file( path, tranfer_out() );

    snprintf( path, sizeof( path ), "%s/raw_immune_heavy.native", out_dir );
    if ( sim_group()->num > 1 )
    {
        simulated_group *group = sim_group();
        n_int transmission = 0;
        for ( cycle = 0; cycle < 96; cycle++ )
        {
            transmission = cycle % PATHOGEN_TRANSMISSION_TOTAL;
            immune_ingest_pathogen( &( group->beings[0].immune_system ), transmission % 4 );
            immune_transmit( &( group->beings[0].immune_system ),
                             &( group->beings[1].immune_system ),
                             ( n_byte )transmission );
            ( void )immune_response( &( group->beings[0].immune_system ),
                                     being_honor_immune( &( group->beings[0] ) ),
                                     being_energy( &( group->beings[0] ) ) );
            sim_cycle();
        }
    }
    write_summary( summary, "raw_immune_heavy" );
    write_n_file( path, tranfer_out() );

    snprintf( path, sizeof( path ), "%s/raw_terrain_heavy.native", out_dir );
    for ( cycle = 0; cycle < 1440; cycle++ )
    {
        sim_cycle();
    }
    write_summary( summary, "raw_terrain_heavy" );
    write_n_file( path, tranfer_out() );

    snprintf( path, sizeof( path ), "%s/raw_save_open_derived.native", out_dir );
    {
        n_file *saved = tranfer_out();
        n_file *load_copy = io_file_duplicate( saved );
        if ( ( load_copy == 0L ) || ( tranfer_in( load_copy ) != 0 ) )
        {
            fprintf( stderr, "failed to reload save/open-derived raw fixture\n" );
            exit( 5 );
        }
        if ( sim_init( KIND_LOAD_FILE, 0, MAP_AREA, 0 ) == 0L )
        {
            fprintf( stderr, "failed to initialize save/open-derived fixture\n" );
            exit( 6 );
        }
        io_file_free( &load_copy );
        io_file_free( &saved );
    }
    sim_cycle();
    write_summary( summary, "raw_save_open_derived" );
    write_n_file( path, tranfer_out() );

    fclose( summary );

    return 0;
}
C

read -r -a CFLAGS_ARGS <<< "$CFLAGS_TEXT"
read -r -a DEFINES_ARGS <<< "$DEFINES_TEXT"
DEFINES_ARGS+=("-DFULL_DATE=\"$FULL_DATE\"")

cd "$C_ROOT"

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
    object_path="$BUILD_DIR/${source//\//_}.o"
    "$COMPILER" "${CFLAGS_ARGS[@]}" "${DEFINES_ARGS[@]}" -c "./$source" -o "$object_path" -w
    objects+=("$object_path")
done

"$COMPILER" "${CFLAGS_ARGS[@]}" "${DEFINES_ARGS[@]}" -I"$C_ROOT" -c "$HARNESS" -o "$BUILD_DIR/native_raw_oracle.o" -w
objects+=("$BUILD_DIR/native_raw_oracle.o")
"$COMPILER" "${CFLAGS_ARGS[@]}" "${DEFINES_ARGS[@]}" -I/usr/include -o "$OUT_DIR/native_raw_oracle" "${objects[@]}" -lz -lm -lpthread

"$OUT_DIR/native_raw_oracle" "$ARTIFACT_DIR"

HASH_FILE="$OUT_DIR/native_raw_binary_hashes.txt"
MAP_FILE="$OUT_DIR/native_raw_binary_byte_map.txt"
rm -f "$HASH_FILE" "$MAP_FILE"

{
    printf '# artifact | size | sha256\n'
    for artifact in "$ARTIFACT_DIR"/*.native; do
        [ -f "$artifact" ] || continue
        size="$(wc -c < "$artifact" | tr -d ' ')"
        hash="$(shasum -a 256 "$artifact" | awk '{print $1}')"
        printf '%s | %s | %s\n' "$(basename "$artifact")" "$size" "$hash"
    done
} > "$HASH_FILE"

{
    printf '# artifact | offset | length | field | note\n'
    for artifact in "$ARTIFACT_DIR"/*.native; do
        [ -f "$artifact" ] || continue
        name="$(basename "$artifact")"
        size="$(wc -c < "$artifact" | tr -d ' ')"
        printf '%s | 0 | %s | native-transfer-text | io_write_buff byte stream generated by tranfer_out()\n' "$name" "$size"
        while IFS=: read -r offset token; do
            printf '%s | %s | 6 | token:%s | native simulated_file_format section/field marker\n' "$name" "$offset" "$token"
        done < <(LC_ALL=C grep -aboE 'simul\{|signa=|verio=|landd\{|dated=|timed=|landg=|topog\{|topby=|weath\{|atmby=|litby=|being\{|locat=|facin=|speed=|energ=|datob=|rando=|state=|brast=|heigt=|masss=|overr=|shout=|crowd=|postu=|inven=|paras=|honor=|conce=|atten=|genet=|fetag=|fathn=|sosim=|drive=|goals=|prefe=|genex=|genen=|chigx=|chign=|awako=|bname=|terit=|immun=|brreg=|brpro=' "$artifact" || true)
    done
} > "$MAP_FILE"

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'compiler=%s\n' "$COMPILER"
    printf 'cflags=%s\n' "$CFLAGS_TEXT"
    printf 'defines=%s\n' "$DEFINES_TEXT"
    printf 'harness=%s\n' "$HARNESS"
    printf 'binary=%s\n' "$OUT_DIR/native_raw_oracle"
    printf 'artifacts=%s\n' "$ARTIFACT_DIR"
    printf 'values=%s\n' "$ARTIFACT_DIR/native_raw_binary_values.trace"
    printf 'hashes=%s\n' "$HASH_FILE"
    printf 'byte_map=%s\n' "$MAP_FILE"
} > "$OUT_DIR/native_raw_binary_oracle_manifest.txt"

echo "native-raw-binary-oracle=$OUT_DIR artifacts=$ARTIFACT_DIR"

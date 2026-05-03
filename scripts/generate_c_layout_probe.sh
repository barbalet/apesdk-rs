#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/c-layout-probe"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"
COMPILER="${CC:-gcc}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"

PROBE="$OUT_DIR/layout_probe.c"
cat > "$PROBE" <<'PROBE_C'
#include <stddef.h>
#include <stdio.h>

#include "toolkit/toolkit.h"
#include "sim/sim.h"
#include "universe/universe.h"

#define SIZEOF(type_name) \
    printf("sizeof(%s) %lu\n", #type_name, (unsigned long)sizeof(type_name))

#define OFFSET(type_name, field_name) \
    printf("offsetof(%s.%s) %lu\n", #type_name, #field_name, (unsigned long)offsetof(type_name, field_name))

int main(void)
{
    SIZEOF(n_byte);
    SIZEOF(n_byte2);
    SIZEOF(n_byte4);
    SIZEOF(n_uint);
    SIZEOF(n_int);
    SIZEOF(n_genetics);
#ifdef IMMUNE_ON
    SIZEOF(simulated_immune_system);
#endif
    SIZEOF(simulated_being_delta);
    SIZEOF(simulated_being_constant);
    SIZEOF(simulated_being_events);
    SIZEOF(simulated_being_brain);
    SIZEOF(simulated_being_volatile);
    SIZEOF(simulated_being);
    SIZEOF(simulated_group);

    OFFSET(simulated_being, delta);
    OFFSET(simulated_being, constant);
    OFFSET(simulated_being, events);
    OFFSET(simulated_being, braindata);
    OFFSET(simulated_being, changes);
#ifdef IMMUNE_ON
    OFFSET(simulated_being, immune_system);
#endif

    OFFSET(simulated_being_delta, location);
    OFFSET(simulated_being_delta, direction_facing);
    OFFSET(simulated_being_delta, velocity);
    OFFSET(simulated_being_delta, stored_energy);
    OFFSET(simulated_being_delta, random_seed);
    OFFSET(simulated_being_delta, macro_state);
    OFFSET(simulated_being_delta, parasites);
    OFFSET(simulated_being_delta, honor);
    OFFSET(simulated_being_delta, crowding);
    OFFSET(simulated_being_delta, height);
    OFFSET(simulated_being_delta, mass);
    OFFSET(simulated_being_delta, posture);
    OFFSET(simulated_being_delta, goal);
    OFFSET(simulated_being_delta, social_coord_x);
    OFFSET(simulated_being_delta, social_coord_y);
    OFFSET(simulated_being_delta, awake);

    OFFSET(simulated_being_constant, date_of_birth);
    OFFSET(simulated_being_constant, generation_min);
    OFFSET(simulated_being_constant, generation_max);
    OFFSET(simulated_being_constant, name);
    OFFSET(simulated_being_constant, genetics);

    OFFSET(simulated_being_events, social);
    OFFSET(simulated_being_events, episodic);
#ifdef TERRITORY_ON
    OFFSET(simulated_being_events, territory);
#endif

#ifdef BRAINCODE_ON
    OFFSET(simulated_being_brain, braincode_register);
    OFFSET(simulated_being_brain, brainprobe);
#endif
#ifdef BRAIN_ON
    OFFSET(simulated_being_brain, brain);
#endif
    OFFSET(simulated_being_brain, brain_state);
    OFFSET(simulated_being_brain, script_overrides);
    OFFSET(simulated_being_brain, attention);

    OFFSET(simulated_being_volatile, drives);
    OFFSET(simulated_being_volatile, shout);
    OFFSET(simulated_being_volatile, inventory);
    OFFSET(simulated_being_volatile, learned_preference);
    OFFSET(simulated_being_volatile, date_of_conception);
    OFFSET(simulated_being_volatile, fetal_genetics);
    OFFSET(simulated_being_volatile, father_name);
    OFFSET(simulated_being_volatile, mother_name);
    OFFSET(simulated_being_volatile, child_generation_min);
    OFFSET(simulated_being_volatile, child_generation_max);

    OFFSET(simulated_group, beings);
    OFFSET(simulated_group, remains);
    OFFSET(simulated_group, select);
    OFFSET(simulated_group, num);
    OFFSET(simulated_group, max);

    return 0;
}
PROBE_C

"$COMPILER" -I"$ROOT" -DAUTOMATED -DCOMMAND_LINE_EXPLICIT -DFULL_DATE="\"$FULL_DATE\"" "$PROBE" -o "$OUT_DIR/layout_probe"
"$OUT_DIR/layout_probe" > "$OUT_DIR/layout_probe.txt"

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'compiler=%s\n' "$COMPILER"
    "$COMPILER" --version | head -n 1
    printf 'layout_probe=%s\n' "$OUT_DIR/layout_probe.txt"
    shasum -a 256 "$OUT_DIR/layout_probe.txt"
} > "$OUT_DIR/layout_probe_manifest.txt"

echo "c-layout-probe=$OUT_DIR"

# Terrain And Food Audit

## C Sources

Movement and eating pull terrain data from:

```text
sim/land.c
sim/sim.h
entity/food.c
entity/being.c
universe/loop.c
universe/universe.h
```

## Ported Coverage

Rust now carries C-shaped helpers for:

```text
APESPACE_TO_MAPSPACE
WATER_TEST
land_tide
land_vect2 height and slope sampling
land_operator and land_operator_interpolated biology operators
food_values
food_eat_land
food_intertidal
food_absorption
immune_ingest_pathogen
being_temporary_speed
being_move_energy terrain and swimming cost
```

The Rust terrain source is still deterministic procedural topography generated
from the saved land genetics. It does not yet load the full C high-definition
tile maps, but the operator stack, water tests, tide range, slope shape, and
food dominance rules now follow the native formulas used by command-line
movement and eating.

## Remaining Gaps

The next terrain/food pass should replace or augment the deterministic Rust
topography with native tile-map parity, then compare biology operator values
against C fixtures at fixed map locations. Food depletion/regrowth and deeper
animal foods are still outside this cycle range.

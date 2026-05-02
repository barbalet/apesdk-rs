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

The Rust terrain source is now stored in native-shaped tile buffers generated
from saved tile genetics. The exact C `tile_patch`/`tile_round` generator is
still pending fixture parity, but height, slope, operator, water, tide, and
food dominance reads now consume tile-backed map bytes. High-definition
topography is exposed through a lazy C-shaped bilinear accessor.

## Remaining Gaps

The next terrain/food pass should replace the deterministic Rust tile fill with
exact native `tile_patch`/`tile_round` generation, then compare map bytes,
high-definition samples, biology operator values, and repeated eating
transcripts against C fixtures. Deeper animal foods remain outside this cycle
range.

## Cycle 116-120 Update

Rust now also has repeated-eating depletion tests, long regrowth tests,
inventory food hooks for fish, cracked nuts, grass, bird eggs, and lizard eggs,
and a checked `TerrainFoodFixtureSample` probe surface. The remaining terrain
drift is still exact C tile generation, not the food decision/eating harness.

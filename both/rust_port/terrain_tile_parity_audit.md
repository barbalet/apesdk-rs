# Terrain Tile Parity Audit

## C Surface

Native command-line terrain is owned by `n_land` in `sim/sim.h`:

```text
n_tile tiles[1]
n_byte2 genetics[2]
n_byte4 date
n_byte2 time
n_byte tide_level
n_byte topography_highdef[HI_RES_MAP_AREA * 2]
n_byte4 highres_tide[HI_RES_MAP_AREA / 32]
```

The non-planet command-line build uses `MAP_BITS = 9`, `MAP_TILES = 1`, and
one `n_tile` with two `topography[MAP_AREA]` buffers. Tile genetics are
preserved by `tile_land_erase`; planet genetics are cleared on land reset.

Relevant C paths:

```text
sim/tile.c: tiles_topography_map, tiles_topography, tiles_set_topography
sim/tile.c: tile_pack_topography, tile_patch, tile_round, tile_land_init
sim/tile.c: tile_land_random, tile_land_erase
sim/land.c: math_bilinear_8_times, land_operator, weather_seven_values
sim/land.c: land_clear, land_seed_genetics, land_init_high_def
entity/food.c: food_values, food_eat_land, food_intertidal, food_eat
```

## Rust Coverage Added In Cycles 106-115

Rust `LandState` now owns a C-shaped `LandTile` array. Each tile carries its
native genetics and two `MAP_AREA` topography buffers. `height_at`,
`terrain_sample`, land biology operators, and food selection read the primary
tile buffer instead of recomputing terrain on demand.

High-definition topography is exposed through a lazy accessor matching the C
8x bilinear interpolation shape. The high-res tide mask is currently virtual:
`highres_tide_at` evaluates the same fixed intertidal topography band that C
uses while avoiding a 16-million-cell eager buffer in normal tests.

Weather-seven bands now follow the native day, night, and dawn/dusk time
classification. Pressure is still a Rust deterministic approximation until the
native atmosphere buffers and fixture dumps are ported.

Food availability now has per-map-cell quantity state. Eating can deplete the
cell, food energy is gated by the remaining quantity, and land time regrows
availability in fixed intervals.

## Remaining C-Parity Work

The remaining byte-level terrain work is the exact `tile_patch`/`tile_round`
generator and a fixture runner that dumps C map bytes, high-definition samples,
biology operator values, and repeated food/eating transcripts for the same
seeds. Binary save/load cycles will also need to decide whether lazy high-res
state remains acceptable internally or whether native save compatibility
requires materializing the generated arrays.

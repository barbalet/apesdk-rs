# Rust Port Cycle 119: Terrain/Food C Fixture Harness

## Objective

Create a stable fixture surface for terrain and food comparisons.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_119.md
```

## Implemented

Added `TerrainFoodFixtureSample` and `terrain_food_fixture_sample` to collect
height, food biology operators, dominant food type, and maximum energy at fixed
ape-space locations.

## Validation

Added checked fixture samples for seed `[7633, 53305]` and time `400`. These
are currently Rust-side fixtures; exact C byte fixtures remain the next terrain
generator task.

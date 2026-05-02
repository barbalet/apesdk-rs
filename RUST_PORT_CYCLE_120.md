# Rust Port Cycle 120: Terrain/Food Parity Freeze

## Objective

Freeze the current terrain/food behavior before moving into save/load work.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
rust_port/terrain_food_audit.md
RUST_PORT_CYCLE_120.md
```

## Implemented

The terrain/food surface now has tile-backed heights, local food quantity,
regrowth, inventory foods, and fixed probe fixtures. Remaining drift is
documented as exact C tile generation and C fixture comparison.

## Validation

The fixture probe, repeated eating, inventory food, and existing movement/food
tests pass together.

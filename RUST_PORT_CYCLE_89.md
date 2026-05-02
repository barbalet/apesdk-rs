# Rust Port Cycle 89: Temporary Speed From Terrain

## Objective

Replace reduced random target speed with terrain-derived temporary speed.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_89.md
```

## Implemented

`BeingSummary::temporary_speed` now uses the current terrain slope, facing
vector, and look-ahead water test. Swimming and water-ahead cases turn toward
higher ground and apply the swim gene speed adjustment.

## Validation

Covered by awake-cycle and movement-energy tests over seeded terrain.

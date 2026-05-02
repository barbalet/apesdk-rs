# Rust Port Cycle 88: Tide And Water State Parity

## Objective

Port tide and water checks used by awake movement.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_88.md
```

## Implemented

Added C constants for `WATER_MAP`, tide amplitudes, `TIDE_MAX`, lunar orbit
minutes, and the sine divisor. Land minute advancement now refreshes the tide
level, and terrain samples expose water and intertidal status.

## Validation

Added a tide-range test around a fixed cycle transition.

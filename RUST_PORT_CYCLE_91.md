# Rust Port Cycle 91: Terrain-Aware Awake Movement

## Objective

Wire terrain speed, swimming, and movement cost into the awake cycle.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_91.md
```

## Implemented

Population minute advancement now passes `LandState` into each being. Awake
movement uses terrain samples, water state bits, swimming inventory effects,
genetic wandering, and terrain movement cost while preserving save/load state.

## Validation

Existing populated `advance_minutes` coverage continues to pass with terrain
movement enabled.

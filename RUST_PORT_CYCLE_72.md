# Rust Port Cycle 72: Food And Eating Surface

## Objective

Port the minimum food/eating surface needed by awake cycling.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_72.md
```

## Implemented

Added food constants and a reduced `food_energy` path for vegetable/grass food.
The calculation uses the same absorption shape as C food intake: gene-derived
food absorption, a denominator across food preferences, max food energy, and
the 320 energy cap.

Hungry stopped beings now eat, reset hunger drive, set the eating state, and
grow if juvenile.

## Validation

Added awake-cycle tests for eating state, hunger reset path, energy gain, and
juvenile growth.


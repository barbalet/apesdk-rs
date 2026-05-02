# Rust Port Cycle 69: Awake Movement Loop

## Objective

Replace placeholder movement with a native-shaped awake movement step.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_69.md
```

## Implemented

`advance_minute` now delegates awake behavior to `cycle_awake`. The new awake
cycle coordinates:

```text
temporary speed
C-style speed convergence
facing wander
vector movement
Ape-space wrapping
movement energy
moving/hungry/eating/no-food state bits
```

The cycle also advances velocity history after drive cycling.

## Validation

Existing populated `step` and `run` tests now execute through the awake movement
loop, and new tests cover movement-energy and speed-history behavior.


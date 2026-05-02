# Rust Port Cycle 93: Land Food Eating Parity

## Objective

Port land food selection among grass, bush, and fruit.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_93.md
```

## Implemented

Added land food dominance rules with C energy caps for grass, bush, and fruit.
Awake eating now records the selected food type in episodic memory.

## Validation

The awake-cycle eating test now selects an edible seeded land location before
checking energy, growth, mass, and eating state.

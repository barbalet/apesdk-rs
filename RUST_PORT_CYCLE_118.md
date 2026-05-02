# Rust Port Cycle 118: Animal And Egg Food Hooks

## Objective

Connect native inventory food types where command-line runtime behavior can
reach them.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_118.md
```

## Implemented

Added native inventory constants and eating hooks for grass, fish, cracked
nuts, bird eggs, and lizard eggs. Hungry awake cycles consume held edible
inventory before falling back to terrain food.

## Validation

Added inventory food tests covering food type, maximum energy, absorption, and
inventory removal after eating.

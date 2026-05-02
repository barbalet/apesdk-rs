# Rust Port Cycle 135: Braincode Arithmetic Operators

## Objective

Port initial braincode data, arithmetic, comparison, and control operators.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_135.md
```

## Implemented

The VM now executes native-shaped `MOV`, `ADD`, `SUB`, `MUL`, `DIV`, `MOD`,
`CTR`, `SWP`, `INV`, `STP`, `LTP`, jump, and skip/conditional operators.
Sensor and actuator opcodes remain decoded no-ops for the next tranche.

## Validation

Added single-step VM tests for arithmetic, register load/store, conditional
skips, counters, and byte mutation behavior.

# Rust Port Cycle 83: Binary Save/Load Parity Investigation

## Objective

Decide whether binary transfer parity should be implemented in this tranche.

## Changed Files

```text
rust_port/binary_save_load_decision.md
crates/simape/src/lib.rs
RUST_PORT_CYCLE_83.md
```

## Implemented

Documented the C binary transfer path and kept Rust save/load JSON-only through
cycle 85. Added a CLI regression test showing binary/native-looking non-JSON
data is rejected through the current load error path.

## Validation

Binary rejection test plus full `cargo test`.


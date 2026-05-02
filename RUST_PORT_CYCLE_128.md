# Rust Port Cycle 128: Binary Writer Skeleton

## Objective

Add a Rust writer for the native transfer section format.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_128.md
```

## Implemented

Added `tranfer_startup_out_native` and `SimState::tranfer_startup_out_native`.
The writer emits `simul`, `landd`, `being`, `sgcia`, and `episo` sections with
native six-character field tokens and numeric array syntax.

## Validation

Writer tests verify section structure and parse the emitted native transfer
back through the Rust native reader.

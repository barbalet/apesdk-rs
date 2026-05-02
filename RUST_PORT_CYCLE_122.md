# Rust Port Cycle 122: Binary Reader Skeleton

## Objective

Add a native reader skeleton without disturbing JSON loading.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
crates/simape/src/lib.rs
RUST_PORT_CYCLE_122.md
```

## Implemented

Added `startup_transfer_from_native_bytes`, `native_transfer_sections`, and
`SimState::load_startup_bytes`. CLI `open`/`load` now try JSON first and the
native transfer format second.

## Validation

Added parser tests for comments, whitespace, section tokens, and raw binary
rejection.

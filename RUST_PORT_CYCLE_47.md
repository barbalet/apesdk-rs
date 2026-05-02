# Rust Port Cycle 47: Native Being Projection

## Objective

Bridge the Rust-owned summary state and the C-compatible `simulated_being`
structs.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_47.md
```

## Implemented

Added projection APIs:

```text
BeingSummary::to_simulated_being
BeingSummary::from_simulated_being
```

The mapping covers constant, delta, changes, braindata, and immune fields that
are currently represented in the summary layer.

## Compatibility Notes

This is a structural bridge, not the full C engine yet. It gives future cycles
a tested way to replace summary internals with native `simulated_being` state.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
93 tests passed
```

## Next Cycle

Cycle 48 should accept C-shaped native being JSON on load.

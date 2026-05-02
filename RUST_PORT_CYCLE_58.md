# Rust Port Cycle 58: Energy Accessors And Saturating Semantics

## Objective

Port the C energy accessor behavior into the Rust being state.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_58.md
```

## Implemented

Added:

```text
BeingSummary::energy_less_than
BeingSummary::energy_delta
```

`energy_delta` clamps at `BEING_DEAD`, matching the important C safety
behavior from `being_energy_delta`, and keeps positive values inside the Rust
`u16` storage range.

## Compatibility Notes

The C code only lower-clamps and then stores into `n_byte2`. Rust clamps the
upper bound as well to avoid wraparound during port scaffolding.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
99 tests passed
```

## Next Cycle

Cycle 59 should port the awake-state calculation.

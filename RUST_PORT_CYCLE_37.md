# Rust Port Cycle 37: Population Age Helpers

## Objective

Expose population age accounting so command output can reason about adults and
juveniles from the Rust state model.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_37.md
```

## Implemented

Added `SimState` helpers:

```text
age_days
adult_count
juvenile_count
```

The helpers use the land date and each being's date of birth. Tests cover the
startup state, reset state, and maturity-date transition.

## Compatibility Notes

The CLI keeps the C-like reset smoke output of 128 adults at date zero for now,
but the state layer has the stricter age-based helpers ready for the real
simulation clock.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
89 tests passed
```

## Next Cycle

Cycle 38 should use the richer summaries to produce populated `stats`,
`appearance`, and `genome` command output.

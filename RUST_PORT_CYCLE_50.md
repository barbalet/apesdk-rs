# Rust Port Cycle 50: Summary Minute Cycling

## Objective

Make populated apes change over time in Rust instead of remaining static after
reset.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_50.md
```

## Implemented

`BeingSummary` now has a deterministic minute advance that updates:

```text
random seed
facing
location
speed
energy
awake state
macro state
drives
honor at day rollover
```

Movement wraps inside Ape-space using the C-sized coordinate bounds.

## Compatibility Notes

This is not yet the full C behavioral loop. It is a tested live-state scaffold
that can be replaced piece by piece by native engine logic.

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

Cycle 51 should expose populated minute advancement from `SimState`.

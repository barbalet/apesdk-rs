# Rust Port Cycle 53: Populated Run Command

## Objective

Remove the populated bounded `run` unimplemented error.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_53.md
```

## Implemented

Bounded `run` commands now advance populated simulations for the parsed number
of minutes, hours, days, months, or years.

The existing `run forever` guard remains in place.

## Compatibility Notes

The command still prints the C-style `Running for ...` line before advancing.

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

Cycle 54 should verify populated native save/open after live stepping.

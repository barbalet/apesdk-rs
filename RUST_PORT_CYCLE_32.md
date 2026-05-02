# Rust Port Cycle 32: Population Summary State

## Objective

Add a compact Rust population layer that can be used safely by the command-line
port while the full `simulated_being` engine remains under construction.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_32.md
```

## Implemented

Rust-owned population types:

```text
BeingSummary
PopulationState
```

`BeingSummary` stores:

```text
name
gender name
family name
date of birth
generation min/max
genetics[4]
```

`PopulationState` stores the current beings, maximum population, and selected
index.

## Compatibility Notes

Initial genetics follow the C `being_random_genetics` shape, including setting
the Y chromosome sex bits to `2` for male and `3` for female.

Names are deterministic Rust placeholders:

```text
Ape 001
Ape 002
...
```

The full C name-table algorithm remains future work.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
83 tests passed
```

## Next Cycle

Cycle 33 should expose selection and name lookup behavior to the CLI.

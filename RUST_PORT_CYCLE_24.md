# Rust Port Cycle 24: Empty Stats and Appearance Commands

## Objective

Port empty-population error behavior for being stats and appearance command
families.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_24.md
```

## Implemented

Command-line behavior:

```text
stats
status
appearance
physical
```

No-argument commands report:

```text
ERROR: No being was specified @ ./universe/command.c 1311
```

Named missing beings report:

```text
ERROR: Being not found @ ./universe/command.c 1300
```

## Compatibility Notes

The aliases now reach C-shaped empty-state behavior instead of the generic Rust
unsupported fallback. Rendering actual stats and appearance fields still depends
on porting being state.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
72 tests passed
```

## Next Cycle

Cycle 25 should port the empty genome/genetics failure paths and the quiet empty
`idea` command.

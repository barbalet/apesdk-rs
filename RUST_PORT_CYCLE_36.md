# Rust Port Cycle 36: Extended Being Summary Transfer

## Objective

Carry more of the native being state through Rust save/open so populated command
output can be generated from stored data rather than placeholders.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_36.md
```

## Implemented

`BeingSummary` now stores and transfers:

```text
location
facing
energy
speed
honor
height
mass
awake
```

The loader keeps backward compatibility with the earlier summary JSON by using
C-like defaults when these fields are absent.

## Compatibility Notes

The fields intentionally mirror values from the C `simulated_being_delta`
surface, while remaining in the summary model until the full native transfer
format is ported.

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

Cycle 37 should add age and demographic helpers over the populated Rust state.

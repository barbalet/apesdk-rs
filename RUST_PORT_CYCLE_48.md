# Rust Port Cycle 48: Native Being JSON Loader

## Objective

Allow Rust `open`/`load` to accept native C-shaped being JSON in addition to
the earlier summary format.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_48.md
```

## Implemented

`BeingSummary::from_transfer_object` now detects nested `delta` and `constant`
objects and parses native fields including:

```text
location
direction_facing
velocity
stored_energy
random_seed
honor
height
mass
date_of_birth
genetics
generation_range
```

Earlier summary JSON is still accepted.

## Compatibility Notes

C JSON does not currently include every field needed by the Rust summary layer,
so missing native extension objects fall back to deterministic defaults.

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

Cycle 49 should add Ape-space motion constants needed for live cycling.

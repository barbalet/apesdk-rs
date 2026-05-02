# Rust Port Cycle 39: Watch Selection

## Objective

Make `watch` useful against the reset population by selecting a named ape and
reporting watch-state commands.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_39.md
```

## Implemented

`watch` now supports:

```text
watch Ape 010
watch off
watch state
```

Selecting a valid name updates the current ape used by `ape`, detail commands,
and navigation.

## Compatibility Notes

Empty-population errors keep the earlier C-compatible wording. Unknown names in
a populated simulation report `Being not found`.

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

Cycle 40 should make `top` rank the current populated summaries.

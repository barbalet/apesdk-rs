# Rust Port Cycle 40: Populated Top Ranking

## Objective

Replace the populated `top` placeholder with a deterministic honor ranking over
the Rust population summaries.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_40.md
```

## Implemented

`top` now always prints the C-style header and, when apes are present, prints
the top ten beings by honor with:

```text
honor
name
sex
age in days
```

## Compatibility Notes

The empty-population output remains unchanged from the previous C-shaped smoke
test.

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

Cycle 41 should add populated social and pathogen summaries for the selected
ape.

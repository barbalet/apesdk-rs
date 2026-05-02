# Rust Port Cycle 38: Populated Stats and Genome Output

## Objective

Replace populated detail-command placeholders with useful summary output for
the selected ape.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_38.md
```

## Implemented

The Rust CLI now formats populated output for:

```text
stats
status
appearance
physical
genome
genetics
```

The genome formatter emits two fixed-width chromosome rows, and the stats
formatter reports location, direction, drives, sex, energy, speed, honor,
height, and age.

## Compatibility Notes

This is still summary-level output, not the complete C per-being engine dump.
It gives the command line a stable populated behavior while the deeper engine
port continues.

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

Cycle 39 should improve `watch` so named apes can be selected from the command
line.

# Rust Port Cycle 80: Episodic Output Parity

## Objective

Replace populated episodic placeholder output with data-backed rows.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_80.md
```

## Implemented

The `episodic` command now prints native episodic memories for the selected
being. Output covers eating, grooming, being groomed, chat, hit/hit-by, and
mate-seeking rows, including elapsed-time text and affect deltas.

## Validation

Added a CLI fixture test that loads episodic memory and verifies data-backed
memory rows.


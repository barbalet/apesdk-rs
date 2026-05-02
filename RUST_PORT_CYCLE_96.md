# Rust Port Cycle 96: Eating Events And Runtime Food Transcript

## Objective

Make runtime eating visible through command-line episodic output.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_96.md
```

## Implemented

Added food-specific episodic labels for fruit, shellfish, and seaweed, and a
runtime CLI test that opens a hungry ape fixture, runs one minute, and confirms
an eating transcript is emitted.

## Validation

Covered by `hungry_runtime_records_food_transcript` plus full workspace tests.

# Rust Port Cycle 123: Version And Section Parsing

## Objective

Parse native version blocks and section boundaries with native error behavior.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_123.md
```

## Implemented

The native reader requires `simul{...};` first, validates signature
`SIMULATED_APE_SIGNATURE`, rejects newer versions, and parses six-character
field tokens with comma/semicolon terminated numeric arrays.

## Validation

Version and land-section tests verify parsed tokens and checked values.

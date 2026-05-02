# Rust Port Cycle 82: Native Load Format Expansion

## Objective

Load the native fields added in cycle 81 while preserving older summary JSON.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_82.md
```

## Implemented

Native load parsing now accepts expanded `changes`, `braindata`, and
`events.episodic` objects. Partial C-shaped arrays are accepted where the C
format can provide sparse runtime data, while legacy summary fields remain
supported.

## Validation

Added a partially populated native JSON fixture covering brain probes, brain
state, inventory, preferences, pregnancy-related fields, and episodic entries.


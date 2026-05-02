# Rust Port Cycle 78: Episodic Memory Native State

## Objective

Extend Rust state and transfer handling for native episodic memory entries.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_78.md
```

## Implemented

`BeingSummary` now carries `[simulated_iepisodic; EPISODIC_SIZE]`. New beings
initialize episodic affect to `EPISODIC_AFFECT_ZERO`, matching C initialization.

Native transfer writes and reads `events.episodic`, including spacetime, names,
event, food, affect, and argument.

## Validation

Added save/open round-trip tests for episodic memory.


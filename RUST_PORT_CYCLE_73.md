# Rust Port Cycle 73: Awake Cycle Integration

## Objective

Create the Rust equivalent of the C awake-cycle coordinator.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_73.md
```

## Implemented

`BeingSummary::cycle_awake` now owns the per-minute awake work after universal
cycling:

```text
state discovery
temporary speed
food/eating
speed convergence
movement
energy cost
mass calculation
macro-state updates
```

This removes the previous summary-only random movement block from
`advance_minute`.

## Validation

Existing populated simulation tests and new awake-cycle tests verify one-minute
and multi-minute movement, state, energy, and body updates.


# Rust Port Cycle 102: Squabble And Show-Force Parity

## Objective

Port native-shaped squabble outcomes.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_102.md
```

## Implemented

Added aggression checks, victor/vanquished selection, disrespect, honor
adjustment, show-force versus attack energy costs, wound marking, flee speed,
facing, and fight episodic records.

## Validation

Added a direct squabble fixture for wounds, honor/flee behavior, and episodic
fight records.

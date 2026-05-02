# Rust Port Cycle 116: Food Depletion

## Objective

Port repeated-eating depletion behavior for land and intertidal foods.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_116.md
```

## Implemented

Added repeated-eating coverage over map-local food quantity. Eating from the
same location now reduces subsequent available maximum energy through the food
quantity gate.

## Validation

Added an eat-twice test that proves the second food source is lower than the
first at the same location.

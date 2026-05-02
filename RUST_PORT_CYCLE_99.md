# Rust Port Cycle 99: Social Attraction Calculations

## Objective

Port C-shaped attraction and prejudice scoring.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_99.md
```

## Implemented

Added pheromone/genetic variation, pigmentation, hair, height, and frame
attraction helpers. New social meetings now seed friend/foe from these scores
and observed features.

## Validation

Covered by social meeting feature and prejudice tests.

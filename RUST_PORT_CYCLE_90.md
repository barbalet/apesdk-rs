# Rust Port Cycle 90: Movement Energy Terrain Cost

## Objective

Expand movement energy from flat cost to terrain-aware native shape.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_90.md
```

## Implemented

Movement energy now includes slope dot product, uphill gene penalty, mass
contribution, swimming energy, and body-fat insulation.

## Validation

Updated movement tests to evaluate energy against seeded land.

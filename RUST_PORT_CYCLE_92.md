# Rust Port Cycle 92: Biology Operator And Food Value Port

## Objective

Port the biology operators used by terrain food lookup.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_92.md
```

## Implemented

Added the C operator table for area, height, water, moving sun, total sun,
salt, bush, grass, tree, seaweed, rockpool, and beach. Food values now use the
same grass, tree, bush, and dither shape as `entity/food.c`.

## Validation

Added seeded scans proving land and intertidal food sources can dominate.

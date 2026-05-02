# Rust Port Cycle 86: Terrain Operator Audit

## Objective

Map the C terrain operator stack needed by command-line movement and eating.

## Changed Files

```text
rust_port/terrain_food_audit.md
RUST_PORT_CYCLE_86.md
```

## Implemented

Documented the C dependencies for `land_operator_interpolated`, `land_vect2`,
`WATER_TEST`, tide handling, and biology operator lookup. The audit records
which pieces are now represented in Rust and where full C tile-map parity is
still missing.

## Validation

Covered by the cycle 86-95 validation run.

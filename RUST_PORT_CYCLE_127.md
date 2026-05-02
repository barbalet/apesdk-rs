# Rust Port Cycle 127: Volatile/Brain/Immune Binary Load

## Objective

Complete first-pass native being load for volatile, brain, and immune fields.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_127.md
```

## Implemented

Native `being{...};` loading now includes `brreg`, `brpro`, and `immun`
fields. Brain probe bytes are grouped into native six-byte probe records, and
immune bytes are mapped into antigens, shapes, antibodies, antibody shapes,
and little-endian random seeds.

## Validation

The native full-being fixture verifies braincode registers, probe fields,
immune arrays, and immune random seed.

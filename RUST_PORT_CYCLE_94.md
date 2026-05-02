# Rust Port Cycle 94: Intertidal Food Eating Parity

## Objective

Port intertidal food selection among seaweed, shellfish, and beach fallback.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_94.md
```

## Implemented

Added intertidal source selection using seaweed, rockpool, beach, and the C
beach dither formula. Beach fallback now produces no food, matching the native
intertidal path.

## Validation

Seeded food scans assert both seaweed and shellfish are reachable.

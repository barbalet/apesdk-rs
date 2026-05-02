# Rust Port Cycle 95: Food Absorption And Pathogen Ingestion

## Objective

Complete food absorption across Rust food types and connect eating to immune
pathogen ingestion.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_95.md
```

## Implemented

Added fruit, shellfish, seaweed, bird-egg, and lizard-egg absorption helpers,
the 320-energy eating cap, and pathogen acquisition through the immune antigen
arrays.

## Validation

Added tests for multiple food types, the cap, and seeded pathogen ingestion.

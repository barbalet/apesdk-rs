# Rust Port Cycle 117: Food Regrowth Cycle

## Objective

Verify time-based food recovery after depletion.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_117.md
```

## Implemented

Food quantity now recovers through land time and is tested across a long
regrowth interval back to full availability.

## Validation

The repeated-eating test advances land time through a full regrowth horizon and
checks that local quantity and available energy return to full.

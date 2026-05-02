# Rust Port Cycle 115: Food Quantity State

## Objective

Add map-local food availability state for repeated eating and regrowth work.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_115.md
```

## Implemented

Added per-map-cell food quantity, food-energy gating, eating depletion, and
time-based regrowth. Population cycling now passes mutable land into awake
being cycles so eating can update the terrain food cell.

## Validation

Added tests showing direct food depletion reduces available energy, land time
regrows the cell, and an eating awake cycle lowers the local food quantity.

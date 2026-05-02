# Rust Port Cycle 112: Land Operator Exactness

## Objective

Bring the Rust land operator closer to the native C inputs now that heights
come from tile bytes.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_112.md
```

## Implemented

Updated the moving-sun operator to use weather-seven bands and the native
weather-divide shape instead of a fixed clear-day offset. The operator stack
continues to use the C area, height, water, sun, fixed-sun, and salt formulas.

## Validation

Terrain biology and food dominance tests continue to pass with tile-backed
heights and weather-aware sun input.

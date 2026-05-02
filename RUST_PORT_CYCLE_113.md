# Rust Port Cycle 113: Weather And Moving Sun Inputs

## Objective

Port the time classification needed by native weather-seven behavior and wire
it into biology operators.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_113.md
```

## Implemented

Added native weather-seven constants, dawn/dusk classification, day/night
bands, and deterministic pressure-based cloudy/rainy categories. Biology
operators now use those bands for moving-sun intensity.

## Validation

Added tests for sunny/cloudy/rainy day range, clear/cloudy/rainy night range,
and exact dawn/dusk classification.

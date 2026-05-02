# Rust Port Cycle 110: High-Definition Topography

## Objective

Expose high-definition topography behavior required by native terrain
side-effects.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_110.md
```

## Implemented

Added lazy high-definition topography sampling using the same 8x bilinear grid
shape as C `math_bilinear_8_times`. Added a virtual high-res tide check using
the C fixed intertidal high-definition height band.

## Validation

Added high-definition tests for exact map-cell alignment, wraparound, and
high-res tide-band classification.

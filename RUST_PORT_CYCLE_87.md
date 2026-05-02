# Rust Port Cycle 87: Terrain Height And Slope Sampling

## Objective

Add Rust height and slope sampling for terrain-aware movement.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_87.md
```

## Implemented

Added `TerrainSample`, ape-space to map-space conversion, deterministic
genetics-backed topography, and C-shaped height/slope sampling for movement.

## Validation

Added deterministic terrain sample tests for height, slope, map position,
water, and intertidal state.

# Rust Port Cycle 114: Tide And Terrain Integration

## Objective

Verify tide, water, intertidal, and high-resolution tide behavior with
tile-backed heights.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_114.md
```

## Implemented

The existing lunar/solar tide state now runs against stored topography bytes.
`terrain_sample` derives water and intertidal state from tile heights, and
high-resolution tide classification is available through `highres_tide_at`.

## Validation

Existing tide tests and new high-resolution tide-band tests pass.

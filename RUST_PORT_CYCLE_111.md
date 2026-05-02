# Rust Port Cycle 111: Replace Procedural Height Source

## Objective

Switch Rust terrain sampling from on-demand procedural heights to tile-backed
heights.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_111.md
```

## Implemented

`height_at`, `terrain_sample`, slope reads, movement cost, water avoidance,
biology operators, and food selection now resolve heights through the primary
tile topography buffer.

## Validation

Existing terrain, movement, water, and eating tests pass against the tile
buffer source.

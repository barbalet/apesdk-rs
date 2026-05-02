# Rust Port Cycle 107: Rust Terrain Tile Data Model

## Objective

Add native-shaped Rust storage for command-line terrain tiles.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_107.md
```

## Implemented

Added `LandTile` with native tile genetics plus primary and working
topography buffers. `LandState` now owns the tile array and per-map-cell food
quantity state.

## Validation

Added tests that verify the default command-line tile count, buffer length,
tile genetics, primary/working buffer agreement, and deterministic seeded tile
sampling.

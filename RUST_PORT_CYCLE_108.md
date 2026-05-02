# Rust Port Cycle 108: Tile Generation Seed Parity

## Objective

Move seeded terrain generation onto tile genetics instead of planet genetics.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_108.md
```

## Implemented

`seed_genetics`, snapshot loading, and land reset now populate tile-backed
topography from the saved tile genetics. Reset preserves tile genetics while
clearing planet genetics, matching `tile_land_erase` ownership.

## Validation

Existing startup seed tests still preserve `[7633, 53305]` as tile genetics
after first-cycle land preparation while planet genetics clear to `[0, 0]`.

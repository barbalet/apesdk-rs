# Rust Port Cycle 109: Topography Map Generation

## Objective

Materialize topography into map buffers so terrain users read stored map bytes.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_109.md
```

## Implemented

Added deterministic tile regeneration across the full `MAP_AREA` and exposed
`topography_buffer` and `topography_at_map` accessors. The current generator
still uses the Rust deterministic formula as the fill source; the important
runtime shift is that later terrain reads now consume stored tile bytes.

## Validation

Added tests comparing direct map reads with indexed primary-buffer bytes and
repeat snapshots.

# Rust Port Cycle 124: Land Binary Load

## Objective

Load native land state from the C transfer section.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
crates/simape/src/lib.rs
RUST_PORT_CYCLE_124.md
```

## Implemented

`landd` now loads date, time, and land genetics into `LandSnapshot`, then
`SimState::from_startup_transfer` rebuilds tile-backed terrain from that seed.
CLI `open` accepts native transfer files with land sections.

## Validation

Added unit tests for native land loading and a `simape open` test for native
transfer startup state.

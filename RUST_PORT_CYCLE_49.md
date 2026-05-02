# Rust Port Cycle 49: Ape-Space Motion Constants

## Objective

Port the core map-to-ape-space constants needed for deterministic movement.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_49.md
```

## Implemented

Added Rust constants matching the command-line C build:

```text
MAP_APE_RESOLUTION_SIZE = 32768
APESPACE_BOUNDS = 32767
BEING_DEAD
BEING_HUNGRY
BEING_FULL
BEING_STATE_AWAKE
BIRTH_HEIGHT
BIRTH_MASS
MIN_CROWDING
DRIVES_MAX
```

Also added wrapping Ape-space helpers for bounded movement.

## Compatibility Notes

These constants match the C headers and are tested alongside the existing map
and terrain constants.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
93 tests passed
```

## Next Cycle

Cycle 50 should introduce deterministic minute cycling for populated summaries.

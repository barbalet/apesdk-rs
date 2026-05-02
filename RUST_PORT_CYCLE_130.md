# Rust Port Cycle 130: Full Binary Round Trip

## Objective

Round-trip Rust state through the native transfer reader and writer.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_130.md
```

## Implemented

The Rust-native transfer path can now write a populated state, load it back
through the same native parser, and preserve the covered land, being, brain,
immune, and event fields.

## Validation

Added a native transfer writer roundtrip test using an expanded being fixture.
Raw byte-structured `NA...` files remain rejected.

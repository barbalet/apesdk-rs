# Rust Port Cycle 66: Facing And Motion Vector Parity

## Objective

Port the C facing helpers used by awake movement and social orientation.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_66.md
```

## Implemented

Added native-shaped facing helpers on `BeingSummary`:

```text
facing_vector
set_facing_towards
wander
```

`facing_vector` now delegates to the toolkit sine table through
`vect2_direction`, using the C divisor scaling from `being_facing_vector`.
`set_facing_towards` uses `math_tan`, and `wander` preserves C-style 8-bit
angle wrapping.

## Validation

Added table-style tests for vector scaling, facing toward a vector, and signed
wander wrapping.


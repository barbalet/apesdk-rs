# Rust Port Cycle 67: Speed And Speed Advance

## Objective

Replace the single Rust speed summary with the native velocity history shape.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
crates/simape/src/lib.rs
RUST_PORT_CYCLE_67.md
```

## Implemented

`BeingSummary` now stores the ten-entry C velocity history. The public `speed`
accessor returns `velocity[0]`, and new helpers expose `velocity`,
`set_speed`, `speed_advance`, and `ten_minute_distance`.

The native transfer layer accepts old scalar velocity values for compatibility
and writes the C-shaped velocity array. Stats output now reports ten-minute
distance for `SPD`, matching the C command behavior more closely.

## Validation

Added unit coverage for speed setting, history shifting, ten-minute distance,
and compatibility with scalar native load fixtures.


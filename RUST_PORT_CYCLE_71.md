# Rust Port Cycle 71: Body Growth And Mass Calculation

## Objective

Port the native height, body-frame, body-fat, and mass calculation surface.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
crates/simape/src/lib.rs
RUST_PORT_CYCLE_71.md
```

## Implemented

Added native body constants and gene helpers for body frame and food/body
energy traits. `BeingSummary` now exposes real height in millimetres,
body frame, body fat, and a native-shaped mass calculation.

Juvenile eating during `cycle_awake` now grows height through the C
`ENERGY_TO_GROWTH` shape, and awake cycling recalculates mass.

## Validation

Added tests proving hungry juvenile eating increases height and recalculates
mass. Appearance output now reports real height, mass, body fat, and body frame.


# Rust Port Cycle 125: Being Constant/Delta Binary Load

## Objective

Load first-pass native being fields from `being{...};`.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_125.md
```

## Implemented

The native reader maps location, facing, speed, energy, birth date, random
seed, macro-state, brain state, height, mass, posture, inventory, parasites,
honor, attention, genetics, fetal genetics, drives, goals, preferences,
generation range, and child generation range into the existing C-shaped JSON
transfer object path.

## Validation

Added a native `being{...};` fixture that loads into `SimState` and verifies
the restored constant, delta, changes, and brain fields.

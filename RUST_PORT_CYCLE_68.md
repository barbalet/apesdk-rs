# Rust Port Cycle 68: Movement Energy

## Objective

Port the flat-land movement energy surface needed by awake cycling.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_68.md
```

## Implemented

Added `BeingSummary::move_energy`, using the same basic C formula shape:
speed-derived movement cost, a resting baseline, and a mass contribution.

The full C terrain slope, swimming, insulation, and hill-climb paths remain for
later terrain/body work, but awake beings now pay a movement cost through the
same hook used by the native engine.

## Validation

Added tests proving fast movement costs more energy than stationary awake
cycling.


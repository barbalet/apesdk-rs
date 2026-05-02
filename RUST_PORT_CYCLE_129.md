# Rust Port Cycle 129: Land And Population Binary Write

## Objective

Write land and population data in native transfer order.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_129.md
```

## Implemented

The native writer emits version, land date/time/genetics, and each being's
constant, delta, volatile, brain, immune, and non-empty event sections.

## Validation

Native writer roundtrip tests compare restored land, location, brain registers,
brain probes, immune seed, and episodic events.

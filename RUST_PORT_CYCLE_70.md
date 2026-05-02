# Rust Port Cycle 70: Body Constants And Body State

## Objective

Port the body constants and body attention state required by C command output.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
crates/simape/src/lib.rs
RUST_PORT_CYCLE_70.md
```

## Implemented

Added C body inventory constants, attention indexes, relationship descriptions,
and body inventory descriptions. `BeingSummary` now carries the native
`braindata.attention` array and transfers it through save/load and native struct
projection.

`stats` now prints the selected body part and relationship link from attention
state rather than hardcoded text.

## Validation

Added tests for body descriptions, default body attention, attention transfer,
and updated stats output.


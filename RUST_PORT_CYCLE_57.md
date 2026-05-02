# Rust Port Cycle 57: Awake State Constants And Descriptions

## Objective

Port C being state flags and C-shaped state/drive descriptions needed by
selected-being command output.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
crates/simape/src/lib.rs
RUST_PORT_CYCLE_57.md
```

## Implemented

Added Rust constants for the C being state flags and awake levels:

```text
BEING_STATE_ASLEEP
BEING_STATE_AWAKE
BEING_STATE_HUNGRY
BEING_STATE_SWIMMING
BEING_STATE_EATING
BEING_STATE_MOVING
BEING_STATE_SPEAKING
BEING_STATE_SHOUTING
BEING_STATE_GROOMING
BEING_STATE_SUCKLING
BEING_STATE_SHOWFORCE
BEING_STATE_ATTACK
BEING_STATE_NO_FOOD
FULLY_ASLEEP
SLIGHTLY_AWAKE
FULLY_AWAKE
```

Added `being_state_description` and `drive_description`, and wired `stats` to
show the selected ape's dominant drive and state instead of the fixed summary
wording.

## Compatibility Notes

The formatter remains compact like the C watch stats line, while still allowing
the current summary-backed output to remain readable.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
99 tests passed
```

## Next Cycle

Cycle 58 should port C energy accessors and saturating semantics.

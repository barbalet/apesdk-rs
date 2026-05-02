# Rust Port Cycle 59: Awake-State Calculation

## Objective

Port the core `being_awake` decision into Rust minute advancement.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_59.md
```

## Implemented

Added `BeingSummary::awake_level_for_time` using C-like rules:

```text
dead energy -> FULLY_ASLEEP
daytime -> FULLY_AWAKE
nighttime and hungry -> SLIGHTLY_AWAKE
nighttime and moving -> SLIGHTLY_AWAKE
otherwise -> FULLY_ASLEEP
```

The being summary now stores both a boolean awake convenience flag and the
native `awake_level` value that maps to `simulated_being_delta.awake`.

## Compatibility Notes

The nighttime water exception from C is not ported yet because terrain-water
queries are part of the later movement/body/food cycles.

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

Cycle 60 should add the universal-cycle skeleton.

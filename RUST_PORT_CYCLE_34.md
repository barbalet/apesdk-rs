# Rust Port Cycle 34: Reset Creates Population

## Objective

Move `reset` beyond an empty shell by creating an initial named population for
the Rust command line.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
crates/simape/src/lib.rs
RUST_PORT_CYCLE_34.md
```

## Implemented

`SimState::reset_new_simulation_from_land_seed` now creates:

```text
128 beings
max population 256
selected ape Ape 001
alternating male/female summary sex bits
```

The CLI `sim` output now reports the populated state after reset:

```text
Population: 128
Adults: 128   Juveniles: 0
```

## Compatibility Notes

This cycle intentionally gets useful population state into Rust before full C
cycle parity. Per-being movement, metabolism, social state, and real age
classification are still deferred.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
83 tests passed
```

## Next Cycle

Cycle 35 should include the new population summaries in Rust save/open transfer
round trips.

# Rust Port Cycle 31: C-Compatible Being Model Foundation

## Objective

Add Rust model structs for the C being, social, episodic, brain, immune, and
remains data shapes so later simulation work has a stable memory map to build
against.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_31.md
```

## Implemented

C-compatible `repr(C)` model structs:

```text
simulated_feature
simulated_featureset
simulated_isocial
simulated_iepisodic
simulated_iplace
simulated_ibrain_probe
simulated_immune_system
simulated_remains
simulated_being_delta
simulated_being_constant
simulated_being_events
simulated_being_brain
simulated_being_volatile
simulated_being
```

The port also added constants for social memory, episodic memory, chromosomes,
immune state, body inventory, attention, preferences, shouts, drives, sex codes,
and initial population sizing.

## Compatibility Notes

The layout test checks the offsets printed by the C `debug` command, including:

```text
simulated_being.braindata offset 4512
simulated_being.changes offset 4632
simulated_being.immune_system offset 4704
simulated_being size 4756
```

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

Cycle 32 should add a Rust-owned population/being summary layer that can power
CLI behavior before the full per-minute being engine is ported.

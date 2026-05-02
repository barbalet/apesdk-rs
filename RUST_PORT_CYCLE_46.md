# Rust Port Cycle 46: Native Being Transfer Shape

## Objective

Move populated Rust saves from summary-only fields toward the C JSON transfer
shape used by `universe/transfer.c`.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_46.md
```

## Implemented

`BeingSummary::transfer_object` now emits native-shaped objects with:

```text
name
delta
constant
changes
braindata
immune_system
```

The `delta` and `constant` objects use C field names such as
`direction_facing`, `stored_energy`, `date_of_birth`, `genetics`, and
`generation_range`.

## Compatibility Notes

The save format keeps Rust extension objects for drives, brain registers, and
immune seed so current CLI behavior round-trips while the full native transfer
surface is being ported.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
93 tests passed
```

## Next Cycle

Cycle 47 should add direct projection between `BeingSummary` and
`simulated_being`.

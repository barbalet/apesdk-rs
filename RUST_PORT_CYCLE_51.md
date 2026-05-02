# Rust Port Cycle 51: Populated SimState Advancement

## Objective

Advance land time and populated beings through one public state API.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
RUST_PORT_CYCLE_51.md
```

## Implemented

Added:

```text
SimState::advance_minutes
```

For empty simulations this keeps the previous land-only behavior. For populated
simulations it advances land one minute at a time and cycles each being summary.

## Compatibility Notes

The state kind transitions to `KIND_NOTHING_TO_RUN`, matching the previous
empty-step path.

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

Cycle 52 should wire `step` to this populated advancement.

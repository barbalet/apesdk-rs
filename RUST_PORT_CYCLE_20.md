# Rust Port Cycle 20: Empty Navigation Aliases

## Objective

Port the empty-population behavior for selected-ape navigation commands.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_20.md
```

## Implemented

Command-line behavior:

```text
next
previous
prev
```

With no apes present, all aliases now print:

```text
No apes selected. Trying (re)running the Simulation
```

## Compatibility Notes

This mirrors C `command_check_ape_present` for an empty group. Selection
rotation remains explicitly deferred until the Rust port has beings, groups, and
current selection pointers.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
printf 'interval\ninterval 2 hours\nlogging off\nevent\ntop\nepic\nnext\nprevious\nquit\n' | cargo run -q -p simape
```

Current result:

```text
67 tests passed
```

The smoke run confirmed interval reporting, logging toggle output, event
fallback, empty `top`/`epic`, and empty selected-ape navigation.

## Next Cycle

Cycle 21 should continue replacing unsupported command fallbacks with C-shaped
empty-state behavior. Good candidates are `watch`/`monitor`, `friends`/`social`,
or `debug`, depending on which C path has the smallest dependency footprint.

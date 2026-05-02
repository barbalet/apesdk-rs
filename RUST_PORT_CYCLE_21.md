# Rust Port Cycle 21: Empty Watch Aliases

## Objective

Port the empty-population behavior for `watch` and `monitor`.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_21.md
```

## Implemented

Command-line behavior:

```text
watch
watch all
monitor off
```

With no apes present, all forms now report:

```text
No apes selected. Trying (re)running the Simulation
```

## Compatibility Notes

This mirrors C `command_watch`, which calls `command_check_ape_present` before
parsing the response. That means `watch off` and `watch all` do not change watch
state when the group is empty.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
72 tests passed
```

## Next Cycle

Cycle 22 should port the empty-being failure paths for social graph and pathogen
detail commands.

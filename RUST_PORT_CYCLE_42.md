# Rust Port Cycle 42: Event and Episodic Surface

## Objective

Bring the `event` command back to C-like toggle behavior and give `episodic`
selected-ape output.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_42.md
```

## Implemented

`event` now supports:

```text
event
event on
event social
event off
```

The console tracks event mode internally, and `episodic` reports the selected
ape's episodic memory heading when a population exists.

## Compatibility Notes

Event recording itself is still future engine work. This cycle restores the
interactive command responses expected by users of the C console.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
89 tests passed
```

## Next Cycle

Cycle 43 should populate braincode, probe, and speech command summaries.

# Rust Port Cycle 18: Event Command Build Fallback

## Objective

Port the current command-line behavior of the C `event` command for this build,
where episodic logging is not compiled in.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_18.md
```

## Implemented

Command-line behavior:

```text
event
event social
event off
```

All forms route to the current non-episodic build message.

## Compatibility Notes

The Rust console now prints:

```text
Episodic not supported in this build
```

This mirrors the C `#else` path when `EPISODIC_ON` is absent. The on/social/off
branches should be revisited when episodic memory and logging are ported.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
67 tests passed
```

The new event transcript covers both empty and `social` responses.

## Next Cycle

Cycle 19 should port the empty-population output for ranking commands such as
`top` and `epic`.

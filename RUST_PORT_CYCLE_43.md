# Rust Port Cycle 43: Braincode, Probe, and Speech Summaries

## Objective

Use stored summary fields to make the remaining selected-being brain and speech
commands produce useful output.

## Changed Files

```text
crates/apesdk-sim/src/lib.rs
crates/simape/src/lib.rs
RUST_PORT_CYCLE_43.md
```

## Implemented

The summary model now transfers braincode registers, and the CLI reports
populated output for:

```text
braincode
probes
speech
```

The output includes deterministic register letters for each reset ape.

## Compatibility Notes

Probe rows are still a heading-only summary until the full brain probe engine
is ported, but the command no longer falls through to an unimplemented path for
selected beings.

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

Cycle 44 should add the `idea` command's populated summary output.

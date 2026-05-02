# Rust Port Cycle 29: Empty Speak Command

## Objective

Port the `speak` command out of the generic unsupported path for the current
empty Rust simulation state.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_29.md
```

## Implemented

Command-line behavior:

```text
speak
speak [file]
```

With no selected ape, Rust emits no console output and creates no file.

## Compatibility Notes

The C `speak` command assumes a selected being and routes through braincode
speech synthesis. The Rust shell cannot represent selected beings yet, so this
cycle deliberately keeps the empty path quiet and safe instead of preserving the
sampled C crash behavior.

Full speech generation remains deferred until beings, braincode, and the audio
stack are ported.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
77 tests passed
```

## Next Cycle

Cycle 30 should port `debug` audit output, the last command table entry that
still depends only on static command-line data.

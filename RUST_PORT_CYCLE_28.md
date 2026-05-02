# Rust Port Cycle 28: Alpha Command Surface

## Objective

Port the command-line surface for `alpha` without pulling in the full C audio
synthesis stack yet.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_28.md
```

## Implemented

Command-line behavior:

```text
alpha [file]
```

With a filename, Rust creates an empty file and emits no console output. With no
filename, Rust is quiet.

## Compatibility Notes

The C `alpha` command calls `speak_out` and normally has no console output. The
existing sampled C command-line binary created a zero-byte file for the alpha
smoke case, so this cycle preserves that visible side effect while deferring
full AIFF synthesis.

File creation failures report:

```text
ERROR: Failed create speak file! @ ./entity/speak.c 199
```

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

Cycle 29 should remove the generic unsupported fallback for `speak` while
keeping empty-selection behavior safe.

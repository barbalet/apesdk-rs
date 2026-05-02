# Rust Port Cycle 27: Default Script Command

## Objective

Port `script` command behavior for the command-line build currently represented
in this Rust port.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_27.md
```

## Implemented

Command-line behavior:

```text
script
script missing.ape
script saved.json
```

No filename is quiet. A missing file stops the simulation and emits no file-open
error, matching the C `io_disk_check` path. Existing files use the same startup
JSON loading path as `open` in the current Rust shell.

## Compatibility Notes

In the default C build, `APESCRIPT_INCLUDED` is undefined, so `command_script`
falls through to the same transfer load branch as `open`. The Rust port follows
that shape for existing files while continuing to use the Rust JSON transfer
format already implemented for `open`.

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

Cycle 28 should remove the generic unsupported fallback for `alpha` and preserve
the observed no-console-output command shape.

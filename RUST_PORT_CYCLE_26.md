# Rust Port Cycle 26: File Format Command

## Objective

Port the `file` command output used to inspect the command-line save/load file
format.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_26.md
```

## Implemented

Command-line behavior:

```text
file
file landd
file timed
file xxxxx
```

The Rust console now mirrors C `io_search_file_format` for the enabled default
format entries:

```text
simul
landd
being
```

and all enabled fields under those sections.

## Compatibility Notes

The C implementation prints this command directly with `printf`; Rust returns
the same text through the console response path so scripted CLI transcripts are
still deterministic.

Missing keys now report:

```text
ERROR: String not found @ ./toolkit/file.c 1458
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

Cycle 27 should port `script` command behavior for the default build, where
`APESCRIPT_INCLUDED` is undefined.

# Rust Port Cycle 25: Empty Genome and Idea Commands

## Objective

Port empty-population behavior for genome/genetics detail commands and the
current empty `idea` path.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_25.md
```

## Implemented

Command-line behavior:

```text
genome
genetics
idea
```

`genome` and `genetics` now use the same C `command_duplicate` empty-state
messages:

```text
ERROR: No being was specified @ ./universe/command.c 1311
ERROR: Being not found @ ./universe/command.c 1300
```

`idea` is quiet for an empty population, matching the C path where there are no
selected beings and no braincode comparisons to report.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
printf 'watch all\nfriends\nsocialgraph Ada\nbraincode\nspeech Ada\nstats\nstatus Ada\ngenome\ngenetics Ada\nidea\nquit\n' | cargo run -q -p simape
```

Current result:

```text
72 tests passed
```

The smoke run confirmed watch/monitor empty selection, duplicate-command no
being/missing being errors, quiet `idea`, and normal quit behavior.

## Next Cycle

Cycle 26 should continue reducing unsupported command fallbacks. Good next
targets are `file`, `alpha`, `speak`, and `debug`, each of which prints outside
the normal console response path in the C implementation.

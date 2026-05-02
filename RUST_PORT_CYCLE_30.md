# Rust Port Cycle 30: Debug Audit Command

## Objective

Port the `debug` command's command-line audit output.

## Changed Files

```text
crates/simape/src/lib.rs
RUST_PORT_CYCLE_30.md
```

## Implemented

Command-line behavior:

```text
debug
```

Rust now prints the same captured C layout audit snapshot for:

```text
simulated_being
simulated_isocial
simulated_iepisodic
```

## Compatibility Notes

The C command prints directly through `printf`. Rust returns the audit text
through the console response path so tests and scripted sessions can capture it.

This cycle also removed the now-unused generic `Unsupported` command action.
Every command in the current Rust command table now has an explicit handler,
even when deeper simulation behavior remains deferred.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
printf 'file landd\nfile timed\nscript missing.ape\nalpha /private/tmp/simape_rs_alpha_cycle30.aiff\nspeak /private/tmp/simape_rs_speak_cycle30.aiff\ndebug\nquit\n' | cargo run -q -p simape
```

Current result:

```text
77 tests passed
```

The smoke run confirmed file-format lookup, missing-script stop behavior,
quiet alpha/speak command output, debug audit output, and normal quit behavior.

## Next Cycle

The 30-cycle command-line surface pass is complete. The next phase should move
from empty-shell command parity into the model work needed for real apes:
being/group state, selection, braincode speech, and simulation cycling.

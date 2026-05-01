# Rust Port Cycle 02: Golden CLI Transcripts

## Objective

Capture representative command-line behavior from the current C `simape` binary
so the Rust port can compare its console behavior against known-good transcripts.

## Added Files

```text
golden/cli/README.md
golden/cli/run_cli_session.expect
golden/cli/sessions/help.commands
golden/cli/sessions/help_errors.commands
golden/cli/sessions/state_save_load.commands
golden/cli/transcripts/help.txt
golden/cli/transcripts/help_errors.txt
golden/cli/transcripts/state_save_load.txt
```

## Transcript Driver

The driver uses Expect to spawn `../simape` under a PTY, send commands from a
session file, and log the console transcript.

The POSIX C console runs through threads, so the driver drains console output
after each command. This avoids the output interleaving seen when multiple
commands are piped into stdin at once.

The current `longterm.c` entry point returns `1` even after a normal `quit`, so
the driver treats EOF as success and ignores the child exit status.

## Captured Sessions

`help.txt` captures:

```text
help
quit
```

`help_errors.txt` captures:

```text
help run
help missing
unknown
quit
```

`state_save_load.txt` captures:

```text
sim
memory
ape
save /private/tmp/simape_cycle02_save.txt
open /private/tmp/simape_cycle02_save.txt
quit
```

The save/load session currently exits during `open` after loader errors, before
the final `quit` command is sent. This is current C behavior and is preserved in
the captured transcript.

## Validation

The three transcript commands completed through the Expect driver. A search for
driver and stdin artifacts found no `Console failure` or `timed out` lines in
the transcript files.

The `state_save_load.txt` transcript contains random startup seed and generated
file-size values. It should be treated as a smoke transcript until the next
cycle adds normalization or a deterministic seed path.

## Next Cycle

Cycle 03 should inventory the public C API and core type surface that must be
ported first, starting with `toolkit/toolkit.h`, `sim/sim.h`, and
`universe/universe.h`.

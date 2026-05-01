# CLI Golden Transcripts

These transcripts capture the current C command-line `simape` behavior for the
Rust port to preserve.

## Sessions

- `help.txt` is an exact command-list baseline.
- `help_errors.txt` is an exact targeted-help and unknown-command baseline.
- `state_save_load.txt` is a smoke baseline for state, memory, selected ape,
  save, and open behavior. It includes random seed and generated file-size
  values, so future automated comparisons should normalize those fields before
  diffing.

Regenerate them from the repository root with:

```sh
./golden/cli/run_cli_session.expect ../simape golden/cli/sessions/help.commands golden/cli/transcripts/help.txt
./golden/cli/run_cli_session.expect ../simape golden/cli/sessions/help_errors.commands golden/cli/transcripts/help_errors.txt
./golden/cli/run_cli_session.expect ../simape golden/cli/sessions/state_save_load.commands golden/cli/transcripts/state_save_load.txt
```

The C `longterm.c` entry point currently returns `1`, so the driver treats EOF
as success and ignores the child exit status. The POSIX console is threaded, so
the driver drains output after each command to avoid interleaving output from
separate console reads.

`state_save_load.commands` writes its temporary save file to:

```text
/private/tmp/simape_cycle02_save.txt
```

That file is intentionally not part of the repository because it contains
randomly seeded simulation state.

Current C behavior: opening the JSON file saved from the startup simulation
fails in the loader with `Signature not first in file`. The transcript captures
that behavior as the compatibility target until the Rust port explicitly decides
to fix or migrate the save/load format.

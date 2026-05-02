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
- `empty_startup_matrix.commands` covers startup help, memory, file, save/open,
  and no-population detail errors.
- `populated_short_matrix.commands` covers a reset, short run, detail commands,
  braincode/probe/speech output, idea output, and navigation.
- `save_open_matrix.commands` covers JSON, native-text, framed-binary, and
  missing-file continuity.
- `command_edges.commands` covers missing arguments, aliases, malformed format
  lookup, targeted help errors, and unknown commands. Rust-only unit tests cover
  the unsupported `run forever` behavior because native C intentionally keeps
  running until stopped.

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
fails in the loader with `Signature not first in file`. The Rust default CLI now
preserves that behavior; alternate library compatibility loaders are kept out of
the native command-line parity gate.

## Harness

The newer local harness scripts wrap these sessions for C/Rust comparison:

```sh
scripts/build_native_simape.sh /private/tmp/apesdk_native_harness
NATIVE_SIMAPE=/private/tmp/apesdk_native_harness/simape \
RUST_SIMAPE=target/debug/simape \
scripts/run_cli_transcripts.sh /private/tmp/apesdk_transcripts
```

Use `scripts/normalize_transcript.sh` only for older fixture review where
volatile paths or generated lengths are already documented by the fixture. The
strict native C gate below does not use this normalizer.

For final release-hardening checks:

```sh
scripts/run_release_debug_gate.sh /private/tmp/apesdk_release_debug_gate
scripts/performance_smoke.sh /private/tmp/apesdk_performance_smoke
```

For the stricter native C gate introduced after cycle 265:

```sh
scripts/run_absolute_parity_ci.sh /private/tmp/apesdk_absolute_parity
```

That path does not use `scripts/normalize_transcript.sh`; it only removes CRLF
transport characters before diffing.

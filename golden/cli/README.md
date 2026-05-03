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
  populated `run forever` stop behavior because native C intentionally keeps
  running until stopped.
- `run_forever_empty.commands` covers the safe native `run forever` command
  text and `quit` path. State-visible `run forever` parity remains in the
  cooperative Rust tests until startup-cycle population parity is promoted.
- `runtime_edges_empty.commands` covers empty-population runtime/detail
  commands, selected-ape navigation without a selected ape, reset/clear
  ordering, and stop/quit output. It is promoted into the raw transcript gate.
- `timing_stop_forever.commands` covers PTY stop behavior after the safe empty
  `run forever` path and is owned by `scripts/run_interactive_timing_gate.sh`.
- `long_seeded_command_corpus.commands`, `multi_day_runtime_matrix.commands`,
  `multi_month_runtime_matrix.commands`, `save_open_runtime_continuity.commands`,
  and `exhaustive_command_surface.commands` are the cycle 341-350 promotion
  corpus. They cover long seeded runtime, top/epic filters, save/open
  continuity, detail commands, quiet file-producing commands, and every visible
  command/alias surface. They are fixture inputs first; promote each into the
  strict raw transcript gate only after its native C value drift is closed.

Regenerate them from the repository root with:

```sh
./golden/cli/run_cli_session.expect ../simape golden/cli/sessions/help.commands golden/cli/transcripts/help.txt
./golden/cli/run_cli_session.expect ../simape golden/cli/sessions/help_errors.commands golden/cli/transcripts/help_errors.txt
./golden/cli/run_cli_session.expect ../simape golden/cli/sessions/state_save_load.commands golden/cli/transcripts/state_save_load.txt
./golden/cli/run_cli_session.expect ../simape golden/cli/sessions/run_forever_empty.commands golden/cli/transcripts/run_forever_empty.txt
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
transport characters before diffing. It now also runs
`scripts/run_interactive_timing_gate.sh` and `scripts/run_fuzz_parity_ci.sh` so
EOF, closed-stdin output-class parity, command fuzz, malformed save, binary
mutation, and seed/population repeatability artifacts are captured from the same
pinned date. It also runs `scripts/run_engine_trace_value_gate.sh` for the
currently promoted direct engine startup, first-cycle baseline, invariant, and
runtime-core value subset, plus
`scripts/run_terrain_food_first_cycle_gate.sh` for exact startup and
first-cycle terrain/weather/food rows. It also runs
`scripts/run_terrain_food_value_inventory.sh` for the still-open multi-day
terrain/weather/food inventory and
`scripts/run_selected_being_value_inventory.sh` for exact selected-being
first-cycle native initialization plus after-day runtime drift inventory.

Use `scripts/run_pending_corpus_inventory.sh` to list the current blockers for
the long seeded, multi-day, multi-month, save/open continuity, and exhaustive
command-surface sessions before attempting promotion into the strict raw
transcript gate.

For the broader required gate and matrix checks:

```sh
scripts/run_platform_absolute_parity_gate.sh /private/tmp/apesdk_platform_gate
scripts/run_profile_compiler_matrix_gate.sh /private/tmp/apesdk_profile_matrix
scripts/run_required_absolute_parity_pipeline.sh /private/tmp/apesdk_required_absolute
```

Closed stdin, verbose-command EOF, and bounded-run EOF use native-backed timing
canonicalizers. The raw native console can omit, reorder, or duplicate
console-failure output around EOF; the gate accepts only the observed native
output class and then diffs the canonical C/Rust transcript exactly.

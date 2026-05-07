# Final Parity Closure Register

Cycles 266-300 retire the prior documented-difference model.

## Status

No difference is treated as complete by documentation alone. Each parity
category is now one of:

- exact under the absolute gate
- represented by an open fixture task
- outside the default command-line path reachable from `longterm.c`

## Closed Items

- Build metadata drift: native C and Rust harness builds share the
  `APESDK_FULL_DATE` value.
- Source-path drift: the native harness compiles C sources with `./folder/file.c`
  paths so error locations match Rust's command output.
- Transcript normalization drift: the absolute transcript gate uses only
  transport cleanup.
- Empty-file open drift: Rust now emits the stable native toolkit allocation
  diagnostic when `open` targets an existing empty file. Volatile native macOS
  malloc diagnostics are filtered by the malformed-loader canonicalizer.
- Empty-runtime command drift: no-selected-being detail commands, empty
  `socialgraph`/`pathogen`, reset/clear ordering, and stop/quit behavior are now
  promoted under `runtime_edges_empty`.
- Reset/startup population timing drift: Rust `reset` and `clear` now match
  native C's pending simulation behavior, with zero command-visible population
  until the first engine cycle. `step`, finite `run`, and populated
  `run forever` initialize that pending state before advancing it.
- Redirected EOF and closed-stdin drift: EOF after quiet/verbose commands,
  bounded-run EOF, closed stdin, and PTY stop-after-forever are now exact under
  the interactive timing gate. Native console-thread output classes for quiet
  EOF, verbose-command EOF, bounded-run EOF, and closed stdin are canonicalized
  by native-backed rules before C/Rust diffing.
- Malformed-loader promoted drift: missing files, empty files, bad JSON/random
  text native parser diagnostics, and truncated native text recovery are exact
  under `scripts/run_malformed_save_fuzz.sh`. Remaining malformed fixtures are
  split into stdout/stderr/combined inventory artifacts before promotion.
- Direct raw populated territory drift: Rust now preserves the full native
  `terit=` word stream when loading direct `tranfer_out()` artifacts and emits
  those words unchanged from the raw writer. The direct raw value gate now
  reports empty startup, reset startup, and populated after-one-cycle artifacts
  byte-exact for the current oracle set.
- Selected minute-60 runtime core drift: Rust now matches C for the promoted
  selected-being minute-60 energy, location, facing, speed, honor, mass,
  awake/state, drives, episodic sample, immune sample, and preference values.
  The value gate includes a mutation check; minute-60 brain/social and later
  runtime rows remain open.

## Open Fixture Tasks

- Retarget Rust default command-line `save` to the completed native C transfer
  path. Native C now writes native transfer text through `tranfer_out()` and
  reopens it through `tranfer_in()`; Rust `open` reads that format, but Rust
  `save` still reflects the older JSON-compatible target.

- Promote direct C state trace emitters for deeper terrain, braincode, social,
  lifecycle, immune, movement, and save/load categories. The startup,
  first-cycle baseline, invariant, and runtime-core engine trace value subset is
  now promoted by `scripts/run_engine_trace_value_gate.sh`; first-cycle terrain
  and selected-being native initialization/body values are also exact. After-day
  selected-being movement, body, honor, brain, social, episodic, and immune
  runtime values remain open.
- Promote broader raw native binary artifacts after C oracle targets generate
  additional reachable command-line byte fixtures. The direct `tranfer_out()`
  raw oracle is now byte-exact for empty, reset, and after-one-cycle artifacts;
  social-heavy, immune-heavy, terrain-heavy, and save/open-derived raw fixtures
  still need exact promotion.
- Promote the cycle 341-350 long seeded, multi-day, multi-month, save/open, and
  exhaustive command-surface scripts after direct trace value drift is closed.
  `scripts/run_pending_corpus_inventory.sh` records their current blockers:
  long runtime values, save/open continuity, selected-being detail values, and
  file-producing command ordering.
- Promote the broader populated raw fixture set: social-heavy, immune-heavy,
  terrain-heavy, and save/open-derived direct raw artifacts.
- Final signoff readiness: `scripts/run_final_signoff_readiness.sh` currently
  reports blocked categories for selected runtime inventory, selected-minute
  inventory, save/open continuity, populated raw fixture coverage, and exact
  corpus promotion. Strict mode remains intentionally non-zero until those
  categories are exact.
- Replace the remaining fuzz triage markers with exact C/Rust diffs after
  seeded population trace values and the broader raw fixture set are available.
- Promote the remaining malformed-loader inventory cases after their exact
  native parser and recovery semantics are implemented.

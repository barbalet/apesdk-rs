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
- Direct raw startup/reset drift: Rust now emits the C-shaped raw native land
  sections (`landd`, `topog`, `weath`) from the raw writer. The direct raw
  value gate reports empty startup and reset startup byte-exact, and populated
  values exact across after-one-cycle, social-heavy, immune-heavy,
  terrain-heavy, and save/open-derived direct raw fixtures. Broader populated
  artifact byte exactness remains open for those populated artifacts.
- Populated Rust internal save/open state drift: Rust native transfer now
  preserves being name, awake state, social/episodic slot order, land payload,
  immune, brain/probe, random state, and selected identity across a populated
  Rust save/open trace.
- Selected minute-60 runtime core drift: Rust now matches C for the promoted
  selected-being minute-60 energy, location, facing, speed, honor, mass,
  awake/state, drives, episodic sample, immune sample, and preference values.
  The value gate includes a mutation check; minute-60 brain/social and later
  runtime rows remain open.

## Open Fixture Tasks

- Promote the native command-line save/open roundtrip into the broader strict
  corpus now that Rust `save` writes native transfer text and C transfer files
  include topography/weather byte sections. Rust now preserves those land bytes
  semantically across load and re-emit, and Rust internal populated
  before-save/after-open state continuity is exact. Remaining save/open drift is
  in C/Rust runtime transcript speed, post-load advancement, and artifact byte
  continuity promotion.

- Promote direct C state trace emitters for deeper terrain, braincode, social,
  lifecycle, immune, movement, and save/load categories. The startup,
  first-cycle baseline, invariant, and runtime-core engine trace value subset is
  now promoted by `scripts/run_engine_trace_value_gate.sh`; first-cycle terrain
  and selected-being native initialization/body values are also exact. After-day
  selected-being drift is now bucketed: selection and body are exact, while
  movement, energy, honor/drives, brain/probe, social, episodic, and immune
  runtime values remain open. The day-one slice inventory narrows the earliest
  selected divergence to honor/drives at minute 120, and movement, energy, and
  body state at minute 180.
- Promote broader raw native binary artifact bytes. The direct `tranfer_out()`
  raw oracle now includes after-one-cycle, social-heavy, immune-heavy,
  terrain-heavy, and save/open-derived populated fixtures, and their value
  summaries load exactly in Rust. Populated byte-exact promotion remains open
  for the broader fixture set; byte-diff inventory now places the first
  difference for all five populated scenarios in `being{` sections.
- Promote the cycle 341-350 long seeded, multi-day, multi-month, save/open, and
  exhaustive command-surface scripts after direct trace value drift is closed.
  `scripts/run_pending_corpus_inventory.sh` records their current blockers as
  concrete categories: day-one movement/energy/honor runtime,
  brain/social/episodic/immune runtime, save/open post-load day continuity,
  save/open raw transcript, selected-minute brain/social/detail values, and
  file-producing command ordering.
- Promote brain/social/episodic/immune runtime buckets from inventory to exact.
  The runtime inventory now marks brain/probe and social as first diverging at
  minute 60, immune at minute 180, and episodic at minute 660.
- Promote the broader populated raw fixture set from value-exact to byte-exact:
  after-one-cycle, social-heavy, immune-heavy, terrain-heavy, and
  save/open-derived direct raw artifacts are present and value-gated, but still
  report populated byte promotion pending.
- Final signoff readiness: `scripts/run_final_signoff_readiness.sh` currently
  reports blocked categories for selected/after-day runtime inventory,
  day-one movement/energy/honor runtime, brain/social/episodic/immune runtime,
  save/open continuity, broader populated raw byte promotion, and exact corpus
  promotion. Strict mode remains intentionally non-zero until those categories
  are exact.
- Replace the remaining fuzz triage markers with exact C/Rust diffs after
  seeded population trace values and the broader raw fixture set are available.
- Promote the remaining malformed-loader inventory cases after their exact
  native parser and recovery semantics are implemented.

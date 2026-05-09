# Absolute Pipeline Status

Cycles 361-370 add the harness layer needed before the final no-triage parity
work:

- `scripts/run_platform_absolute_parity_gate.sh` records host platform,
  compiler, Rust toolchain, and can optionally run the absolute gate on the host.
- `scripts/run_profile_compiler_matrix_gate.sh` compares native release, native
  debug, Rust debug, and Rust release transcripts for the promoted command
  sessions.
- `scripts/run_absolute_parity_failure_smoke.sh` proves trace, transcript, and
  save-byte mutations fail the relevant comparison primitives.
- `scripts/run_native_raw_binary_oracle_gate.sh` verifies direct native
  `tranfer_out()` byte artifacts and token byte maps are reproducible.
- `scripts/run_native_raw_binary_value_gate.sh` compares direct native raw C
  value summaries against Rust loads and checks exact empty/reset raw
  roundtrips.
- `scripts/run_engine_trace_value_gate.sh` compares the promoted direct engine
  startup, first-cycle baseline, invariant, and runtime-core value subset and
  proves the comparison fails on a deliberate one-field mutation.
- `scripts/run_required_absolute_parity_pipeline.sh` composes platform,
  absolute, profile/compiler, and failure-smoke gates.
- `scripts/run_interactive_timing_gate.sh` now includes exact EOF-after-quiet,
  EOF-after-verbose, bounded-run EOF, closed-stdin, and stop-after-forever
  timing behavior.
- `scripts/run_timing_regression_lock.sh` rejects extra console failures,
  missing banners, and stop/quit ordering drift while preserving the native
  output-class rule for quiet EOF and verbose-command EOF failures.

Remaining open no-triage categories are malformed loader diagnostics, raw binary
byte/value oracles, seeded population engine trace values, and promotion of the
long seeded/multi-day/save-open/exhaustive corpora.

Cycle 371-380 update: promoted malformed loader fixtures now include missing,
empty, bad JSON, random text, and truncated native text. The remaining
malformed cases are inventory artifacts, and the absolute pipeline now runs the
native raw byte oracle reproducibility gate.

Cycle 381-390 update: direct native raw transfers without `landd{}` now load in
Rust with the same default land fallback as C. The absolute pipeline now runs
the raw value gate. At that point, empty and reset raw transfers roundtripped
byte-for-byte while populated raw transfer values matched C at the summary
level.

Cycle 391-400 update: the absolute pipeline now runs the promoted engine trace
value subset gate. The pending long/runtime/save-open/exhaustive corpora are
classified by `scripts/run_pending_corpus_inventory.sh`; at that point they
remained blocked by reset/startup and deeper selected-being runtime value drift.

Cycle 401-410 update: the Rust direct engine trace now advances the same
pending `KIND_START_UP` lifecycle phase as C's first `sim_cycle()`. The value
gate now promotes first-cycle clock/population/runtime-core fields and the
first-cycle social/episodic/territory/preference baseline.

Cycle 441-480 update: command-visible reset/startup timing now follows native
C's pending simulation shape. `reset` and `clear` show zero population until
the first engine cycle, while `step`, finite `run`, and populated
`run forever` initialize and advance the population. The absolute pipeline
passes with exact empty runtime edges, exact startup/first-cycle selected-being
values, and raw empty/reset byte streams. At that point, populated runtime
transcript parity remained pending on native identity/detail values,
long-runtime engine values, save/open continuity, and populated raw territory
byte emission.

Cycle 481-510 update: direct raw populated territory words are now preserved and
the raw value gate reports the populated after-one-cycle artifact as byte-exact
for the current oracle set. The absolute pipeline now also runs
`scripts/run_corpus_promotion_inventory.sh` and
`scripts/run_selected_minute_trace_inventory.sh`. The corpus inventory keeps the
five long/runtime/save-open/exhaustive sessions visible as blocked rather than
silently pending. The selected-minute inventory emits 39 C/Rust rows from the
first day, confirms schema and row-count parity, and records the remaining
after-day selected-being drift as an inventory diff.

Cycle 511-540 update: the absolute pipeline now runs a selected minute-60 value
gate, save/open continuity inventory, and populated raw fixture inventory. The
minute-60 selected core is exact for movement/body/energy/honor/drive samples,
while minute-60 brain/social and later selected-minute rows remain inventory.
Save/open continuity is present on the default native transfer path; remaining
promotion work is value-level populated runtime drift and the broader populated
raw fixture set tracked as social-heavy, immune-heavy, terrain-heavy, and
save/open-derived artifacts.

Cycle 541-550 update: exact corpus promotion and final signoff now have strict
readiness commands. `scripts/run_exact_corpus_promotion_gate.sh` fails while any
promoted corpus remains blocked. `scripts/run_final_signoff_readiness.sh`
reports the combined selected-runtime, save/open, populated-raw-fixture, and
corpus status, and exits non-zero in strict mode with
`APESDK_REQUIRE_ABSOLUTE_SIGNOFF=1`.

Cycle 601-630 update: the absolute pipeline now has an after-day drift
inventory gate. That gate buckets selected-being day-one drift and reports
selection/body exact with movement, energy, honor/drives, brain/probe, social,
episodic, and immune still inventory-only. The direct raw oracle now generates
all broader populated fixture families; the raw value gate covers all seven
artifacts and final readiness separately blocks on the five populated
byte-pending scenarios.

Cycle 631-660 update: the absolute pipeline now also runs day-one slice and
brain/social runtime inventories. Final readiness records first-mismatch bucket
counts for movement/energy/body/honor and brain/social/episodic/immune runtime
fields, plus save/open post-load minute/day continuity. Save/open is exact
through one post-load minute and inventory-only by one post-load day.

Cycle 661-680 update: the absolute pipeline now includes populated raw
byte-diff inventory. Final readiness reuses the native raw value gate for byte
diffing instead of generating duplicate raw artifacts, reaches a manifest under
the current host storage limit, and reports `status=blocked blockers=5`.

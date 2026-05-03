# Corpus Completeness Audit

The command-line runtime reachable from `longterm.c` uses these native folders:

- `toolkit`
- `script`
- `render`
- `sim`
- `entity`
- `universe`
- `longterm.c`

Absolute parity coverage map:

- CLI command surface: promoted smoke coverage for help, targeted help, unknown
  commands, command edges, safe empty `run forever`, empty runtime/detail
  commands, file-format lookup, logging, event toggles, watch, interval, exact
  EOF-after-command timing, native-backed closed-stdin output-class timing, and
  PTY stop-after-forever timing.
- Save/open: default JSON save and native-text open behavior now follows C.
- Binary: default CLI no longer uses the Rust framed-binary substitute.
- Binary artifacts: `golden/absolute/binary_scenarios.txt` inventories the
  reachable command-line save bytes and the raw binary scenarios that still need
  a dedicated native oracle target.
- Engine traces: command-visible native trace extraction exists for startup
  land/time/tide smoke checks, and a direct C engine trace probe now covers the
  broad field schema for deeper categories. `scripts/run_engine_trace_value_gate.sh`
  promotes the startup, first-cycle baseline, invariant, and runtime-core value
  subset with a deliberate mutation negative check; exact C/Rust runtime value
  closure remains open by category.
- Pending promotion corpus: `scripts/run_pending_corpus_inventory.sh`
  inventories the long seeded, multi-day, multi-month, save/open continuity, and
  exhaustive command-surface sessions and classifies their current blockers.
  `scripts/run_corpus_promotion_inventory.sh` wraps that inventory for the
  absolute pipeline, fails on missing session files, and optionally records raw
  diff blockers. Reset/startup population timing and direct raw populated
  territory bytes are no longer classified as blockers. The active blockers are
  long runtime engine value drift, save/open continuity drift, selected-being
  detail value drift, and file-producing command ordering.
- Save/open promotion inventory:
  `scripts/run_save_open_continuity_inventory.sh` records the populated
  save/open/run session and can run the raw C/Rust transcript diff as an
  inventory artifact. The optional diff remains blocked until populated runtime
  continuity and failed-open output ordering are exact.
- Populated raw fixture inventory:
  `scripts/run_populated_raw_fixture_inventory.sh` verifies the current direct
  raw oracle artifacts and tracks the missing social-heavy, immune-heavy,
  terrain-heavy, and save/open-derived populated raw fixtures needed before
  broader raw corpus promotion.
- Exact corpus promotion gate:
  `scripts/run_exact_corpus_promotion_gate.sh` is the strict counterpart to the
  inventory. It exits non-zero unless every promoted corpus session is ready and
  no optional raw diff is blocked. It currently blocks by design because all
  five promotion corpora remain blocked.
- Fuzzing: deterministic command grammar plus promoted malformed missing,
  empty, bad JSON, random text, and truncated-native save fixtures now run under
  exact C/Rust diffing. Remaining malformed loader cases are inventoried with
  split stdout/stderr artifacts. Raw binary mutation and seed/population fuzzing
  are captured as repeatable triage artifacts until native byte and value
  oracles are promoted.

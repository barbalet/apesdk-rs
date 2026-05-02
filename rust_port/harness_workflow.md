# C/Rust Harness Workflow

Cycles 226-245 add the first local harness layer around the native C and Rust
command-line binaries.

## Build

```sh
scripts/build_native_simape.sh /private/tmp/apesdk_native_harness
```

The script builds native C `simape`, Rust `simape`, and writes
`native_build_manifest.txt` with compiler and toolchain versions.

## Transcript Capture

```sh
NATIVE_SIMAPE=/private/tmp/apesdk_native_harness/simape \
RUST_SIMAPE=target/debug/simape \
scripts/run_cli_transcripts.sh /private/tmp/apesdk_transcripts
```

The transcript wrapper runs every command file under `golden/cli/sessions`.
When Expect is available, it uses the existing PTY driver. Otherwise it falls
back to stdin piping for local smoke runs.

## Normalization

`scripts/normalize_transcript.sh` normalizes line endings, temporary paths, and
known volatile string-length lines. It intentionally does not normalize command
output, counts, state values, or event text.

## Trace Diffing

`scripts/trace_diff.sh expected.trace actual.trace` reports a pass/fail summary
and the first trace mismatch. Rust unit tests also exercise trace diff behavior
directly through `diff_trace_text`.

## Manifest

`golden/fixture_manifest.txt` lists session, transcript, and trace fixtures with
their gate and normalization category. It is intentionally line-oriented so it
can be reviewed and edited without a schema dependency.

## Full Local Pass

```sh
scripts/run_parity_ci.sh /private/tmp/apesdk_parity_ci
```

This runs Rust formatting, Rust tests, native C build, Rust build, and transcript
capture in a stable order.

## Release/Debug Gate

```sh
scripts/run_release_debug_gate.sh /private/tmp/apesdk_release_debug_gate
```

This builds debug and release Rust `simape`, runs every CLI session in both
profiles, normalizes the Rust transcripts, and diffs them.

## Performance Smoke

```sh
scripts/performance_smoke.sh /private/tmp/apesdk_performance_smoke
```

This records lightweight wall-clock smoke timings for the help, populated
short-run, and save/open command matrices. It is a regression tripwire, not a
substitute for transcript or trace gates.

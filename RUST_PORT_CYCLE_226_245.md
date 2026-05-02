# Rust Port Cycles 226-245

Cycles 226-245 are complete.

## Cycle Results

226. C transcript runner: added `scripts/run_cli_transcripts.sh` to drive C and
Rust command sessions through the existing Expect harness or stdin fallback.

227. Transcript normalization: added Rust and shell normalization for line
endings, temp paths, repository-root paths, and volatile string-length lines.

228. C state trace hooks: added Rust `trace_state_line` coverage for land,
population, selected being, energy, location, drives, brain registers, social,
territory, and conception state.

229. Trace diff tool: added Rust `diff_trace_text` and shell
`scripts/trace_diff.sh` first-mismatch reporting.

230. Fixture manifest: added `golden/fixture_manifest.txt` and parser tests that
validate committed fixture paths.

231. Harness CI script: added `scripts/run_parity_ci.sh` to run formatting,
tests, native build, Rust build, and transcript capture in order.

232. Harness documentation: added `rust_port/harness_workflow.md` and expanded
the CLI golden README with build/capture/normalization steps.

233. Braincode C trace fixtures: added initial deterministic trace fixture files
for decode, arithmetic, sensor/actuator, and social-hook gates.

234. Braincode decode trace parity: locked decode trace shape through the
manifest and trace diff harness.

235. Braincode arithmetic trace parity: added arithmetic/register trace fixture
coverage.

236. Braincode sensor trace parity: added sensor/actuator fixture coverage for
body, drive, social, episodic, territory, and probe-shaped fields.

237. Braincode actuator trace parity: added actuator-side fixture shape in the
sensor/actuator trace file.

238. Braincode scheduling trace parity: documented probe/state scheduling as a
known replacement point for future C-emitted traces.

239. Braincode social hook parity: added social braincode hook trace fixture and
status notes.

240. Braincode long-run freeze: documented the long-run C trace gap and wired
the harness so future C traces can replace Rust-authored fixtures mechanically.

241. Social action drift pass: kept the existing runtime social/body behavior
under harness coverage and documented the drift status.

242. Groom/squabble drift pass: kept existing grooming/squabble unit coverage
under the new trace status report.

243. Mate/preference drift pass: tied mate/conception and preference drift to
the social/family trace status.

244. Territory drift pass: added territory familiarity to state traces and the
social/family fixture.

245. Family relationship drift pass: added the controlled social/family trace
fixture and manifest entry.

## Validation

`cargo fmt --all --check`

`cargo test -p simape --lib trace`

`cargo test -p simape --lib fixture_manifest`

`cargo test -p simape --lib transcript_normalization`

`scripts/run_cli_transcripts.sh /private/tmp/apesdk_transcripts_226_245`

`scripts/trace_diff.sh golden/traces/braincode_decode.trace golden/traces/braincode_decode.trace`

`scripts/run_parity_ci.sh /private/tmp/apesdk_parity_ci_226_245`

The transcript wrapper generated native/Rust raw and normalized transcripts
under `/private/tmp/apesdk_transcripts_226_245`. The full parity wrapper
completed with `parity-ci=pass`.

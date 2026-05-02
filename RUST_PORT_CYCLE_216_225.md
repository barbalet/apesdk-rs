# Rust Port Cycles 216-225

Cycles 216-225 are complete.

## Cycle Results

216. Binary writer framework: added byte-preserving `NFile::from_bytes`, framed
binary section writing, and primitive little-endian payload helpers.

217. Binary land write: Rust writes `FIL_VER` and `FIL_LAN` payloads with a
stable byte-diff test.

218. Binary being write: Rust writes compact `FIL_BEI` payloads plus optional
social and episodic sections.

219. Binary empty/maximum write: empty, one-being, and `LARGE_SIM` fixtures load
back through the Rust binary reader.

220. Binary cross-load round trip: native text loads can be written as framed
binary and reloaded with matching state fields.

221. Binary byte diff gate: the land-only fixture is locked to an exact byte
sequence.

222. Binary CLI integration: `simape save` writes framed binary for `.bin`,
`.binary`, and `.nab`; `open` reads it through the shared loader.

223. Binary regression lock: added writer, reader, CLI, malformed, and population
edge coverage.

224. Binary compatibility report: documented supported versions, sections, and
known drift.

225. C build harness: added `scripts/build_native_simape.sh` to build native C
`simape`, Rust `simape`, and a compiler/version manifest.

## Validation

`cargo fmt --all --check`

`cargo test`

`bash scripts/build_native_simape.sh /private/tmp/apesdk_native_harness`

Result: formatting passed, full workspace tests passed with 79 `apesdk-sim`
tests, 27 `apesdk-toolkit` tests, and 45 `simape` tests, and the harness built
both native C and Rust `simape` binaries.

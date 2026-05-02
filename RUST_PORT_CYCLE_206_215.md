# Rust Port Cycles 206-215

Cycles 206-215 are complete.

## Cycle Results

206. Raw binary format audit: documented that active C command-line transfer is
native text and that the table describes compact section payloads.

207. Binary reader framework: added a magic-framed `NAB1` reader with little
endian section headers and truncation checks.

208. Binary version and land load: added version/signature validation and land
date/time/genetics import.

209. Binary being delta load: imported compact location, facing, speed, energy,
state, body, goal, and social coordinate fields.

210. Binary being constant load: imported birth date, generation range, father
name bytes, fetal genetics, and genetics words.

211. Binary events load: imported optional social and episodic sections and a
compact territory familiarity span.

212. Binary brain/volatile load: imported brain state, script overrides,
attention, registers, probes, drives, shout, inventory, preferences, conception,
and child-generation fields.

213. Binary immune load: imported the compact immune span available before the
brain-register offset.

214. Binary edge cases: covered missing magic, wrong order, wrong signature,
newer version, missing land, unknown sections, social-before-being, truncation,
and too many beings.

215. Legacy corpus setup: added the first save-format expectation table.

## Validation

`cargo fmt --all --check`

`cargo test -p apesdk-sim --lib binary`

`cargo test`

Result: full workspace suite passed with 75 `apesdk-sim` tests, 27
`apesdk-toolkit` tests, and 44 `simape` tests.

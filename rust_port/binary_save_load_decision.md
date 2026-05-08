# Binary Save/Load Decision

## Decision

Historical note: the Rust CLI remained JSON-only for save/load through cycle
85. The active command-line target is now native transfer text for both C and
Rust.

The C runtime has two transfer paths:

```text
tranfer_out_json
tranfer_out
tranfer_in
```

`tranfer_out_json` remains a compatibility path. The C native transfer path uses
the file-format table and section identifiers for land, topography/weather,
beings, social memory, episodic memory, and territory memory.

## Rationale

Binary support should not be bolted on until the Rust native structure coverage
is closer to complete. The binary loader would otherwise create a false parity
claim while braincode, territory, deeper social behavior, and some runtime
fields are still reduced.

## Current Behavior

Rust `save` writes native transfer text on the default command-line path. Rust
`open` reads native transfer text and preserves the C topography/weather
sections through semantic land state; JSON remains available for library
regression fixtures, not CLI parity.

Cycles 581-600 add populated native continuity coverage: Rust preserves
selected being name, awake state, social/episodic slot order, land payload,
immune, brain/probe, random state, and population count across a populated
native save/open trace. The raw native writer also emits `landd`, `topog`, and
`weath`, so the direct raw startup/reset oracle can compare byte streams rather
than only semantic values.

## Later Work

When binary parity is resumed, implement it against `universe/transfer.c` and
`universe/universe_internal.h`, with byte-for-byte fixtures for:

```text
FIL_VER
FIL_LAN
FIL_BEI
FIL_SOE
FIL_EPI
FIL_TOP
FIL_WEA
FILE_EOF
```

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
`open` reads native transfer text and accepts the C topography/weather sections;
JSON remains available for library regression fixtures, not CLI parity.

## Later Work

When binary parity is resumed, implement it against `universe/transfer.c` and
`universe/universe_internal.h`, with byte-for-byte fixtures for:

```text
FIL_VER
FIL_LAN
FIL_BEI
FIL_SOE
FIL_EPI
FILE_EOF
```

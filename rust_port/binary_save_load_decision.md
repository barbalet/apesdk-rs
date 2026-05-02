# Binary Save/Load Decision

## Decision

The Rust CLI remains JSON-only for save/load through cycle 85.

The C runtime has two transfer paths:

```text
tranfer_out_json
tranfer_out
tranfer_in
```

`tranfer_out_json` is the shape the Rust CLI currently writes and reads. The C
binary path uses the file-format table, section identifiers, and raw native
structure blocks for land, beings, social memory, and episodic memory.

## Rationale

Binary support should not be bolted on until the Rust native structure coverage
is closer to complete. The binary loader would otherwise create a false parity
claim while braincode, territory, deeper social behavior, and some runtime
fields are still reduced.

## Current Behavior

Rust `save` writes C-shaped JSON. Rust `open` accepts JSON whose signature is
`SIMULATED_APE_SIGNATURE` and rejects non-JSON/binary data through the existing
read failure path.

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


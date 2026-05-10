# Binary Compatibility Report

Cycles 216-225 added Rust write support for the framed binary fixture format
introduced in cycles 206-215.

## Supported

Rust now reads and writes `NAB1` framed binary startup transfers:

```text
magic: "NAB1"
section: kind u8, length u16 little-endian, payload bytes
```

Supported section payloads:

| Section | Kind | Read | Write | Notes |
| --- | ---: | --- | --- | --- |
| version | `0x10` | yes | yes | signature and version |
| land | `0x20` | yes | yes | date, time, land genetics |
| being | `0x30` | yes | yes | compact transfer-table payload |
| social | `0x40` | yes | yes | written for non-empty social entries |
| episodic | `0x50` | yes | yes | written for non-empty episodic events |

The command line now writes framed binary saves for `.bin`, `.binary`, and
`.nab` paths. JSON, `.native`, and `.ape` behavior is unchanged.

## Regression Gates

The Rust suite covers:

- exact byte output for a stable land-only binary fixture
- empty, single-being, and maximum-population binary write/load
- populated binary write/load with delta, constant, volatile, brain, social,
  episodic, compact territory, and compact immune fields
- native-text load followed by binary write and binary reload
- CLI binary save/open
- malformed binary reader rejection cases

## Known Drift

The active C command-line save path still emits tokenized native text through
`io_write_buff`; this repository does not yet have a C-produced raw binary
artifact to diff byte-for-byte.

The compact table span for `FIL_BEI` cannot preserve the full current
`simulated_immune_system` layout because `brreg=` begins at byte 451 while the
Rust/C immune struct is 52 bytes at current build settings. Rust writes and
loads the 45-byte compact immune span available in the transfer table.

The compact territory span is one byte per territory cell in the framed Rust
fixture. It preserves familiarity clamped to a byte, but does not preserve
territory names.

## Next Compatibility Work

Cycles 226+ should add C/Rust transcript runners, trace hooks, fixture manifests,
and C-produced save artifacts. True byte-for-byte parity should be claimed only
after C fixture capture confirms the exact native artifact shape.

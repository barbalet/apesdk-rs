# Raw Binary Transfer Audit

Cycles 206-215 checked the C transfer path and added a Rust binary fixture
reader around the compact section layout.

## C Path Reality

`universe/transfer.c::tranfer_out` calls `io_write_buff`, and
`toolkit/file.c::io_write_buff` emits tokenized text sections such as
`simul{...};`, `landd{...};`, and `being{...};`. `tranfer_in` strips
whitespace/comments, then `io_read_buff` parses the same tokenized sections
back into a temporary byte buffer.

That means the active command-line save/load path is native text, not a
standalone raw struct byte stream. The file-format table still exposes compact
section identifiers and offsets:

| Section | Kind | Active In C Build | Rust Binary Fixture |
| --- | ---: | --- | --- |
| `FIL_VER` | `0x10` | yes | version payload |
| `FIL_LAN` | `0x20` | declared, C write path TODO | land payload |
| `FIL_BEI` | `0x30` | yes | compact being payload |
| `FIL_SOE` | `0x40` | disabled by default | optional social payload |
| `FIL_EPI` | `0x50` | disabled by default | optional episodic payload |

The table offsets are compact transfer offsets, not current full Rust/C struct
offsets. For example, the Rust C-shaped `simulated_being` is 4,756 bytes and
`simulated_immune_system` is 52 bytes, while the compact table places immune
data at byte 406 and brain registers at byte 451.

## Rust Fixture Format

Rust now supports a deliberately framed fixture format so random raw bytes are
not mistaken for a C command-line save:

```text
magic: "NAB1"
repeat:
  kind: u8
  length: u16 little-endian
  payload: length bytes
```

Payloads use little-endian primitive values and the compact transfer-table
field order:

| Payload | Bytes | Notes |
| --- | ---: | --- |
| version | 4 | signature `0x4e41`, version |
| land | 10 | date, time, two land genetics values |
| being | 550 | compact `FIL_BEI` through brain probes |
| social | 154 | compact `FIL_SOE` plus braincode |
| episodic | 24 | compact `FIL_EPI` |

Territory is currently decoded as a 256-byte familiarity map from the compact
`FIL_BEI` territory span. The C table is ambiguous here because the current
C-shaped territory struct is larger than that span.

Immune data is decoded from the 45-byte compact span before the brain register
offset. That covers antigens, antigen shapes, antibodies, and the first 13
shape-antibody bytes. Full 52-byte immune seed parity belongs to the writer and
C fixture comparison cycles.

## Rejection Rules

The Rust binary reader rejects missing magic, missing first version section,
wrong signature, newer versions, missing land, unknown section kinds, social or
episodic sections before a being, truncated headers/payloads, and populations
larger than `LARGE_SIM`.

## Follow-On Work

Cycles 216-265 still need binary writer support, fixture corpus capture,
CLI wiring, byte diffs, and long transcript hardening before this can be called
byte-for-byte C parity.

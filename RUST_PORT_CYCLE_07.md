# Rust Port Cycle 07: File And Object JSON Foundation

## Objective

Port the first file/object serialization foundation from the C toolkit into
safe Rust so later cycles can reproduce `tranfer_out_json` and the captured
save/open command-line behavior.

## Changed Files

```text
crates/apesdk-toolkit/src/lib.rs
RUST_PORT_CYCLE_07.md
```

## Implemented

File buffer behavior:

```text
NFile
io_file_new
io_file_new_from_string_block
io_file_new_from_string
io_file_duplicate
io_file_hash
NFile::write
NFile::write_byte
NFile::reused
```

Object/array construction:

```text
ObjectType
ObjectValue
ObjectEntry
array_number
array_boolean
array_string
array_object
array_array
array_add
object_number
object_boolean
object_string
object_object
object_array
```

JSON output:

```text
unknown_json
object_top_object
```

## Compatibility Notes

`NFile::new` matches the C `io_file_new` allocation rule: 4096 bytes, zeroed
data, and location 0.

`NFile::from_string` matches the C rule that input string buffers allocate
`string_length * 2`, copy the input bytes, and leave `location` at 0.

`NFile::write_byte` keeps the C growth trigger: if `location + 1 == size`, the
buffer grows before writing. For small files the buffer grows by 4x.

`io_file_hash` hashes the native-endian `location`, native-endian `size`, and
the whole allocated data buffer, matching the C `n_file` hash behavior.

The object model is safe Rust (`Vec<ObjectEntry>` and `Vec<ObjectValue>`) rather
than a raw clone of the C linked-list/pointer representation. The JSON writer
preserves the compact C output shape: no whitespace, object insertion order,
quoted names, raw string contents, and comma separators.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
30 tests passed
```

The new tests cover:

```text
NFile constructor sizes and locations
NFile dynamic growth and reuse
NFile duplicate and exact hash fixture
compact object/array JSON output
top-level unknown_json object/array behavior
object name hashing
```

## Next Cycle

Cycle 08 should start translating the simulator startup JSON path from
`universe/transfer.c`:

```text
transfer_land_out_json equivalent
transfer_version_out_json equivalent
minimal startup transfer object
comparison against the `state_save_load` golden JSON shape
```

That will be the first bridge from generic toolkit serialization into
simulation-specific output.

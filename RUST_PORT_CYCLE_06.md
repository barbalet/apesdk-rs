# Rust Port Cycle 06: Toolkit Hash, Random, Root, And Number Parsing

## Objective

Continue the toolkit behavior port by moving the C hash/random/math helpers and
numeric string helpers into Rust with tests pinned to current C outputs.

## Changed Files

```text
crates/apesdk-toolkit/src/lib.rs
RUST_PORT_CYCLE_06.md
```

## Implemented

Hash helpers:

```text
math_hash_fnv1
math_hash
```

Random helpers:

```text
math_random
math_random3
```

Math helpers:

```text
math_root
math_tan
```

Number/string helpers:

```text
io_number_bytes
io_number_to_string
io_string_number
```

## Compatibility Notes

`math_hash` follows the 64-bit `n_uint` branch used by the current macOS
command-line build. The Rust implementation also keeps the 32-bit fallback
shape for later portability.

`math_random3` intentionally performs exactly three calls to `math_random`,
matching the C implementation.

`io_number_bytes` preserves the C parser model: decimal input is stored as an
integer plus a decimal-divisor count. For example, `-12.34` becomes:

```text
characters_read = 4
actual_value = -1234
decimal_divisor = 2
```

The parser keeps the historical edge case where `"-"` parses as zero with zero
characters read.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
25 tests passed
```

The new tests pin reference outputs for:

```text
FNV-1 hashes
ApeSDK block hashes
random seed transitions
integer square roots
cardinal tangent directions
integer and decimal io_number parsing
io_number error cases
number-to-string output
```

## Next Cycle

Cycle 07 should port the first file/object serialization foundation:

```text
n_file-safe wrapper
io_file_new_from_string
io_file_duplicate
io_file_hash
object/array construction basics
unknown_json output path
```

This prepares the Rust port for reproducing `tranfer_out_json` and the current
save/open behavior captured by the golden CLI smoke transcript.

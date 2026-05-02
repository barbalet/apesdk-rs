# Rust Port Cycle 05: First Toolkit Behavior

## Objective

Port the first executable toolkit behavior from C into Rust: memory helpers,
byte-string helpers, and basic vector math used throughout the simulator.

## Changed Files

```text
crates/apesdk-toolkit/src/lib.rs
RUST_PORT_CYCLE_05.md
```

## Implemented

Memory behavior:

```text
memory_copy
memory_erase
MemoryList::new
MemoryList::copy_units
```

Byte-string behavior:

```text
io_lower_bytes
io_length_bytes
io_find_bytes
io_string_write_bytes
```

Vector and sine behavior:

```text
area2_add
vect2_byte2
vect2_back_byte2
vect2_add
vect2_center
vect2_subtract
vect2_divide
vect2_multiplier
vect2_d
vect2_dot
vect2_distance_under
math_sine
vect2_direction
vect2_rotate90
vect2_delta
vect2_offset
vect2_copy
vect2_populate
vect2_rotation
vect2_rotation_bitshift
vect2_nonzero
vect2_min_max
```

## Compatibility Notes

`io_find_bytes` preserves the current C behavior of returning the index just
after a matched command. This matters for console parsing because `io_console`
checks the byte at that returned index for either a space or NUL.

`io_length_bytes` preserves the current C return behavior:

```text
null-equivalent empty safe input: 0 through the safe Rust API
max < 1: -1
zero byte before max: index of zero byte
no zero byte before max: max
```

`MemoryList` is a safe Rust wrapper rather than a raw-pointer FFI surface. It
keeps the C growth rule that a list doubles after reaching capacity.

The vector functions use value/reference-oriented Rust signatures instead of
raw pointers. Later FFI or exact-layout transfer work can wrap these helpers if
raw C ABI compatibility is needed.

## Validation

Run:

```sh
cargo fmt --all --check
cargo test
```

Current result:

```text
19 tests passed
```

## Next Cycle

Cycle 06 should continue the toolkit port with hash/random/math helpers:

```text
math_hash_fnv1
math_hash
math_root
math_random
math_random3
io_number
io_number_to_string
```

Those functions should get tests against the current C behavior before moving
up into file/object serialization.

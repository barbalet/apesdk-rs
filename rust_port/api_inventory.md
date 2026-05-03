# Rust Port API Inventory

This inventory maps the C command-line ApeSDK surface into the Rust port order.
It is based on the current trimmed build set documented in
cycle 01 of `RUST_PORT_CYCLES.md`.

## Header Surface

The command-line build exposes these public headers:

```text
toolkit/toolkit.h
script/script.h
render/graph.h
render/glrender.h
sim/sim.h
entity/entity.h
entity/entity_internal.h
universe/universe.h
universe/universe_internal.h
```

The first Rust crates should follow the existing dependency direction:

```text
toolkit -> script
toolkit -> render
toolkit -> sim
toolkit + sim + universe -> entity
toolkit + sim + entity + script -> universe
universe -> simape binary
```

There is a circular C include relationship between `entity/entity.h` and
`universe/universe.h`. Rust should break that with a shared `universe_types` or
`model` module containing `simulated_being`, `simulated_group`, and related
state structs.

## Primitive Type Policy

Rust should use explicit aliases at the compatibility boundary:

```text
n_double          f64
n_char            std::os::raw::c_char
n_string          *mut c_char at FFI edges, Rust String/CString internally
n_constant_string *const c_char at FFI edges, &'static str internally
n_string_block    [c_char; 4096]
n_byte            u8
n_byte2           u16
n_byte4           u32
n_c_int           std::os::raw::c_int
n_int             std::os::raw::c_long
n_uint            std::os::raw::c_ulong
n_audio           i16
```

`n_int` and `n_uint` are `long` and `unsigned long` on the current non-Windows
target. Rust code that is intended to match C layout should not silently replace
them with `i32` or `u32`. File-format fields that are explicitly `n_byte`,
`n_byte2`, or `n_byte4` should remain fixed-width Rust integers.

## Layout-Critical Types

These types should be modeled with `#[repr(C)]` or an explicitly documented
Rust-only replacement before behavior is ported:

```text
toolkit:
  n_vect2, n_area2, n_quad, n_line, n_vect3, n_rgba, n_rgba32
  n_points, n_spacetime, simulated_file_entry
  memory_list, int_list, number_array, number_array_list
  n_file, n_file_chain
  n_array, n_object
  simulated_console_command

sim:
  n_version
  n_tile, n_land, n_tile_coordinates

universe:
  simulated_file_definition
  simulated_feature, simulated_featureset
  simulated_isocial, simulated_iepisodic, simulated_iplace
  simulated_ibrain_probe
  simulated_immune_system
  simulated_idead_body, simulated_remains
  simulated_being_constant, simulated_being_delta
  simulated_being_events, simulated_being_volatile
  simulated_being_brain, simulated_being
  simulated_group, simulated_timing

entity:
  being_nearest
  being_listen_struct
  drives_sociability_data
  being_remove_loop2_struct

script:
  n_ae_error, n_brace, n_interpret, n_individual_interpret
```

The C code uses unions for vector and color conveniences. Rust can represent
the stable memory layout with `#[repr(C)]` structs and provide indexed helper
methods instead of exposing unions everywhere.

## Constants And Feature Flags

The Rust command-line port must start with the same default feature state:

```text
COMMAND_LINE_EXPLICIT is passed by build.sh by default
CONSOLE_ONLY is defined by longterm.c
CONSOLE_REQUIRED is defined by longterm.c
APESCRIPT_INCLUDED is undefined
EPISODIC_ON is defined
TERRITORY_ON is defined
BRAINCODE_ON is defined
IMMUNE_ON is defined
FEATURE_SET is defined
BRAIN_ON is undefined in the default command-line build
SIMULATED_PLANET is undefined
```

Important runtime constants:

```text
SHORT_VERSION_NAME = "Simulated Ape 0.708 "
VERSION_NUMBER = 708
SIMULATED_APE_SIGNATURE = ('N' << 8) | 'A'
MAP_BITS = 9
MAP_DIMENSION = 512
MAP_AREA = 262144
LARGE_SIM = 256
BRAINCODE_SIZE = 128
BRAINCODE_PROBES = 16
STRING_BLOCK_SIZE = 4096
```

`FULL_DATE` is currently C `__DATE__`, so transcript diffs that include the
banner are compile-date sensitive.

## Public API Groups

### Toolkit

Port first because every other folder depends on it.

Required groups:

```text
memory:
  memory_erase, memory_copy, memory_new, memory_free, memory_new_range
  memory_list_new, memory_list_copy, memory_list_free
  int_list_new, int_list_copy, int_list_find, int_list_free

vectors:
  vect2_* helpers
  vect3_* helpers

math:
  math_hash_fnv1, math_hash, math_root, math_tan
  math_random, math_random3
  math_sine, math_join*, math_line*, math_do_intersect

files and strings:
  io_file_new*, io_file_free, io_file_duplicate
  io_disk_read, io_disk_write, io_disk_check
  io_read*, io_write*, io_command, io_find, io_length
  io_string_write, io_string_copy, io_three_string_combination
  io_int_to_bytes, io_bytes_to_int

object/json:
  array_*, object_*, unknown_file_to_tree, unknown_json, unknown_free
  obj_contains*, obj_get*, obj_array*

console:
  io_console, io_help, io_quit
  io_console_entry, io_console_entry_clean, io_console_out
  io_command_line_execution_set, io_command_line_execution

audio:
  audio_* helpers used by speak/alpha commands
```

### Sim

Port after toolkit. It owns version constants, map dimensions, land/weather
state, and time helpers.

Required groups:

```text
weather_init, weather_cycle, weather_wind, weather_pressure
weather_seven_values, weather_lightning_test, weather_set_lightning
land_seed_genetics, land_init, land_clear, land_cycle
land_vect2, land_operator_interpolated, land_location*
land_date, land_time, land_genetics, land_tide_level
tile_*, tiles_*
spacetime_*
```

### Universe

This is the main command-line integration layer.

Required lifecycle and serialization APIs:

```text
sim_init
sim_cycle
sim_update_output
sim_close
sim_group
sim_timing
sim_memory_allocated
tranfer_out
tranfer_out_json
tranfer_in
sim_console
sim_set_console_input
sim_set_console_output
sim_output_string
```

Required command APIs are the functions behind `control_commands`, including:

```text
command_reset, command_open, command_script, command_save
command_quit, command_stop
command_run, command_step, command_interval
command_simulation, command_memory, command_debug
command_list, command_next, command_previous
command_being, command_watch
command_top, command_epic
command_file, command_event, command_logging
command_genome, command_stats, command_appearance
command_social_graph, command_pathogen_graph
command_braincode, command_speech, command_episodic, command_probes
command_speak, command_alphabet
```

The command matcher in C uses `io_find`, not an anchored exact match. Rust should
either preserve this behavior for compatibility or normalize it deliberately and
update the golden transcripts.

### Entity

Entity is the largest behavior surface. Port after the core state structs are
available.

First entity APIs to preserve for command-line behavior:

```text
being_init, being_init_group, being_erase, being_remove
being_name_simple, being_name_byte2, being_from_name, being_find_name
being_location*, being_space, being_high_res
being_energy, being_dead, being_energy_delta
being_state*, being_awake, being_female, being_pregnant
being_genetics, being_fetal_genetics, being_genetic_comparison
being_cycle_awake, being_cycle_universal, being_move
drives_cycle, drives_hunger, drives_fatigue
social_*, episodic_*, immune_*, food_eat
brain_sentence, brain_three_byte_command, brain_dialogue
speak_out
```

### Script And Render

`script/*.c` and `render/*.c` are compiled by `build.sh`, but the default
command-line feature set has `APESCRIPT_INCLUDED` undefined. Port enough of both
to keep linkage and command behavior stable, then deepen behavior coverage once
the core simulator is running.

Render starts with the small `graph_*` API used by skeleton/entity code.

Script starts with parser/interpreter data types and error reporting, then the
`command_script` behavior can be tested against golden sessions.

## Initial Rust Crate Shape

Recommended crate/module split:

```text
apesdk-toolkit
apesdk-script
apesdk-render
apesdk-sim
apesdk-model      shared universe/entity state structs
apesdk-entity
apesdk-universe
simape            binary
```

This keeps the Rust module graph acyclic while still reflecting the C source
folders.

## Port Order For Next Implementation Cycles

1. Create Rust aliases/constants for primitive and fixed-size types.
2. Port `n_file`, string helpers, memory helpers, and vector helpers.
3. Port math/hash/random behavior and add C/Rust comparison tests.
4. Port object/json helpers used by `tranfer_out_json`.
5. Port `n_land`, map/time constants, and land startup state.
6. Port universe/entity state structs with layout tests.
7. Port transfer output/input enough to reproduce current save/open behavior.
8. Port console command registration and command matching.
9. Port the minimal `sim_console` run loop and `simape` binary.
10. Expand entity behavior until `run`, `step`, `list`, and watch commands match.

## Compatibility Risks

- `n_int`/`n_uint` are pointer-width C long types on this target.
- The banner contains `__DATE__`, which changes when C is rebuilt.
- Error text includes C source paths and line numbers.
- POSIX console behavior is threaded.
- Save via `tranfer_out_json` currently creates JSON that `command_open` fails
  to read as a native transfer file in the captured startup smoke transcript.
- `APESCRIPT_INCLUDED` is undefined, even though `script/*.c` is compiled.

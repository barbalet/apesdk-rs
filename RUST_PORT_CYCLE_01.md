# Rust Port Cycle 01: C Command-Line Baseline

## Objective

Capture the current C command-line build as the compatibility target for the
Rust port. The Rust command-line version must keep the same build entry point
and produce a `simape` binary with matching console commands and behavior.

## Verified Build Command

```sh
bash build.sh
```

The build script writes the binary to:

```text
../simape
```

The build was verified after the tree was reduced to the command-line source
set. The sandboxed link step cannot write outside this workspace, but the same
command succeeds when allowed to write `../simape`.

## Build Modes

`build.sh` accepts these modes:

- default: `CFLAGS=-O2`, `COMMANDLINEE=-DCOMMAND_LINE_EXPLICIT`
- `--debug`: `CFLAGS=-g`, `COMMANDLINEE=-DCOMMAND_LINE_EXPLICIT`
- `--additional`: `CFLAGS=-O2`, `COMMANDLINEE=-DNOTHING_NEEDED_HERE`

All modes link with:

```text
-lz -lm -lpthread
```

## Required Source Folders

The command-line build compiles top-level `.c` files from exactly these folders:

- `toolkit`
- `script`
- `render`
- `sim`
- `entity`
- `universe`

The command-line source root is:

- `longterm.c`

## Compile Units

```text
toolkit/object.c
toolkit/memory.c
toolkit/math.c
toolkit/io.c
toolkit/vect.c
toolkit/file.c
script/interpret.c
script/parse.c
render/glrender.c
render/graph.c
sim/land.c
sim/console.c
sim/tile.c
sim/spacetime.c
sim/territory.c
sim/audio.c
entity/skeleton.c
entity/brain.c
entity/drives.c
entity/speak.c
entity/social.c
entity/immune.c
entity/being.c
entity/episodic.c
entity/food.c
entity/body.c
universe/sim.c
universe/loop.c
universe/command.c
universe/transfer.c
longterm.c
```

## Public Headers In The Build Set

```text
toolkit/toolkit.h
script/script.h
render/glrender.h
render/graph.h
sim/sim.h
entity/entity.h
entity/entity_internal.h
universe/universe.h
universe/universe_internal.h
```

## Startup Contract

`longterm.c` defines:

```c
#define CONSOLE_ONLY
#define CONSOLE_REQUIRED
#undef AUDIT_FILE
```

The runtime entry path is:

```text
main
command_line_run
sim_console
```

`command_line_run` sets:

```text
simulation_filename = "realtime.txt"
srand((unsigned int)time(NULL))
sim_console(simulation_filename, rand())
```

`sim_console` prints the console banner, sets command-line execution mode,
starts the simulation with `sim_init(KIND_START_UP, randomise, MAP_AREA, 0)`,
attempts to load `realtime.txt` if it exists, then runs the console loop.

## Command Compatibility Target

The default build defines `COMMAND_LINE_EXPLICIT`, so the Rust port must support
these command names and aliases:

```text
help
reset
clear
open
load
script
save
quit
exit
close
stop
speak
alpha
file
run
step
top
epic
interval
event
logging
log
simulation
sim
watch
monitor
idea
ape
pwd
pathogen
friends
social
socialgraph
graph
braincode
speech
episodic
probes
stats
status
appearance
physical
genome
genetics
list
ls
dir
next
previous
prev
debug
memory
```

The `--additional` build mode omits the `COMMAND_LINE_EXPLICIT` commands:

```text
reset
clear
open
load
```

## Next Cycle

Cycle 02 should create golden command-line transcripts by driving the current C
`../simape` binary with representative command sessions and saving the expected
stdout/stderr behavior for the Rust implementation.

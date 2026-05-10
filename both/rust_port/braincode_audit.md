# Braincode Audit

## C Surface

Native braincode is defined across:

```text
sim/sim.h
entity/brain.c
entity/entity.h
universe/universe_internal.h
```

The instruction stream is 128 bytes. Each instruction is three bytes:

```text
byte 0: opcode in low 6 bits, constant flags in bits 64 and 128
byte 1: argument/value 0
byte 2: argument/value 1
```

Addresses wrap across `BRAINCODE_MAX_ADDRESS = BRAINCODE_SIZE * 2`, allowing
instructions to address local and peer braincode memory. The C VM runs
arithmetic/data/control instructions through `math_general_execution`; sensor
and actuator instructions dispatch to terrain, social, episodic, body, and
preference hooks.

## Rust Coverage Added In Cycles 133-135

Rust now defines the native opcode constants, constant-bit flags, decoder, and
a first `BraincodeVm` scaffold. The VM supports decode, register state, local
and remote address space, and the initial data/arithmetic/control operators:

```text
DAT0 DAT1
ADD SUB MUL DIV MOD
MVB MOV JMP CTR SWP INV STP LTP
JMZ JMN DJN AND OR SEQ SNE SLT
```

Sensors and actuators are decoded but intentionally no-op until the next
cycles wire them into body, terrain, social, episodic, and preference state.

## Cycle 136-144 Update

Rust now executes the sensor and actuator opcode bands through `BraincodeIo`.
The per-minute being cycle runs internal braincode from the self social entry,
nearby social interactions run paired braincode dialogue, and probe frequency,
address, offset, position, state, learned preference, shout, posture, goal,
anecdote, and intention hooks are represented in executable state.

The implementation intentionally keeps the VM deterministic and fixtureable:
sensor values are collected from the current being, met being, social entry,
episodic focus, territory focus, immune state, drives, body inventory, land
time, and weather pressure before each dialogue run; actuator effects are then
applied back onto the same C-shaped being fields.

## Remaining Work

The C fixture harness still needs native traces for the full `brain_dialogue`
surface, especially the nuanced attention-similarity searches and externally
observed social action side effects. The Rust VM has executable coverage for
the opcodes and hooks, but byte-for-byte braincode trace parity is not yet
proven against generated C traces.

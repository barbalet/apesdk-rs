# Native Engine Stabilization Notes

## Cycle 85 Status

The Rust command-line simulator now advances populated beings through
native-shaped universal, immune, drive, movement, body, social-memory, and
episodic-memory paths. JSON save/load carries substantially more native being
state than the earlier summary port.

## Covered In Rust

```text
awake/asleep/dead state
energy clamp and movement cost
immune arrays and response
drive cycling
facing, speed, velocity history, movement
body height/mass/fat/frame
social memory entries and social-coordinate maintenance
reduced grooming/squabble/mate-seeking hooks
episodic storage/fading/output
expanded JSON transfer fields
runtime transcript coverage
```

## Remaining Gaps

```text
full terrain slope/water movement cost
full food terrain lookup and pathogen food ingestion
complete social attraction/prejudice/stereotype calculations
complete grooming/squabble/mating probabilities
braincode execution and probes as active behavior
territory memory evolution and output
event logging callbacks matching C watch/event modes
binary transfer_in/tranfer_out parity
long-duration C-vs-Rust transcript diffs
```

## Recommended Next Tranche

Start with terrain/food parity and social behavior depth before binary transfer.
Those paths decide the runtime state that binary save/load would need to prove.


# Final Parity Closure Register

Cycles 266-300 retire the prior documented-difference model.

## Status

No difference is treated as complete by documentation alone. Each parity
category is now one of:

- exact under the absolute gate
- represented by an open fixture task
- outside the default command-line path reachable from `longterm.c`

## Closed Items

- Default command-line save/open drift: Rust `save` now writes JSON for every
  extension like native C, and Rust `open` now accepts native transfer text
  while rejecting JSON/framed binary input on the default CLI path.
- Build metadata drift: native C and Rust harness builds share the
  `APESDK_FULL_DATE` value.
- Source-path drift: the native harness compiles C sources with `./folder/file.c`
  paths so error locations match Rust's command output.
- Transcript normalization drift: the absolute transcript gate uses only
  transport cleanup.

## Open Fixture Tasks

- Promote direct C state trace emitters for deeper terrain, braincode, social,
  lifecycle, immune, movement, and save/load categories.
- Promote raw native binary artifacts only after a C oracle target generates
  reachable command-line byte fixtures.

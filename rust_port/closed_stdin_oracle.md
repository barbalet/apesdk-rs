# Closed Stdin Oracle

Cycles 366-370 promote closed-stdin behavior out of triage.

Native C console output around redirected EOF is produced by a threaded console
path. Depending on process teardown timing, the raw transcript can contain no
bytes, one console-failure line after the banner, or two console-failure lines
after the banner. The promoted oracle is the native output class, canonicalized
to:

```text

 *** Simulated Ape 0.708 Console, May  1 2026 ***
      For a list of commands type 'help'

ERROR: Console failure @ ./sim/console.c 220
ERROR: Console failure @ ./sim/console.c 220
```

`scripts/run_closed_stdin_oracle.sh` records raw native/Rust classes across
repeated runs, canonicalizes both sides with
`scripts/canonicalize_closed_stdin_transcript.sh`, and diffs the canonical
transcripts. `scripts/run_interactive_timing_gate.sh` uses the same canonical
oracle for closed stdin and quiet EOF.

Bounded-run EOF can reorder `Running for 1 mins` and the console-failure line
in native C. `scripts/canonicalize_eof_bounded_run_transcript.sh` accepts only
that native output class and canonicalizes it to banner, running line, then
console failure before diffing C/Rust output.

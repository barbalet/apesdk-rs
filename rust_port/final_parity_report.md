# Final Parity Report

Cycles 266-300 extend the earlier approved-corpus closure into the strict
native command-line parity path.

## Default CLI Parity

- Native C `save` now writes native transfer text through `tranfer_out()`, and
  native C `open` reads it back through `tranfer_in()`.
- Rust `open` already reads native transfer text on the default command-line
  path, but Rust `save` still needs to move from the older JSON-compatible
  behavior to the completed C transfer target.
- JSON and framed-binary compatibility remain library-level regression support,
  not default CLI behavior.

## Absolute Gates

- `scripts/run_raw_transcript_diff.sh` compares native C and Rust transcripts
  with only CRLF transport cleanup.
- `scripts/run_absolute_parity_ci.sh` runs formatting, tests, strict transcript
  smoke diffing, and trace diff smoke.
- `scripts/generate_c_oracle_artifacts.sh` captures native C oracle transcripts
  for review.

## Last Strict Scope

The promoted exact transcript corpus currently includes `help`, `help_errors`,
and `command_edges`. Deeper engine categories require direct C trace promotion
before they can be signed off under the same no-normalization rule.

## Signoff Rule

The project no longer accepts documented drift as a completion condition.
Outstanding parity work must be represented as fixture tasks and closed with
native C oracle output.

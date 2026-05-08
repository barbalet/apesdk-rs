# Final Parity Report

Cycles 266-300 extend the earlier approved-corpus closure into the strict
native command-line parity path.

## Default CLI Parity

- Native C `save` now writes native transfer text through `tranfer_out()`, and
  native C `open` reads it back through `tranfer_in()`.
- Rust `save` and `open` now use native transfer text on the default
  command-line path. The Rust reader applies and re-emits the C
  topography/weather sections that are now written by `tranfer_out()`.
- Populated Rust native save/open now preserves selected state internally,
  including selected identity, awake state, being name, land payload, social and
  episodic slot order, immune, brain/probe, random state, and population count.
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
before they can be signed off under the same no-normalization rule. Current
open runtime categories are selected after-day movement/body/energy/honor,
brain/social/episodic/immune ordering, C/Rust save/open post-load advancement,
broader populated raw artifact bytes, and exact corpus promotion.

Cycle 601-630 update: the broader raw fixture set is now generated and
value-gated for after-one-cycle, social-heavy, immune-heavy, terrain-heavy, and
save/open-derived artifacts, with populated byte-exact promotion still open.
After-day drift is now bucketed: selection and body are exact, while movement,
energy, honor/drives, brain/probe, social, episodic, and immune remain
inventory-only.

Cycle 631-660 update: day-one runtime drift now has first-mismatch buckets:
honor/drives first diverges at minute 120, movement/energy/body at minute 180,
brain/probe and social at minute 60, immune at minute 180, and episodic at
minute 660. Rust save/open internal continuity is exact through one post-load
minute but still diverges after one post-load day.

## Signoff Rule

The project no longer accepts documented drift as a completion condition.
Outstanding parity work must be represented as fixture tasks and closed with
native C oracle output.

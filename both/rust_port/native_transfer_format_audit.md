# Native Transfer Format Audit

## C Surface

The command-line C transfer path is implemented in `universe/transfer.c` and
the `simulated_file_format` table in `universe/universe_internal.h`.

Despite earlier shorthand calling this "binary", the active command-line path
writes a compact native text section format:

```text
simul{signa=20033;verio=708;};
landd{dated=...;timed=...;landg=...,...;};
being{locat=...,...;facin=...;...;};
```

`tranfer_in` strips comments and whitespace with `io_whitespace`, requires
`FIL_VER` first, then accepts `FIL_LAN`, topography, weather, `FIL_BEI`,
social, episodic, and territory sections. In this checkout `USE_FIL_VER`,
`USE_FIL_LAN`, `USE_FIL_TOP`, `USE_FIL_WEA`, `USE_FIL_BEI`, `USE_FIL_SOE`,
`USE_FIL_EPI`, and `USE_FIL_TER` are enabled.

The C CLI `save` path now uses `tranfer_out()` instead of the JSON-only writer,
so native command-line save/open has a real round trip for startup and populated
sessions. The topography section writes both `MAP_AREA` byte buffers, and the
weather section writes both raw atmosphere buffers plus the lightning byte map.
The high-definition topography/tide buffers remain generated from the loaded
map and time.

## Rust Coverage Added In Cycles 121-125

Rust now has a native section parser that:

```text
removes C block comments and whitespace
parses six-character section and field tokens
validates SIMULATED_APE_SIGNATURE and VERSION_NUMBER
loads land date/time/genetics from landd
loads first-pass being delta/constant/change/brain fields from being
rejects raw byte-structured data and unsupported sections explicitly
```

The CLI `open` and `load` paths try JSON first and then the native transfer
format, so existing JSON behavior remains unchanged.

## Cycle 126-132 Update

Rust now also loads `sgcia` social sections, `episo` episodic sections,
braincode registers/probes, and immune bytes from native `being` sections.
It can write native transfer text through `tranfer_startup_out_native`.

## Cycle 145-160 Update

Rust now carries territory memory in the C-shaped being state and round-trips
non-empty territory entries through native transfer text as `terri` sections.
JSON transfer also includes the complete territory array under
`events.territory`, preserving the existing structured transfer path.

## Current Update

Rust CLI `save` now uses `tranfer_startup_out_native()` for every filename, so
the default command-line path no longer writes the older JSON transfer. Rust
`open` now accepts, validates, applies, and re-emits C `topog{}` and `weath{}`
sections, so command-line saves preserve loaded terrain/weather bytes across
Rust save-open-save loops.

Cycles 581-600 extend the populated transfer layer. C and Rust native transfer
now preserve selected being name and awake state with `bname=` and `awako=`,
Rust writes every social and episodic slot to keep C slot ordering stable, and
the raw native Rust writer emits `landd`, `topog`, and `weath` sections. The
Rust `--save-open-trace` fixture proves populated Rust native save/open state
continuity is exact for the selected snapshot before save and immediately after
open.

## Remaining Work

Next binary/native cycles should close generated-value parity for
high-definition topography/tide behavior, older version migration behavior, raw
byte-structured compatibility if required by external save corpora, C/Rust
post-open advancement parity, and deeper C-vs-Rust fixture generation. Raw
byte-structured compatibility remains unsupported because the C command-line
writer currently emits the native text format, not a raw `NA` byte stream.

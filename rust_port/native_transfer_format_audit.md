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
`FIL_VER` first, then accepts `FIL_LAN`, `FIL_BEI`, and compiled-in optional
sections. In this checkout `USE_FIL_VER`, `USE_FIL_LAN`, and `USE_FIL_BEI` are
enabled, while `USE_FIL_SOE` and `USE_FIL_EPI` are disabled.

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
It can write native transfer text through `tranfer_startup_out_native`, and
the CLI writes native transfer files when saving to filenames containing
`.native` or ending in `.ape`.

## Remaining Work

Next binary/native cycles should port territory bytes, weather sections, older
version migration behavior, and deeper C-vs-Rust fixture generation. Raw
byte-structured compatibility remains unsupported because the C command-line
writer currently emits the native text format, not a raw `NA` byte stream.

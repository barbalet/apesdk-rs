# Binary Save Corpus

Cycle 215 created the initial load expectation table for save formats used in
the Rust port.

| Corpus Entry | Source | Rust Support | Expected Result |
| --- | --- | --- | --- |
| Native text empty land | C/Rust `io_write_buff` style | yes | loads land |
| Native text one being | C/Rust `being{...};` sections | yes | loads population |
| JSON startup object | Rust transfer JSON | yes | loads land/population |
| Rust framed binary land | `NAB1` + `FIL_VER` + `FIL_LAN` | yes | loads land |
| Rust framed binary one being | `NAB1` + `FIL_BEI` fixture | yes | loads compact state |
| Rust framed binary events | `FIL_SOE`/`FIL_EPI` after being | yes | loads social/episodic |
| CLI framed binary open | `simape open` with `NAB1` land fixture | yes | restores land state |
| CLI framed binary save | `simape save *.bin` | yes | writes `NAB1`, reopens |
| Maximum framed population | Rust-generated `LARGE_SIM` fixture | yes | loads 256 beings |
| Malformed binary | bad magic/truncated/unknown section | yes | rejected |
| Raw C struct dump | not emitted by current CLI transfer path | no fixture yet | explicitly unsupported |

The current checked corpus is unit-test generated so the payloads remain
deterministic in the repository. Later cycles should replace or supplement this
with C-produced save fixtures and byte-level expectations.

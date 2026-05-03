#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/selected-minute-value-gate"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"
"$ROOT/scripts/generate_c_engine_trace_probe.sh" "$OUT_DIR/c" >/dev/null
"$ROOT/scripts/generate_rust_engine_trace_probe.sh" "$OUT_DIR/rust" >/dev/null

extract_minute60_core() {
    awk '
    function reset_fields(   key) {
        for (key in fields) {
            delete fields[key]
        }
    }

    function read_fields(   field, parts) {
        reset_fields()
        for (field = 2; field <= NF; field++) {
            split($field, parts, "=")
            fields[parts[1]] = parts[2]
        }
    }

    /^SELECTED-MINUTE / {
        read_fields()
        if (fields["minute"] == "60") {
            printf "selected-minute60-core label=%s minute=%s date=%s time=%s population=%s selected=%s energy=%s loc=%s facing=%s speed=%s honor=%s mass=%s awake=%s state=%s drives=%s episodic0=%s immune=%s preference=%s\n",
                fields["label"],
                fields["minute"],
                fields["date"],
                fields["time"],
                fields["population"],
                fields["selected"],
                fields["energy"],
                fields["loc"],
                fields["facing"],
                fields["speed"],
                fields["honor"],
                fields["mass"],
                fields["awake"],
                fields["state"],
                fields["drives"],
                fields["episodic0"],
                fields["immune"],
                fields["preference"]
        }
    }
    ' "$1"
}

extract_minute60_core "$OUT_DIR/c/c_engine_trace.trace" > "$OUT_DIR/c.minute60-core"
extract_minute60_core "$OUT_DIR/rust/rust_engine_trace.trace" > "$OUT_DIR/rust.minute60-core"
diff -u "$OUT_DIR/c.minute60-core" "$OUT_DIR/rust.minute60-core"

awk '
NR == 1 {
    sub(/energy=3852/, "energy=3853")
}
{ print }
' "$OUT_DIR/c.minute60-core" > "$OUT_DIR/c.minute60-core.mutated"

if diff -u "$OUT_DIR/c.minute60-core.mutated" "$OUT_DIR/rust.minute60-core" > "$OUT_DIR/mutated.diff"; then
    echo "selected minute60 mutation unexpectedly passed" >&2
    exit 1
fi

{
    printf 'c_trace=%s\n' "$OUT_DIR/c/c_engine_trace.trace"
    printf 'rust_trace=%s\n' "$OUT_DIR/rust/rust_engine_trace.trace"
    printf 'covered=minute60-selected-energy-location-facing-speed-honor-mass-awake-state-drives-episodic-immune-preference\n'
    printf 'negative_check=minute60-energy-mutation-fails\n'
    printf 'open=minute60-brain-social later-selected-minute-values after-day-selected-runtime-values\n'
} > "$OUT_DIR/selected_minute_value_gate_manifest.txt"

echo "selected-minute-value-gate=pass out=$OUT_DIR covered=minute60-selected-core"

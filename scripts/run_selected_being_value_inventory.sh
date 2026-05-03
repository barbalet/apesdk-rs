#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/selected-being-value-inventory"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"
"$ROOT/scripts/generate_c_engine_trace_probe.sh" "$OUT_DIR/c" >/dev/null
"$ROOT/scripts/generate_rust_engine_trace_probe.sh" "$OUT_DIR/rust" >/dev/null

extract_selected_values() {
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

    /^TRACE / {
        read_fields()
        printf "TRACE-SELECTED snapshot=%s population=%s selected=%s name=%s energy=%s loc=%s facing=%s velocity=%s honor=%s height=%s mass=%s posture=%s awake=%s drives=%s brain=%s probe0=%s social0=%s episodic0=%s territory0=%s conception=%s family=%s immune=%s inventory=%s shout=%s preference=%s\n",
            fields["snapshot"],
            fields["population"],
            fields["selected"],
            fields["name"],
            fields["energy"],
            fields["loc"],
            fields["facing"],
            fields["velocity"],
            fields["honor"],
            fields["height"],
            fields["mass"],
            fields["posture"],
            fields["awake"],
            fields["drives"],
            fields["brain"],
            fields["probe0"],
            fields["social0"],
            fields["episodic0"],
            fields["territory0"],
            fields["conception"],
            fields["family"],
            fields["immune"],
            fields["inventory"],
            fields["shout"],
            fields["preference"]
    }
    ' "$1"
}

extract_selected_values "$OUT_DIR/c/c_engine_trace.trace" > "$OUT_DIR/c.selected"
extract_selected_values "$OUT_DIR/rust/rust_engine_trace.trace" > "$OUT_DIR/rust.selected"
awk '$2 == "snapshot=after_cycle_1" { print }' "$OUT_DIR/c.selected" > "$OUT_DIR/c.first-cycle"
awk '$2 == "snapshot=after_cycle_1" { print }' "$OUT_DIR/rust.selected" > "$OUT_DIR/rust.first-cycle"

diff -u "$OUT_DIR/c.first-cycle" "$OUT_DIR/rust.first-cycle"

selected_status=exact
if ! diff -u "$OUT_DIR/c.selected" "$OUT_DIR/rust.selected" > "$OUT_DIR/selected.diff"; then
    selected_status=inventory
fi

selected_mismatches="$(awk '/^[+-]TRACE-SELECTED / { count++ } END { print count + 0 }' "$OUT_DIR/selected.diff")"
sample_count="$(wc -l < "$OUT_DIR/c.selected" | tr -d ' ')"

{
    printf 'c_trace=%s\n' "$OUT_DIR/c/c_engine_trace.trace"
    printf 'rust_trace=%s\n' "$OUT_DIR/rust/rust_engine_trace.trace"
    printf 'first_cycle_status=exact\n'
    printf 'selected_status=%s\n' "$selected_status"
    printf 'selected_samples=%s\n' "$sample_count"
    printf 'selected_mismatches=%s\n' "$selected_mismatches"
    printf 'covered=selected-being-startup-empty-and-first-cycle-native-init-body-drive-brain-probe-immune-values\n'
    printf 'open=after-day-movement-body-energy-honor-brain-social-episodic-immune-runtime-values\n'
} > "$OUT_DIR/selected_being_value_inventory_manifest.txt"

echo "selected-being-value-inventory=pass out=$OUT_DIR first_cycle_status=exact selected_status=$selected_status samples=$sample_count mismatches=$selected_mismatches"

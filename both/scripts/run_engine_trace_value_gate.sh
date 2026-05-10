#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/engine-trace-value-gate"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"
"$ROOT/both/scripts/generate_c_engine_trace_probe.sh" "$OUT_DIR/c" >/dev/null
"$ROOT/both/scripts/generate_rust_engine_trace_probe.sh" "$OUT_DIR/rust" >/dev/null

extract_values() {
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

    function emit(field) {
        printf " %s=%s", field, fields[field]
    }

    /^TRACE / {
        read_fields()
        if (fields["snapshot"] == "startup") {
            printf "startup"
            startup_count = split("date time genetics tide hightide population selected name energy loc facing velocity honor height mass posture awake drives brain probe0 social0 episodic0 territory0 conception family immune inventory shout preference", startup_fields, " ")
            for (i = 1; i <= startup_count; i++) {
                emit(startup_fields[i])
            }
            printf "\n"
        }

        printf "invariant snapshot=%s", fields["snapshot"]
        invariant_count = split("hightide velocity posture conception family inventory shout", invariant_fields, " ")
        for (i = 1; i <= invariant_count; i++) {
            emit(invariant_fields[i])
        }
        printf "\n"

        if (fields["snapshot"] == "after_cycle_1") {
            printf "first-cycle-baseline snapshot=%s", fields["snapshot"]
            first_cycle_count = split("social0 episodic0 territory0 preference", first_cycle_fields, " ")
            for (i = 1; i <= first_cycle_count; i++) {
                emit(first_cycle_fields[i])
            }
            printf "\n"

            printf "terrain-first-cycle snapshot=%s", fields["snapshot"]
            terrain_count = split("genetics tide topo high hightide weather pressure food", terrain_fields, " ")
            for (i = 1; i <= terrain_count; i++) {
                emit(terrain_fields[i])
            }
            printf "\n"

            printf "selected-first-cycle snapshot=%s", fields["snapshot"]
            selected_count = split("population selected name energy loc facing velocity honor height mass posture awake drives brain probe0 social0 episodic0 territory0 conception family immune inventory shout preference", selected_fields, " ")
            for (i = 1; i <= selected_count; i++) {
                emit(selected_fields[i])
            }
            printf "\n"
        }

        if (fields["snapshot"] != "startup") {
            printf "runtime-core snapshot=%s", fields["snapshot"]
            runtime_count = split("date time tide hightide population selected velocity height posture awake conception family inventory shout", runtime_fields, " ")
            for (i = 1; i <= runtime_count; i++) {
                emit(runtime_fields[i])
            }
            printf "\n"
        }
    }
    ' "$1"
}

extract_values "$OUT_DIR/c/c_engine_trace.trace" > "$OUT_DIR/c.values"
extract_values "$OUT_DIR/rust/rust_engine_trace.trace" > "$OUT_DIR/rust.values"
diff -u "$OUT_DIR/c.values" "$OUT_DIR/rust.values"

awk '
NR == 1 {
    sub(/date=0/, "date=1")
}
{ print }
' "$OUT_DIR/c.values" > "$OUT_DIR/c.values.mutated"

if diff -u "$OUT_DIR/c.values.mutated" "$OUT_DIR/rust.values" > "$OUT_DIR/mutated.diff"; then
    echo "engine trace value mutation unexpectedly passed" >&2
    exit 1
fi

{
    printf 'c_trace=%s\n' "$OUT_DIR/c/c_engine_trace.trace"
    printf 'rust_trace=%s\n' "$OUT_DIR/rust/rust_engine_trace.trace"
    printf 'covered=startup-value-subset invariant-zero-fields first-cycle-baseline runtime-core-fields terrain-first-cycle selected-first-cycle\n'
    printf 'negative_check=one-field-mutation-fails\n'
    printf 'open=multi-day-weather after-day-selected-being movement-body-energy honor brain social immune runtime-values\n'
} > "$OUT_DIR/engine_trace_value_gate_manifest.txt"

echo "engine-trace-value-gate=pass out=$OUT_DIR covered=startup-value-subset,invariant-zero-fields,first-cycle-baseline,runtime-core-fields,terrain-first-cycle,selected-first-cycle"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/immune-episodic-runtime-inventory"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"

TRACE_DIR="$OUT_DIR/traces"
"$ROOT/both/scripts/generate_c_engine_trace_probe.sh" "$TRACE_DIR/c" >/dev/null
"$ROOT/both/scripts/generate_rust_engine_trace_probe.sh" "$TRACE_DIR/rust" >/dev/null

C_MINUTE="$OUT_DIR/c.selected-minute"
RUST_MINUTE="$OUT_DIR/rust.selected-minute"
BUCKETS="$OUT_DIR/immune_episodic_runtime_buckets.txt"
MUTATED="$OUT_DIR/rust.selected-minute.mutated"

grep '^SELECTED-MINUTE ' "$TRACE_DIR/c/c_engine_trace.trace" > "$C_MINUTE"
grep '^SELECTED-MINUTE ' "$TRACE_DIR/rust/rust_engine_trace.trace" > "$RUST_MINUTE"

c_count="$(wc -l < "$C_MINUTE" | tr -d ' ')"
rust_count="$(wc -l < "$RUST_MINUTE" | tr -d ' ')"
if [ "$c_count" != "$rust_count" ]; then
    echo "immune/episodic selected-minute count mismatch: c=$c_count rust=$rust_count" >&2
    exit 1
fi

awk 'NR == 1 { sub(/immune=[^ ]+/, "immune=255:255:255:255:255:255") } { print }' "$RUST_MINUTE" > "$MUTATED"
if cmp -s "$RUST_MINUTE" "$MUTATED"; then
    echo "immune/episodic mutation fixture did not change" >&2
    exit 1
fi

awk -v c_minute="$C_MINUTE" -v rust_minute="$RUST_MINUTE" '
function reset(values,   key) {
    for (key in values) {
        delete values[key]
    }
}

function parse(line, values,   count, idx, parts, kv) {
    reset(values)
    count = split(line, parts, " ")
    for (idx = 2; idx <= count; idx++) {
        split(parts[idx], kv, "=")
        values[kv[1]] = kv[2]
    }
}

function diff_fields(fields,   count, idx, split_fields, field, diffs) {
    diffs = ""
    count = split(fields, split_fields, " ")
    for (idx = 1; idx <= count; idx++) {
        field = split_fields[idx]
        if (c[field] != rust[field]) {
            if (diffs != "") {
                diffs = diffs ","
            }
            diffs = diffs field ":" c[field] "->" rust[field]
        }
    }
    return diffs
}

function compare_bucket(name, fields,   diffs) {
    if (status[name] == "inventory") {
        return
    }
    diffs = diff_fields(fields)
    if (diffs != "") {
        status[name] = "inventory"
        first_minute[name] = c["minute"]
        first_diff[name] = diffs
    }
}

function capture_target(prefix, fields,   diffs) {
    diffs = diff_fields(fields)
    if (diffs == "") {
        target[prefix] = "exact"
    } else {
        target[prefix] = diffs
    }
}

function print_bucket(name) {
    print "bucket." name ".status=" status[name]
    if (first_minute[name] != "") {
        print "bucket." name ".first_minute=" first_minute[name]
        print "bucket." name ".first_minute_diff=" first_diff[name]
    }
    if (status[name] != "exact") {
        open_count++
        if (open_buckets != "") {
            open_buckets = open_buckets " "
        }
        open_buckets = open_buckets name
    }
}

BEGIN {
    bucket_count = 3
    bucket_names[1] = "immune_runtime"
    bucket_names[2] = "episodic_runtime"
    bucket_names[3] = "energy_random_coupling"
    fields["immune_runtime"] = "immune seed energy state terrain food"
    fields["episodic_runtime"] = "episodic0 attention social0 brainstate"
    fields["energy_random_coupling"] = "seed energy drives state immune"
    for (idx = 1; idx <= bucket_count; idx++) {
        status[bucket_names[idx]] = "exact"
    }

    while ((getline c_line < c_minute) > 0) {
        if ((getline rust_line < rust_minute) <= 0) {
            print "status=missing-rust-minute-row"
            exit 2
        }
        parse(c_line, c)
        parse(rust_line, rust)
        for (idx = 1; idx <= bucket_count; idx++) {
            name = bucket_names[idx]
            compare_bucket(name, fields[name])
        }
        if (c["minute"] == "180") {
            capture_target("minute180_immune", "immune seed energy state terrain food")
        }
        if (c["minute"] == "660") {
            capture_target("minute660_episodic", "episodic0 attention social0 brainstate")
        }
    }
    close(c_minute)
    close(rust_minute)

    open_count = 0
    open_buckets = ""
    print "c_selected_minute=" c_minute
    print "rust_selected_minute=" rust_minute
    for (idx = 1; idx <= bucket_count; idx++) {
        print_bucket(bucket_names[idx])
    }
    print "target.minute180_immune_diff=" target["minute180_immune"]
    print "target.minute660_episodic_diff=" target["minute660_episodic"]
    print "open_bucket_count=" open_count
    print "open_buckets=" open_buckets
    if (open_count == 0) {
        print "immune_episodic_runtime_status=exact"
    } else {
        print "immune_episodic_runtime_status=inventory"
    }
}
' > "$BUCKETS"

status="$(awk -F= '$1 == "immune_episodic_runtime_status" { print $2 }' "$BUCKETS")"
open_bucket_count="$(awk -F= '$1 == "open_bucket_count" { print $2 }' "$BUCKETS")"
open_buckets="$(awk -F= '$1 == "open_buckets" { print $2 }' "$BUCKETS")"

{
    printf 'c_trace=%s\n' "$TRACE_DIR/c/c_engine_trace.trace"
    printf 'rust_trace=%s\n' "$TRACE_DIR/rust/rust_engine_trace.trace"
    printf 'selected_minute_samples=%s\n' "$c_count"
    printf 'bucket_inventory=%s\n' "$BUCKETS"
    printf 'mutation_check=pass\n'
    printf 'mutation_fixture=%s\n' "$MUTATED"
    printf 'immune_episodic_runtime_status=%s\n' "$status"
    printf 'open_bucket_count=%s\n' "$open_bucket_count"
    printf 'open_buckets=%s\n' "$open_buckets"
    printf 'covered=minute-180-immune-and-minute-660-episodic-sidecar-first-mismatch-classification-with-mutation-fixture\n'
    printf 'open=promote-immune-and-episodic-runtime-buckets-to-exact\n'
} > "$OUT_DIR/immune_episodic_runtime_inventory_manifest.txt"

echo "immune-episodic-runtime-inventory=pass out=$OUT_DIR status=$status open_buckets=$open_bucket_count"

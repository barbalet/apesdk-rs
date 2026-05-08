#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/brain-social-runtime-inventory"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"

TRACE_DIR="$OUT_DIR/traces"
"$ROOT/scripts/generate_c_engine_trace_probe.sh" "$TRACE_DIR/c" >/dev/null
"$ROOT/scripts/generate_rust_engine_trace_probe.sh" "$TRACE_DIR/rust" >/dev/null

C_MINUTE="$OUT_DIR/c.selected-minute"
RUST_MINUTE="$OUT_DIR/rust.selected-minute"
C_AFTER_DAY="$OUT_DIR/c.after-day"
RUST_AFTER_DAY="$OUT_DIR/rust.after-day"
BUCKETS="$OUT_DIR/brain_social_runtime_buckets.txt"

grep '^SELECTED-MINUTE ' "$TRACE_DIR/c/c_engine_trace.trace" > "$C_MINUTE"
grep '^SELECTED-MINUTE ' "$TRACE_DIR/rust/rust_engine_trace.trace" > "$RUST_MINUTE"
awk '$2 == "snapshot=after_day" { print }' "$TRACE_DIR/c/c_engine_trace.trace" > "$C_AFTER_DAY"
awk '$2 == "snapshot=after_day" { print }' "$TRACE_DIR/rust/rust_engine_trace.trace" > "$RUST_AFTER_DAY"

c_count="$(wc -l < "$C_MINUTE" | tr -d ' ')"
rust_count="$(wc -l < "$RUST_MINUTE" | tr -d ' ')"
if [ "$c_count" != "$rust_count" ]; then
    echo "brain/social selected-minute count mismatch: c=$c_count rust=$rust_count" >&2
    exit 1
fi

awk -v c_minute="$C_MINUTE" -v rust_minute="$RUST_MINUTE" \
    -v c_after="$C_AFTER_DAY" -v rust_after="$RUST_AFTER_DAY" '
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

function compare_minute_bucket(name, fields,   diffs) {
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

function compare_final_bucket(name, fields,   diffs) {
    diffs = diff_fields(fields)
    if (diffs != "") {
        status[name] = "inventory"
        final_diff[name] = diffs
    }
}

function print_bucket(name) {
    print "bucket." name ".status=" status[name]
    if (first_minute[name] != "") {
        print "bucket." name ".first_minute=" first_minute[name]
        print "bucket." name ".first_minute_diff=" first_diff[name]
    }
    if (final_diff[name] != "") {
        print "bucket." name ".after_day_diff=" final_diff[name]
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
    bucket_count = 4
    bucket_names[1] = "brain_probe"
    bucket_names[2] = "social"
    bucket_names[3] = "episodic"
    bucket_names[4] = "immune"
    minute_fields["brain_probe"] = "brain"
    minute_fields["social"] = "social0"
    minute_fields["episodic"] = "episodic0"
    minute_fields["immune"] = "immune"
    final_fields["brain_probe"] = "brain probe0 shout"
    final_fields["social"] = "social0 territory0 family conception"
    final_fields["episodic"] = "episodic0"
    final_fields["immune"] = "immune"
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
            compare_minute_bucket(name, minute_fields[name])
        }
    }
    close(c_minute)
    close(rust_minute)

    if ((getline c_line < c_after) <= 0 || (getline rust_line < rust_after) <= 0) {
        print "status=missing-after-day-row"
        exit 2
    }
    close(c_after)
    close(rust_after)
    parse(c_line, c)
    parse(rust_line, rust)
    for (idx = 1; idx <= bucket_count; idx++) {
        name = bucket_names[idx]
        compare_final_bucket(name, final_fields[name])
    }

    open_count = 0
    open_buckets = ""
    print "c_selected_minute=" c_minute
    print "rust_selected_minute=" rust_minute
    print "c_after_day=" c_after
    print "rust_after_day=" rust_after
    for (idx = 1; idx <= bucket_count; idx++) {
        print_bucket(bucket_names[idx])
    }
    print "open_bucket_count=" open_count
    print "open_buckets=" open_buckets
    if (open_count == 0) {
        print "brain_social_runtime_status=exact"
    } else {
        print "brain_social_runtime_status=inventory"
    }
}
' > "$BUCKETS"

brain_social_runtime_status="$(awk -F= '$1 == "brain_social_runtime_status" { print $2 }' "$BUCKETS")"
open_bucket_count="$(awk -F= '$1 == "open_bucket_count" { print $2 }' "$BUCKETS")"
open_buckets="$(awk -F= '$1 == "open_buckets" { print $2 }' "$BUCKETS")"

{
    printf 'c_trace=%s\n' "$TRACE_DIR/c/c_engine_trace.trace"
    printf 'rust_trace=%s\n' "$TRACE_DIR/rust/rust_engine_trace.trace"
    printf 'selected_minute_samples=%s\n' "$c_count"
    printf 'bucket_inventory=%s\n' "$BUCKETS"
    printf 'brain_social_runtime_status=%s\n' "$brain_social_runtime_status"
    printf 'open_bucket_count=%s\n' "$open_bucket_count"
    printf 'open_buckets=%s\n' "$open_buckets"
    printf 'covered=brain-probe-social-episodic-immune-minute-and-after-day-first-mismatch-classification\n'
    printf 'open=promote-brain-probe-social-episodic-immune-runtime-buckets-to-exact\n'
} > "$OUT_DIR/brain_social_runtime_inventory_manifest.txt"

echo "brain-social-runtime-inventory=pass out=$OUT_DIR status=$brain_social_runtime_status open_buckets=$open_bucket_count"

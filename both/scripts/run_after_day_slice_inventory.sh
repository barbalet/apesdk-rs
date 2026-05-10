#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/after-day-slice-inventory"}"

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
C_AFTER_DAY="$OUT_DIR/c.after-day"
RUST_AFTER_DAY="$OUT_DIR/rust.after-day"
BUCKETS="$OUT_DIR/after_day_slice_buckets.txt"

grep '^SELECTED-MINUTE ' "$TRACE_DIR/c/c_engine_trace.trace" > "$C_MINUTE"
grep '^SELECTED-MINUTE ' "$TRACE_DIR/rust/rust_engine_trace.trace" > "$RUST_MINUTE"
awk '$2 == "snapshot=after_day" { print }' "$TRACE_DIR/c/c_engine_trace.trace" > "$C_AFTER_DAY"
awk '$2 == "snapshot=after_day" { print }' "$TRACE_DIR/rust/rust_engine_trace.trace" > "$RUST_AFTER_DAY"

c_count="$(wc -l < "$C_MINUTE" | tr -d ' ')"
rust_count="$(wc -l < "$RUST_MINUTE" | tr -d ' ')"
if [ "$c_count" != "$rust_count" ]; then
    echo "after-day selected-minute count mismatch: c=$c_count rust=$rust_count" >&2
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
    bucket_names[1] = "movement"
    bucket_names[2] = "energy"
    bucket_names[3] = "body"
    bucket_names[4] = "honor_drives"
    fields["movement"] = "loc facing speed terrain food"
    fields["energy"] = "energy food"
    fields["body"] = "height mass posture parasites crowding awake state inventory"
    fields["honor_drives"] = "honor drives preference goal socialcoord seed"
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
            compare_minute_bucket(name, fields[name])
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
    compare_final_bucket("movement", "loc facing velocity")
    compare_final_bucket("energy", "energy")
    compare_final_bucket("body", "height mass posture awake inventory")
    compare_final_bucket("honor_drives", "honor drives preference")

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
        print "after_day_slice_status=exact"
    } else {
        print "after_day_slice_status=inventory"
    }
}
' > "$BUCKETS"

after_day_slice_status="$(awk -F= '$1 == "after_day_slice_status" { print $2 }' "$BUCKETS")"
open_bucket_count="$(awk -F= '$1 == "open_bucket_count" { print $2 }' "$BUCKETS")"
open_buckets="$(awk -F= '$1 == "open_buckets" { print $2 }' "$BUCKETS")"

{
    printf 'c_trace=%s\n' "$TRACE_DIR/c/c_engine_trace.trace"
    printf 'rust_trace=%s\n' "$TRACE_DIR/rust/rust_engine_trace.trace"
    printf 'selected_minute_samples=%s\n' "$c_count"
    printf 'bucket_inventory=%s\n' "$BUCKETS"
    printf 'after_day_slice_status=%s\n' "$after_day_slice_status"
    printf 'open_bucket_count=%s\n' "$open_bucket_count"
    printf 'open_buckets=%s\n' "$open_buckets"
    printf 'covered=day-one-selected-movement-terrain-food-energy-body-honor-drive-random-first-mismatch-classification\n'
    printf 'open=promote-open-day-one-movement-terrain-food-energy-body-honor-drive-random-buckets-to-exact\n'
} > "$OUT_DIR/after_day_slice_inventory_manifest.txt"

echo "after-day-slice-inventory=pass out=$OUT_DIR status=$after_day_slice_status open_buckets=$open_bucket_count"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/after-day-drift-inventory"}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"

SELECTED_DIR="$OUT_DIR/selected-being"
"$ROOT/scripts/run_selected_being_value_inventory.sh" "$SELECTED_DIR" >/dev/null

C_AFTER_DAY="$OUT_DIR/c.after_day"
RUST_AFTER_DAY="$OUT_DIR/rust.after_day"
BUCKETS="$OUT_DIR/after_day_drift_buckets.txt"

awk '$2 == "snapshot=after_day" { print }' "$SELECTED_DIR/c.selected" > "$C_AFTER_DAY"
awk '$2 == "snapshot=after_day" { print }' "$SELECTED_DIR/rust.selected" > "$RUST_AFTER_DAY"

awk -v c_file="$C_AFTER_DAY" -v rust_file="$RUST_AFTER_DAY" '
function read_values(path, values,   line, count, parts, idx, kv) {
    if ((getline line < path) <= 0) {
        return 0
    }
    close(path)
    count = split(line, parts, " ")
    for (idx = 2; idx <= count; idx++) {
        split(parts[idx], kv, "=")
        values[kv[1]] = kv[2]
    }
    return 1
}

function compare_bucket(name, fields,   count, idx, split_fields, field, status, diffs) {
    status = "exact"
    diffs = ""
    count = split(fields, split_fields, " ")
    for (idx = 1; idx <= count; idx++) {
        field = split_fields[idx]
        if (c[field] != rust[field]) {
            status = "inventory"
            if (diffs != "") {
                diffs = diffs ","
            }
            diffs = diffs field ":" c[field] "->" rust[field]
        }
    }
    print "bucket." name ".status=" status
    if (diffs != "") {
        print "bucket." name ".diff=" diffs
    }
    if (status != "exact") {
        open_count++
        if (open_buckets != "") {
            open_buckets = open_buckets " "
        }
        open_buckets = open_buckets name
    }
}

BEGIN {
    if (!read_values(c_file, c) || !read_values(rust_file, rust)) {
        print "status=missing-after-day-row"
        exit 2
    }
    open_count = 0
    open_buckets = ""
    print "c_after_day=" c_file
    print "rust_after_day=" rust_file
    compare_bucket("selection", "population selected name")
    compare_bucket("movement", "loc facing velocity")
    compare_bucket("energy", "energy")
    compare_bucket("body", "height mass posture awake inventory")
    compare_bucket("honor_drives", "honor drives preference")
    compare_bucket("brain_probe", "brain probe0 shout")
    compare_bucket("social", "social0 territory0 family conception")
    compare_bucket("episodic", "episodic0")
    compare_bucket("immune", "immune")
    print "open_bucket_count=" open_count
    print "open_buckets=" open_buckets
    if (open_count == 0) {
        print "after_day_status=exact"
    } else {
        print "after_day_status=inventory"
    }
}
' > "$BUCKETS"

after_day_status="$(awk -F= '$1 == "after_day_status" { print $2 }' "$BUCKETS")"
open_bucket_count="$(awk -F= '$1 == "open_bucket_count" { print $2 }' "$BUCKETS")"
open_buckets="$(awk -F= '$1 == "open_buckets" { print $2 }' "$BUCKETS")"

{
    printf 'selected_inventory=%s\n' "$SELECTED_DIR/selected_being_value_inventory_manifest.txt"
    printf 'c_after_day=%s\n' "$C_AFTER_DAY"
    printf 'rust_after_day=%s\n' "$RUST_AFTER_DAY"
    printf 'bucket_inventory=%s\n' "$BUCKETS"
    printf 'first_cycle_status=exact\n'
    printf 'after_day_status=%s\n' "$after_day_status"
    printf 'open_bucket_count=%s\n' "$open_bucket_count"
    printf 'open_buckets=%s\n' "$open_buckets"
    printf 'covered=after-day-selected-being-drift-bucket-classification\n'
    printf 'open=promote-open-after-day-buckets-to-exact-value-gates\n'
} > "$OUT_DIR/after_day_drift_inventory_manifest.txt"

echo "after-day-drift-inventory=pass out=$OUT_DIR after_day_status=$after_day_status open_buckets=$open_bucket_count"

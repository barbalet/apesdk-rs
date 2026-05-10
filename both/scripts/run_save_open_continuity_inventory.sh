#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/save-open-continuity-inventory"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"

RUST_TRACE="$OUT_DIR/rust_save_open_continuity.trace"
BEFORE_STATE="$OUT_DIR/before_save.state"
AFTER_STATE="$OUT_DIR/after_open.state"
STATE_DIFF="$OUT_DIR/save_open_state.diff"
CONTINUED_MINUTE_STATE="$OUT_DIR/continued_minute.state"
POST_OPEN_MINUTE_STATE="$OUT_DIR/post_open_minute.state"
POST_OPEN_MINUTE_DIFF="$OUT_DIR/post_open_minute.diff"
CONTINUED_DAY_STATE="$OUT_DIR/continued_day.state"
POST_OPEN_DAY_STATE="$OUT_DIR/post_open_day.state"
POST_OPEN_DAY_DIFF="$OUT_DIR/post_open_day.diff"
BEFORE_SIDECAR="$OUT_DIR/before_save.sidecar"
AFTER_SIDECAR="$OUT_DIR/after_open.sidecar"
SIDECAR_DIFF="$OUT_DIR/save_open_sidecar.diff"
CONTINUED_MINUTE_SIDECAR="$OUT_DIR/continued_minute.sidecar"
POST_OPEN_MINUTE_SIDECAR="$OUT_DIR/post_open_minute.sidecar"
POST_OPEN_MINUTE_SIDECAR_DIFF="$OUT_DIR/post_open_minute_sidecar.diff"
CONTINUED_DAY_SIDECAR="$OUT_DIR/continued_day.sidecar"
POST_OPEN_DAY_SIDECAR="$OUT_DIR/post_open_day.sidecar"
POST_OPEN_DAY_SIDECAR_DIFF="$OUT_DIR/post_open_day_sidecar.diff"
SIDECAR_BUCKETS="$OUT_DIR/save_open_sidecar_buckets.txt"

APESDK_FULL_DATE="$FULL_DATE" cargo run --manifest-path "$ROOT/rust/Cargo.toml" -p simape -- --save-open-trace > "$RUST_TRACE"
grep '^SAVE-OPEN snapshot=before_save ' "$RUST_TRACE" | sed 's/^SAVE-OPEN snapshot=[^ ]* //' > "$BEFORE_STATE"
grep '^SAVE-OPEN snapshot=after_open ' "$RUST_TRACE" | sed 's/^SAVE-OPEN snapshot=[^ ]* //' > "$AFTER_STATE"
if diff -u "$BEFORE_STATE" "$AFTER_STATE" > "$STATE_DIFF"; then
    state_status="exact"
else
    state_status="inventory"
fi
grep '^SAVE-OPEN snapshot=continued_minute ' "$RUST_TRACE" | sed 's/^SAVE-OPEN snapshot=[^ ]* //' > "$CONTINUED_MINUTE_STATE"
grep '^SAVE-OPEN snapshot=post_open_minute ' "$RUST_TRACE" | sed 's/^SAVE-OPEN snapshot=[^ ]* //' > "$POST_OPEN_MINUTE_STATE"
if diff -u "$CONTINUED_MINUTE_STATE" "$POST_OPEN_MINUTE_STATE" > "$POST_OPEN_MINUTE_DIFF"; then
    post_load_minute_status="exact"
else
    post_load_minute_status="inventory"
fi
grep '^SAVE-OPEN snapshot=continued_day ' "$RUST_TRACE" | sed 's/^SAVE-OPEN snapshot=[^ ]* //' > "$CONTINUED_DAY_STATE"
grep '^SAVE-OPEN snapshot=post_open_day ' "$RUST_TRACE" | sed 's/^SAVE-OPEN snapshot=[^ ]* //' > "$POST_OPEN_DAY_STATE"
if diff -u "$CONTINUED_DAY_STATE" "$POST_OPEN_DAY_STATE" > "$POST_OPEN_DAY_DIFF"; then
    post_load_day_status="exact"
else
    post_load_day_status="inventory"
fi
grep '^SAVE-OPEN-SIDECAR snapshot=before_save ' "$RUST_TRACE" | sed 's/^SAVE-OPEN-SIDECAR snapshot=[^ ]* //' > "$BEFORE_SIDECAR"
grep '^SAVE-OPEN-SIDECAR snapshot=after_open ' "$RUST_TRACE" | sed 's/^SAVE-OPEN-SIDECAR snapshot=[^ ]* //' > "$AFTER_SIDECAR"
if diff -u "$BEFORE_SIDECAR" "$AFTER_SIDECAR" > "$SIDECAR_DIFF"; then
    sidecar_status="exact"
else
    sidecar_status="inventory"
fi
grep '^SAVE-OPEN-SIDECAR snapshot=continued_minute ' "$RUST_TRACE" | sed 's/^SAVE-OPEN-SIDECAR snapshot=[^ ]* //' > "$CONTINUED_MINUTE_SIDECAR"
grep '^SAVE-OPEN-SIDECAR snapshot=post_open_minute ' "$RUST_TRACE" | sed 's/^SAVE-OPEN-SIDECAR snapshot=[^ ]* //' > "$POST_OPEN_MINUTE_SIDECAR"
if diff -u "$CONTINUED_MINUTE_SIDECAR" "$POST_OPEN_MINUTE_SIDECAR" > "$POST_OPEN_MINUTE_SIDECAR_DIFF"; then
    post_load_minute_sidecar_status="exact"
else
    post_load_minute_sidecar_status="inventory"
fi
grep '^SAVE-OPEN-SIDECAR snapshot=continued_day ' "$RUST_TRACE" | sed 's/^SAVE-OPEN-SIDECAR snapshot=[^ ]* //' > "$CONTINUED_DAY_SIDECAR"
grep '^SAVE-OPEN-SIDECAR snapshot=post_open_day ' "$RUST_TRACE" | sed 's/^SAVE-OPEN-SIDECAR snapshot=[^ ]* //' > "$POST_OPEN_DAY_SIDECAR"
if diff -u "$CONTINUED_DAY_SIDECAR" "$POST_OPEN_DAY_SIDECAR" > "$POST_OPEN_DAY_SIDECAR_DIFF"; then
    post_load_day_sidecar_status="exact"
else
    post_load_day_sidecar_status="inventory"
fi

awk -v continued_day="$CONTINUED_DAY_SIDECAR" -v post_open_day="$POST_OPEN_DAY_SIDECAR" '
function reset(values,   key) {
    for (key in values) {
        delete values[key]
    }
}

function parse(path, values,   line, count, parts, idx, kv) {
    reset(values)
    if ((getline line < path) <= 0) {
        return 0
    }
    close(path)
    count = split(line, parts, " ")
    for (idx = 1; idx <= count; idx++) {
        split(parts[idx], kv, "=")
        values[kv[1]] = kv[2]
    }
    return 1
}

function compare_bucket(name, fields,   count, idx, split_fields, field, diffs, status) {
    status = "exact"
    diffs = ""
    count = split(fields, split_fields, " ")
    for (idx = 1; idx <= count; idx++) {
        field = split_fields[idx]
        if (continued[field] != post_open[field]) {
            status = "inventory"
            if (diffs != "") {
                diffs = diffs ","
            }
            diffs = diffs field ":" continued[field] "->" post_open[field]
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
    if (!parse(continued_day, continued) || !parse(post_open_day, post_open)) {
        print "status=missing-sidecar-row"
        exit 2
    }
    open_count = 0
    open_buckets = ""
    print "continued_day_sidecar=" continued_day
    print "post_open_day_sidecar=" post_open_day
    compare_bucket("clock_land", "minute date time population")
    compare_bucket("selected_identity_random", "selected name seed")
    compare_bucket("movement_energy_body", "energy loc facing speed honor height mass posture parasites crowding awake state drives terrain food")
    compare_bucket("brain_social_episodic", "brain brainstate attention probe0 script shout social0 episodic0 territory0 goal socialcoord family conception")
    compare_bucket("immune_inventory", "immune inventory preference")
    print "open_bucket_count=" open_count
    print "open_buckets=" open_buckets
}
' > "$SIDECAR_BUCKETS"
post_load_day_sidecar_open_buckets="$(awk -F= '$1 == "open_bucket_count" { print $2 }' "$SIDECAR_BUCKETS")"
artifact_line="$(grep '^SAVE-OPEN artifact=' "$RUST_TRACE")"

SESSION="save_open_runtime_continuity"
COMMANDS="$ROOT/both/golden/cli/sessions/$SESSION.commands"
if [ ! -f "$COMMANDS" ]; then
    echo "missing save/open continuity commands: $COMMANDS" >&2
    exit 1
fi

command_count="$(grep -Evc '^[[:space:]]*($|#)' "$COMMANDS")"
transcript_status="not-run"
if [ "${APESDK_RUN_SAVE_OPEN_RAW_DIFF:-0}" = "1" ]; then
    if "$ROOT/both/scripts/run_raw_transcript_diff.sh" "$OUT_DIR/raw-diff" "$SESSION" > "$OUT_DIR/raw-diff.log" 2>&1; then
        transcript_status="exact"
    else
        transcript_status="inventory"
    fi
fi

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'session=%s\n' "$SESSION"
    printf 'commands=%s\n' "$COMMANDS"
    printf 'command_count=%s\n' "$command_count"
    printf 'rust_trace=%s\n' "$RUST_TRACE"
    printf 'rust_state_status=%s\n' "$state_status"
    printf 'rust_state_diff=%s\n' "$STATE_DIFF"
    printf 'rust_sidecar_status=%s\n' "$sidecar_status"
    printf 'rust_sidecar_diff=%s\n' "$SIDECAR_DIFF"
    printf 'post_load_minute_status=%s\n' "$post_load_minute_status"
    printf 'post_load_minute_diff=%s\n' "$POST_OPEN_MINUTE_DIFF"
    printf 'post_load_minute_sidecar_status=%s\n' "$post_load_minute_sidecar_status"
    printf 'post_load_minute_sidecar_diff=%s\n' "$POST_OPEN_MINUTE_SIDECAR_DIFF"
    printf 'post_load_day_status=%s\n' "$post_load_day_status"
    printf 'post_load_day_diff=%s\n' "$POST_OPEN_DAY_DIFF"
    printf 'post_load_day_sidecar_status=%s\n' "$post_load_day_sidecar_status"
    printf 'post_load_day_sidecar_diff=%s\n' "$POST_OPEN_DAY_SIDECAR_DIFF"
    printf 'post_load_day_sidecar_buckets=%s\n' "$SIDECAR_BUCKETS"
    printf 'post_load_day_sidecar_open_buckets=%s\n' "$post_load_day_sidecar_open_buckets"
    printf '%s\n' "$artifact_line"
    printf 'raw_diff_status=%s\n' "$transcript_status"
    if [ -f "$OUT_DIR/raw-diff.log" ]; then
        printf 'raw_diff_log=%s\n' "$OUT_DIR/raw-diff.log"
    fi
    printf 'covered=populated-save-open-run-save-open-session-presence-rust-state-continuity-selected-sidecars-post-load-advancement-and-optional-raw-diff\n'
    printf 'open=c-rust-runtime-transcript-speed-post-open-advancement-sidecar-parity artifact-byte-continuity\n'
} > "$OUT_DIR/save_open_continuity_inventory_manifest.txt"

echo "save-open-continuity-inventory=pass out=$OUT_DIR commands=$command_count rust_state_status=$state_status post_load_minute_status=$post_load_minute_status post_load_day_status=$post_load_day_status post_load_day_sidecar_open_buckets=$post_load_day_sidecar_open_buckets raw_diff_status=$transcript_status"

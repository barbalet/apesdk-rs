#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
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

APESDK_FULL_DATE="$FULL_DATE" cargo run --manifest-path "$ROOT/Cargo.toml" -p simape -- --save-open-trace > "$RUST_TRACE"
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
artifact_line="$(grep '^SAVE-OPEN artifact=' "$RUST_TRACE")"

SESSION="save_open_runtime_continuity"
COMMANDS="$ROOT/golden/cli/sessions/$SESSION.commands"
if [ ! -f "$COMMANDS" ]; then
    echo "missing save/open continuity commands: $COMMANDS" >&2
    exit 1
fi

command_count="$(grep -Evc '^[[:space:]]*($|#)' "$COMMANDS")"
transcript_status="not-run"
if [ "${APESDK_RUN_SAVE_OPEN_RAW_DIFF:-0}" = "1" ]; then
    if "$ROOT/scripts/run_raw_transcript_diff.sh" "$OUT_DIR/raw-diff" "$SESSION" > "$OUT_DIR/raw-diff.log" 2>&1; then
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
    printf 'post_load_minute_status=%s\n' "$post_load_minute_status"
    printf 'post_load_minute_diff=%s\n' "$POST_OPEN_MINUTE_DIFF"
    printf 'post_load_day_status=%s\n' "$post_load_day_status"
    printf 'post_load_day_diff=%s\n' "$POST_OPEN_DAY_DIFF"
    printf '%s\n' "$artifact_line"
    printf 'raw_diff_status=%s\n' "$transcript_status"
    if [ -f "$OUT_DIR/raw-diff.log" ]; then
        printf 'raw_diff_log=%s\n' "$OUT_DIR/raw-diff.log"
    fi
    printf 'covered=populated-save-open-run-save-open-session-presence-rust-state-continuity-post-load-advancement-and-optional-raw-diff\n'
    printf 'open=c-rust-runtime-transcript-speed-and-post-open-advancement-parity artifact-byte-continuity\n'
} > "$OUT_DIR/save_open_continuity_inventory_manifest.txt"

echo "save-open-continuity-inventory=pass out=$OUT_DIR commands=$command_count rust_state_status=$state_status post_load_minute_status=$post_load_minute_status post_load_day_status=$post_load_day_status raw_diff_status=$transcript_status"

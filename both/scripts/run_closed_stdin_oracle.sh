#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/closed-stdin-oracle"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"
RUNS="${APESDK_CLOSED_STDIN_RUNS:-6}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

NATIVE_DIR="$OUT_DIR/native-build"
COMMAND_DIR="$OUT_DIR/commands"
RAW_DIR="$OUT_DIR/raw"
CANONICAL_DIR="$OUT_DIR/canonical"

mkdir -p "$COMMAND_DIR" "$RAW_DIR/native" "$RAW_DIR/rust" "$CANONICAL_DIR/native" "$CANONICAL_DIR/rust"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/both/scripts/build_native_simape.sh" "$NATIVE_DIR" >/dev/null

: > "$COMMAND_DIR/closed_stdin.commands"

run_pipe() {
    local binary="$1"
    local commands="$2"
    local output="$3"
    local run_dir="$output.cwd"
    mkdir -p "$run_dir"
    ( cd "$run_dir" && "$binary" < "$commands" > "$output" ) || true
}

classify() {
    local transcript="$1"
    if [ ! -s "$transcript" ]; then
        printf 'empty'
        return
    fi
    local failures
    failures="$(grep -c '^ERROR: Console failure @ ./sim/console.c 220$' "$transcript" || true)"
    if grep -q "For a list of commands type 'help'" "$transcript" && [ "$failures" -eq 1 ]; then
        printf 'banner-one-failure'
    elif grep -q "For a list of commands type 'help'" "$transcript" && [ "$failures" -eq 2 ]; then
        printf 'banner-two-failures'
    else
        printf 'other'
    fi
}

manifest="$OUT_DIR/closed_stdin_oracle_manifest.txt"
{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'runs=%s\n' "$RUNS"
    printf 'canonical=banner-two-console-failures\n'
    printf 'native=%s\n' "$NATIVE_DIR/simape"
    printf 'rust=%s\n' "$ROOT/rust/target/debug/simape"
} > "$manifest"

for (( index = 1; index <= RUNS; index++ )); do
    run_pipe "$NATIVE_DIR/simape" "$COMMAND_DIR/closed_stdin.commands" "$RAW_DIR/native/closed_stdin_$index.txt"
    run_pipe "$ROOT/rust/target/debug/simape" "$COMMAND_DIR/closed_stdin.commands" "$RAW_DIR/rust/closed_stdin_$index.txt"

    "$ROOT/both/scripts/canonicalize_closed_stdin_transcript.sh" "$RAW_DIR/native/closed_stdin_$index.txt" > "$CANONICAL_DIR/native/closed_stdin_$index.txt"
    "$ROOT/both/scripts/canonicalize_closed_stdin_transcript.sh" "$RAW_DIR/rust/closed_stdin_$index.txt" > "$CANONICAL_DIR/rust/closed_stdin_$index.txt"
    diff -u "$CANONICAL_DIR/native/closed_stdin_$index.txt" "$CANONICAL_DIR/rust/closed_stdin_$index.txt"

    printf 'native_run_%02d=%s\n' "$index" "$(classify "$RAW_DIR/native/closed_stdin_$index.txt")" >> "$manifest"
    printf 'rust_run_%02d=%s\n' "$index" "$(classify "$RAW_DIR/rust/closed_stdin_$index.txt")" >> "$manifest"
done

echo "closed-stdin-oracle=pass out=$OUT_DIR runs=$RUNS full_date=$FULL_DATE"

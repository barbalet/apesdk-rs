#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/raw-transcript-diff"}"
shift || true

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

if [ "$#" -eq 0 ]; then
    set -- help help_errors command_edges run_forever_empty runtime_edges_empty
fi

FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"
NATIVE_DIR="$OUT_DIR/native-build"
RAW_DIR="$OUT_DIR/raw"
TRANSPORT_DIR="$OUT_DIR/transport"

mkdir -p "$RAW_DIR/native" "$RAW_DIR/rust" "$TRANSPORT_DIR/native" "$TRANSPORT_DIR/rust"

APESDK_FULL_DATE="$FULL_DATE" "$ROOT/both/scripts/build_native_simape.sh" "$NATIVE_DIR" >/dev/null

run_one() {
    local binary="$1"
    local commands="$2"
    local output="$3"
    local run_dir="$output.cwd"
    mkdir -p "$run_dir"
    if command -v expect >/dev/null 2>&1; then
        ( cd "$run_dir" && "$ROOT/both/golden/cli/run_cli_session.expect" "$binary" "$commands" "$output" )
    else
        ( cd "$run_dir" && "$binary" < "$commands" > "$output" ) || true
    fi
}

for name in "$@"; do
    commands="$ROOT/both/golden/cli/sessions/$name.commands"
    if [ ! -f "$commands" ]; then
        echo "missing session: $name" >&2
        exit 1
    fi
    if [ "$name" != "run_forever_empty" ] && grep -Eiq '^[[:space:]]*run[[:space:]]+forever([[:space:]]|$)' "$commands"; then
        echo "session is unsafe for native raw diff: $name" >&2
        exit 1
    fi

    run_one "$NATIVE_DIR/simape" "$commands" "$RAW_DIR/native/$name.txt"
    run_one "$ROOT/rust/target/debug/simape" "$commands" "$RAW_DIR/rust/$name.txt"
    "$ROOT/both/scripts/transport_normalize_transcript.sh" "$RAW_DIR/native/$name.txt" > "$TRANSPORT_DIR/native/$name.txt"
    "$ROOT/both/scripts/transport_normalize_transcript.sh" "$RAW_DIR/rust/$name.txt" > "$TRANSPORT_DIR/rust/$name.txt"
    diff -u "$TRANSPORT_DIR/native/$name.txt" "$TRANSPORT_DIR/rust/$name.txt"
done

echo "raw-transcript-diff=pass out=$OUT_DIR full_date=$FULL_DATE sessions=$*"

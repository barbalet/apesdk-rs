#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/native-trace-oracles"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR/transcripts" "$OUT_DIR/traces"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/build_native_simape.sh" "$OUT_DIR/native-build" >/dev/null

for name in trace_land_startup; do
    run_dir="$OUT_DIR/run-cwds/$name"
    mkdir -p "$run_dir"
    ( cd "$run_dir" && "$ROOT/golden/cli/run_cli_session.expect" \
        "$OUT_DIR/native-build/simape" \
        "$ROOT/golden/cli/sessions/$name.commands" \
        "$OUT_DIR/transcripts/$name.txt" )
    "$ROOT/scripts/native_sim_transcript_to_trace.sh" \
        "$OUT_DIR/transcripts/$name.txt" \
        "$name" > "$OUT_DIR/traces/$name.trace"
done

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'native=%s\n' "$OUT_DIR/native-build/simape"
    printf 'transcripts=%s\n' "$OUT_DIR/transcripts"
    printf 'traces=%s\n' "$OUT_DIR/traces"
    find "$OUT_DIR/traces" -maxdepth 1 -type f -print | sort
} > "$OUT_DIR/native_trace_manifest.txt"

echo "native-trace-oracles=$OUT_DIR"

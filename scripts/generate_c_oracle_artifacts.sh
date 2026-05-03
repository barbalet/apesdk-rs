#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/c-oracles"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR/transcripts" "$OUT_DIR/artifacts"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/build_native_simape.sh" "$OUT_DIR/native-build" >/dev/null

for commands in "$ROOT"/golden/cli/sessions/*.commands; do
    name="$(basename "$commands" .commands)"
    if [ "$name" != "run_forever_empty" ] && grep -Eiq '^[[:space:]]*run[[:space:]]+forever([[:space:]]|$)' "$commands"; then
        echo "skip native unsafe oracle session: $name" >&2
        continue
    fi
    run_dir="$OUT_DIR/run-cwds/$name"
    mkdir -p "$run_dir"
    ( cd "$run_dir" && "$ROOT/golden/cli/run_cli_session.expect" "$OUT_DIR/native-build/simape" "$commands" "$OUT_DIR/transcripts/$name.txt" )
done

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'native=%s\n' "$OUT_DIR/native-build/simape"
    printf 'transcripts=%s\n' "$OUT_DIR/transcripts"
    printf 'artifacts=%s\n' "$OUT_DIR/artifacts"
    find "$OUT_DIR/transcripts" -type f -maxdepth 1 -print | sort
} > "$OUT_DIR/oracle_manifest.txt"

echo "c-oracles=$OUT_DIR"

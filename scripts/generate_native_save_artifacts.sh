#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/native-save-artifacts"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR/transcripts" "$OUT_DIR/artifacts"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/build_native_simape.sh" "$OUT_DIR/native-build" >/dev/null

copy_artifact() {
    local source="$1"
    local name="$2"
    if [ -f "$source" ]; then
        cp "$source" "$OUT_DIR/artifacts/$name"
        shasum -a 256 "$OUT_DIR/artifacts/$name" >> "$OUT_DIR/artifact_hashes.txt"
    fi
}

rm -f /private/tmp/simape_cycle02_save.txt \
      /private/tmp/simape_runtime_parity.json \
      /private/tmp/simape_matrix.json \
      /private/tmp/simape_matrix.native \
      /private/tmp/simape_matrix.bin \
      "$OUT_DIR/artifact_hashes.txt"

for name in state_save_load runtime_parity save_open_matrix; do
    run_dir="$OUT_DIR/run-cwds/$name"
    mkdir -p "$run_dir"
    ( cd "$run_dir" && "$ROOT/golden/cli/run_cli_session.expect" \
        "$OUT_DIR/native-build/simape" \
        "$ROOT/golden/cli/sessions/$name.commands" \
        "$OUT_DIR/transcripts/$name.txt" )
done

copy_artifact /private/tmp/simape_cycle02_save.txt empty_startup_cli_save.txt
copy_artifact /private/tmp/simape_runtime_parity.json populated_short_cli_save.json
copy_artifact /private/tmp/simape_matrix.json matrix_cli_save.json
copy_artifact /private/tmp/simape_matrix.native matrix_cli_save.native
copy_artifact /private/tmp/simape_matrix.bin matrix_cli_save.bin

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'native=%s\n' "$OUT_DIR/native-build/simape"
    printf 'scenario_inventory=%s\n' "$ROOT/golden/absolute/binary_scenarios.txt"
    printf 'transcripts=%s\n' "$OUT_DIR/transcripts"
    printf 'artifacts=%s\n' "$OUT_DIR/artifacts"
    if [ -f "$OUT_DIR/artifact_hashes.txt" ]; then
        cat "$OUT_DIR/artifact_hashes.txt"
    fi
} > "$OUT_DIR/native_save_artifacts_manifest.txt"

echo "native-save-artifacts=$OUT_DIR"

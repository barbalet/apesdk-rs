#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/fuzz-parity-ci"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/run_command_fuzz_gate.sh" "$OUT_DIR/command"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/run_malformed_save_fuzz.sh" "$OUT_DIR/malformed-save"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/run_raw_binary_mutation_fuzz.sh" "$OUT_DIR/raw-binary"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/run_seed_population_fuzz.sh" "$OUT_DIR/seed-population"

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'gates=command malformed-save raw-binary seed-population\n'
    printf 'note=raw-binary and seed-population gates include triage markers for categories still waiting on native byte/value oracles.\n'
} > "$OUT_DIR/fuzz_parity_manifest.txt"

echo "fuzz-parity-ci=pass out=$OUT_DIR full_date=$FULL_DATE"

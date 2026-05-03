#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/native-raw-binary-oracle-gate"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/generate_native_raw_binary_oracle.sh" "$OUT_DIR/first" >/dev/null
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/generate_native_raw_binary_oracle.sh" "$OUT_DIR/second" >/dev/null

diff -u "$OUT_DIR/first/native_raw_binary_hashes.txt" "$OUT_DIR/second/native_raw_binary_hashes.txt"
diff -u "$OUT_DIR/first/native_raw_binary_byte_map.txt" "$OUT_DIR/second/native_raw_binary_byte_map.txt"

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'first=%s\n' "$OUT_DIR/first"
    printf 'second=%s\n' "$OUT_DIR/second"
    printf 'hashes=stable\n'
    printf 'byte_map=stable\n'
} > "$OUT_DIR/native_raw_binary_oracle_gate_manifest.txt"

echo "native-raw-binary-oracle-gate=pass out=$OUT_DIR full_date=$FULL_DATE"

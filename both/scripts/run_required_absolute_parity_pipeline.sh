#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/required-absolute-parity"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/both/scripts/run_platform_absolute_parity_gate.sh" "$OUT_DIR/platform"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/both/scripts/run_absolute_parity_ci.sh" "$OUT_DIR/absolute"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/both/scripts/run_profile_compiler_matrix_gate.sh" "$OUT_DIR/profile-compiler"
"$ROOT/both/scripts/run_absolute_parity_failure_smoke.sh" "$OUT_DIR/failure-smoke"

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'gates=platform absolute profile-compiler failure-smoke\n'
    printf 'absolute=%s\n' "$OUT_DIR/absolute"
    printf 'profile_compiler=%s\n' "$OUT_DIR/profile-compiler"
    printf 'failure_smoke=%s\n' "$OUT_DIR/failure-smoke"
} > "$OUT_DIR/required_absolute_parity_manifest.txt"

echo "required-absolute-parity=pass out=$OUT_DIR full_date=$FULL_DATE"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/platform-absolute-parity"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"
EXPECTED_PLATFORM="${APESDK_EXPECT_PLATFORM:-}"
RUN_ABSOLUTE="${APESDK_RUN_PLATFORM_ABSOLUTE:-0}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"
host_kernel="$(uname -s)"
host_machine="$(uname -m)"
case "$host_kernel" in
    Linux) platform="linux" ;;
    Darwin) platform="macos" ;;
    *) platform="$(printf '%s' "$host_kernel" | tr '[:upper:]' '[:lower:]')" ;;
esac

if [ -n "$EXPECTED_PLATFORM" ] && [ "$EXPECTED_PLATFORM" != "$platform" ]; then
    echo "expected platform $EXPECTED_PLATFORM but running on $platform" >&2
    exit 1
fi

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'platform=%s\n' "$platform"
    printf 'kernel=%s\n' "$host_kernel"
    printf 'machine=%s\n' "$host_machine"
    printf 'run_absolute=%s\n' "$RUN_ABSOLUTE"
    printf 'rustc='
    rustc --version
    printf 'cargo='
    cargo --version
    printf 'cc='
    "${CC:-cc}" --version | head -n 1
} > "$OUT_DIR/platform_manifest.txt"

if [ "$RUN_ABSOLUTE" = "1" ]; then
    APESDK_FULL_DATE="$FULL_DATE" "$ROOT/scripts/run_absolute_parity_ci.sh" "$OUT_DIR/absolute"
fi

echo "platform-absolute-parity=pass out=$OUT_DIR platform=$platform full_date=$FULL_DATE"

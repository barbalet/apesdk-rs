#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
C_ROOT="$ROOT/c"
OUT_DIR="${1:-"$ROOT/target/native"}"
BUILD_DIR="$OUT_DIR/objects"
COMPILER="${CC:-gcc}"
CFLAGS_TEXT="${CFLAGS:--O2}"
DEFINES_TEXT="${DEFINES:--DCOMMAND_LINE_EXPLICIT -DAUTOMATED}"
APESDK_FULL_DATE_TEXT="${APESDK_FULL_DATE:-May  1 2026}"

read -r -a CFLAGS_ARGS <<< "$CFLAGS_TEXT"
read -r -a DEFINES_ARGS <<< "$DEFINES_TEXT"
DEFINES_ARGS+=("-DFULL_DATE=\"$APESDK_FULL_DATE_TEXT\"")

mkdir -p "$BUILD_DIR"
cd "$C_ROOT"

MANIFEST="$OUT_DIR/native_build_manifest.txt"
{
    printf 'root=%s\n' "$ROOT"
    printf 'c_root=%s\n' "$C_ROOT"
    printf 'output=%s\n' "$OUT_DIR/simape"
    printf 'compiler=%s\n' "$COMPILER"
    "$COMPILER" --version | head -n 1
    printf 'cflags=%s\n' "$CFLAGS_TEXT"
    printf 'defines=%s\n' "$DEFINES_TEXT"
    printf 'full_date=%s\n' "$APESDK_FULL_DATE_TEXT"
    printf 'rustc='
    rustc --version
    printf 'cargo='
    cargo --version
} > "$MANIFEST"

sources=(
    toolkit/*.c
    script/*.c
    render/*.c
    sim/*.c
    entity/*.c
    universe/*.c
    longterm.c
)

objects=()
for source in "${sources[@]}"; do
    source_path="./$source"
    object_path="$BUILD_DIR/${source//\//_}.o"
    "$COMPILER" "${CFLAGS_ARGS[@]}" "${DEFINES_ARGS[@]}" -c "$source_path" -o "$object_path" -w
    objects+=("$object_path")
done

"$COMPILER" "${CFLAGS_ARGS[@]}" "${DEFINES_ARGS[@]}" -I/usr/include -o "$OUT_DIR/simape" "${objects[@]}" -lz -lm -lpthread
APESDK_FULL_DATE="$APESDK_FULL_DATE_TEXT" cargo build --manifest-path "$ROOT/rust/Cargo.toml" -p simape

printf 'native=%s\nrust=%s\nmanifest=%s\n' "$OUT_DIR/simape" "$ROOT/rust/target/debug/simape" "$MANIFEST"

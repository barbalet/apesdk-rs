#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/profile-compiler-matrix"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

NATIVE_RELEASE_DIR="$OUT_DIR/native-release"
NATIVE_DEBUG_DIR="$OUT_DIR/native-debug"
RAW_DIR="$OUT_DIR/raw"
TRANSPORT_DIR="$OUT_DIR/transport"
sessions=(help_errors command_edges run_forever_empty runtime_edges_empty)

mkdir -p "$RAW_DIR" "$TRANSPORT_DIR"
APESDK_FULL_DATE="$FULL_DATE" CFLAGS="-O2" "$ROOT/scripts/build_native_simape.sh" "$NATIVE_RELEASE_DIR" >/dev/null
APESDK_FULL_DATE="$FULL_DATE" CFLAGS="-O0" "$ROOT/scripts/build_native_simape.sh" "$NATIVE_DEBUG_DIR" >/dev/null
APESDK_FULL_DATE="$FULL_DATE" cargo build --manifest-path "$ROOT/Cargo.toml" --release -p simape >/dev/null

run_one() {
    local profile="$1"
    local binary="$2"
    local session="$3"
    local commands="$ROOT/golden/cli/sessions/$session.commands"
    local output="$RAW_DIR/$profile/$session.txt"
    local run_dir="$output.cwd"
    mkdir -p "$run_dir" "$TRANSPORT_DIR/$profile"
    if command -v expect >/dev/null 2>&1; then
        ( cd "$run_dir" && "$ROOT/golden/cli/run_cli_session.expect" "$binary" "$commands" "$output" )
    else
        ( cd "$run_dir" && "$binary" < "$commands" > "$output" ) || true
    fi
    "$ROOT/scripts/transport_normalize_transcript.sh" "$output" > "$TRANSPORT_DIR/$profile/$session.txt"
}

for session in "${sessions[@]}"; do
    run_one native-release "$NATIVE_RELEASE_DIR/simape" "$session"
    run_one native-debug "$NATIVE_DEBUG_DIR/simape" "$session"
    run_one rust-debug "$ROOT/target/debug/simape" "$session"
    run_one rust-release "$ROOT/target/release/simape" "$session"

    diff -u "$TRANSPORT_DIR/native-release/$session.txt" "$TRANSPORT_DIR/native-debug/$session.txt"
    diff -u "$TRANSPORT_DIR/native-release/$session.txt" "$TRANSPORT_DIR/rust-debug/$session.txt"
    diff -u "$TRANSPORT_DIR/native-release/$session.txt" "$TRANSPORT_DIR/rust-release/$session.txt"
done

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'sessions=%s\n' "${sessions[*]}"
    printf 'native_release=%s\n' "$NATIVE_RELEASE_DIR/simape"
    printf 'native_debug=%s\n' "$NATIVE_DEBUG_DIR/simape"
    printf 'rust_debug=%s\n' "$ROOT/target/debug/simape"
    printf 'rust_release=%s\n' "$ROOT/target/release/simape"
    printf 'compiler_release_manifest=%s\n' "$NATIVE_RELEASE_DIR/native_build_manifest.txt"
    printf 'compiler_debug_manifest=%s\n' "$NATIVE_DEBUG_DIR/native_build_manifest.txt"
} > "$OUT_DIR/profile_compiler_matrix_manifest.txt"

echo "profile-compiler-matrix=pass out=$OUT_DIR full_date=$FULL_DATE sessions=${sessions[*]}"

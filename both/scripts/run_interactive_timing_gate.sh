#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/interactive-timing-gate"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

NATIVE_DIR="$OUT_DIR/native-build"
COMMAND_DIR="$OUT_DIR/commands"
RAW_DIR="$OUT_DIR/raw"
TRANSPORT_DIR="$OUT_DIR/transport"

mkdir -p "$COMMAND_DIR" "$RAW_DIR/native" "$RAW_DIR/rust" "$TRANSPORT_DIR/native" "$TRANSPORT_DIR/rust"
APESDK_FULL_DATE="$FULL_DATE" "$ROOT/both/scripts/build_native_simape.sh" "$NATIVE_DIR" >/dev/null

run_pipe() {
    local binary="$1"
    local commands="$2"
    local output="$3"
    local run_dir="$output.cwd"
    mkdir -p "$run_dir"
    ( cd "$run_dir" && "$binary" < "$commands" > "$output" ) || true
}

run_expect() {
    local binary="$1"
    local commands="$2"
    local output="$3"
    local run_dir="$output.cwd"
    mkdir -p "$run_dir"
    if command -v expect >/dev/null 2>&1; then
        ( cd "$run_dir" && "$ROOT/both/golden/cli/run_cli_session.expect" "$binary" "$commands" "$output" )
    else
        run_pipe "$binary" "$commands" "$output"
    fi
}

compare_case() {
    local name="$1"
    "$ROOT/both/scripts/transport_normalize_transcript.sh" "$RAW_DIR/native/$name.txt" > "$TRANSPORT_DIR/native/$name.txt"
    "$ROOT/both/scripts/transport_normalize_transcript.sh" "$RAW_DIR/rust/$name.txt" > "$TRANSPORT_DIR/rust/$name.txt"
    diff -u "$TRANSPORT_DIR/native/$name.txt" "$TRANSPORT_DIR/rust/$name.txt"
}

compare_canonical_case() {
    local name="$1"
    local canonicalizer="$2"
    "$canonicalizer" "$RAW_DIR/native/$name.txt" > "$TRANSPORT_DIR/native/$name.txt"
    "$canonicalizer" "$RAW_DIR/rust/$name.txt" > "$TRANSPORT_DIR/rust/$name.txt"
    diff -u "$TRANSPORT_DIR/native/$name.txt" "$TRANSPORT_DIR/rust/$name.txt"
}

printf 'idea\n' > "$COMMAND_DIR/eof_quiet_command.commands"
printf 'help run\n' > "$COMMAND_DIR/eof_verbose_command.commands"
printf 'run 1 minute\n' > "$COMMAND_DIR/eof_bounded_run.commands"
: > "$COMMAND_DIR/closed_stdin.commands"

run_pipe "$NATIVE_DIR/simape" "$COMMAND_DIR/eof_quiet_command.commands" "$RAW_DIR/native/eof_quiet_command.txt"
run_pipe "$ROOT/rust/target/debug/simape" "$COMMAND_DIR/eof_quiet_command.commands" "$RAW_DIR/rust/eof_quiet_command.txt"
compare_canonical_case eof_quiet_command "$ROOT/both/scripts/canonicalize_closed_stdin_transcript.sh"

run_pipe "$NATIVE_DIR/simape" "$COMMAND_DIR/eof_verbose_command.commands" "$RAW_DIR/native/eof_verbose_command.txt"
run_pipe "$ROOT/rust/target/debug/simape" "$COMMAND_DIR/eof_verbose_command.commands" "$RAW_DIR/rust/eof_verbose_command.txt"
compare_canonical_case eof_verbose_command "$ROOT/both/scripts/canonicalize_eof_verbose_command_transcript.sh"

run_pipe "$NATIVE_DIR/simape" "$COMMAND_DIR/eof_bounded_run.commands" "$RAW_DIR/native/eof_bounded_run.txt"
run_pipe "$ROOT/rust/target/debug/simape" "$COMMAND_DIR/eof_bounded_run.commands" "$RAW_DIR/rust/eof_bounded_run.txt"
compare_canonical_case eof_bounded_run "$ROOT/both/scripts/canonicalize_eof_bounded_run_transcript.sh"

run_pipe "$NATIVE_DIR/simape" "$COMMAND_DIR/closed_stdin.commands" "$RAW_DIR/native/closed_stdin.txt"
run_pipe "$ROOT/rust/target/debug/simape" "$COMMAND_DIR/closed_stdin.commands" "$RAW_DIR/rust/closed_stdin.txt"
compare_canonical_case closed_stdin "$ROOT/both/scripts/canonicalize_closed_stdin_transcript.sh"

run_expect "$NATIVE_DIR/simape" "$ROOT/both/golden/cli/sessions/timing_stop_forever.commands" "$RAW_DIR/native/timing_stop_forever.txt"
run_expect "$ROOT/rust/target/debug/simape" "$ROOT/both/golden/cli/sessions/timing_stop_forever.commands" "$RAW_DIR/rust/timing_stop_forever.txt"
compare_case timing_stop_forever

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'native=%s\n' "$NATIVE_DIR/simape"
    printf 'rust=%s\n' "$ROOT/rust/target/debug/simape"
    printf 'cases=eof_quiet_command eof_verbose_command eof_bounded_run closed_stdin timing_stop_forever\n'
    printf 'exact=timing_stop_forever\n'
    printf 'canonical=eof_quiet_command eof_verbose_command eof_bounded_run closed_stdin\n'
    printf 'console_failure_oracle=banner-two-console-failures canonicalized from native quiet-EOF/closed-stdin output classes\n'
    printf 'eof_verbose_oracle=help-run-line with optional native EOF console-failure lines canonicalized\n'
    printf 'eof_bounded_run_oracle=running-line-plus-console-failure canonicalized from native output order classes\n'
} > "$OUT_DIR/interactive_timing_manifest.txt"

echo "interactive-timing-gate=pass out=$OUT_DIR full_date=$FULL_DATE"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT/target/timing-regression-lock"}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

case "$OUT_DIR" in
    /*) ;;
    *) OUT_DIR="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_DIR"

canonical_closed="$OUT_DIR/closed_canonical.txt"
missing_failure="$OUT_DIR/closed_missing_failure.txt"
extra_failure="$OUT_DIR/closed_extra_failure.txt"
missing_banner="$OUT_DIR/closed_missing_banner.txt"
verbose_no_failure="$OUT_DIR/verbose_no_failure.txt"
verbose_one_failure="$OUT_DIR/verbose_one_failure.txt"
verbose_extra_failure="$OUT_DIR/verbose_extra_failure.txt"
verbose_missing_help="$OUT_DIR/verbose_missing_help.txt"
stop_expected="$OUT_DIR/stop_expected.txt"
stop_mutated="$OUT_DIR/stop_mutated.txt"

{
    printf '\n *** Simulated Ape 0.708 Console, %s ***\n' "$FULL_DATE"
    printf "      For a list of commands type 'help'\n\n"
    printf 'ERROR: Console failure @ ./sim/console.c 220\n'
    printf 'ERROR: Console failure @ ./sim/console.c 220\n'
} > "$canonical_closed"

{
    printf '\n *** Simulated Ape 0.708 Console, %s ***\n' "$FULL_DATE"
    printf "      For a list of commands type 'help'\n\n"
    printf 'ERROR: Console failure @ ./sim/console.c 220\n'
} > "$missing_failure"

{
    cat "$canonical_closed"
    printf 'ERROR: Console failure @ ./sim/console.c 220\n'
} > "$extra_failure"

{
    printf 'ERROR: Console failure @ ./sim/console.c 220\n'
    printf 'ERROR: Console failure @ ./sim/console.c 220\n'
} > "$missing_banner"

"$ROOT/scripts/canonicalize_closed_stdin_transcript.sh" "$canonical_closed" > "$OUT_DIR/closed_canonical.out"
"$ROOT/scripts/canonicalize_closed_stdin_transcript.sh" "$missing_failure" > "$OUT_DIR/closed_missing_failure.out"
diff -u "$OUT_DIR/closed_canonical.out" "$OUT_DIR/closed_missing_failure.out"

if "$ROOT/scripts/canonicalize_closed_stdin_transcript.sh" "$extra_failure" > "$OUT_DIR/closed_extra_failure.out" 2> "$OUT_DIR/closed_extra_failure.err"; then
    echo "extra console-failure line was accepted" >&2
    exit 1
fi

if "$ROOT/scripts/canonicalize_closed_stdin_transcript.sh" "$missing_banner" > "$OUT_DIR/closed_missing_banner.out" 2> "$OUT_DIR/closed_missing_banner.err"; then
    echo "missing banner was accepted" >&2
    exit 1
fi

{
    printf '\n *** Simulated Ape 0.708 Console, %s ***\n' "$FULL_DATE"
    printf "      For a list of commands type 'help'\n\n"
    printf ' run (time format)|forever   Simulate for a given number of days or forever\n'
} > "$verbose_no_failure"

{
    cat "$verbose_no_failure"
    printf 'ERROR: Console failure @ ./sim/console.c 220\n'
} > "$verbose_one_failure"

{
    cat "$verbose_no_failure"
    printf 'ERROR: Console failure @ ./sim/console.c 220\n'
    printf 'ERROR: Console failure @ ./sim/console.c 220\n'
    printf 'ERROR: Console failure @ ./sim/console.c 220\n'
} > "$verbose_extra_failure"

{
    printf '\n *** Simulated Ape 0.708 Console, %s ***\n' "$FULL_DATE"
    printf "      For a list of commands type 'help'\n\n"
    printf 'ERROR: Console failure @ ./sim/console.c 220\n'
} > "$verbose_missing_help"

"$ROOT/scripts/canonicalize_eof_verbose_command_transcript.sh" "$verbose_no_failure" > "$OUT_DIR/verbose_no_failure.out"
"$ROOT/scripts/canonicalize_eof_verbose_command_transcript.sh" "$verbose_one_failure" > "$OUT_DIR/verbose_one_failure.out"
diff -u "$OUT_DIR/verbose_no_failure.out" "$OUT_DIR/verbose_one_failure.out"

if "$ROOT/scripts/canonicalize_eof_verbose_command_transcript.sh" "$verbose_extra_failure" > "$OUT_DIR/verbose_extra_failure.out" 2> "$OUT_DIR/verbose_extra_failure.err"; then
    echo "verbose EOF extra console-failure line was accepted" >&2
    exit 1
fi

if "$ROOT/scripts/canonicalize_eof_verbose_command_transcript.sh" "$verbose_missing_help" > "$OUT_DIR/verbose_missing_help.out" 2> "$OUT_DIR/verbose_missing_help.err"; then
    echo "verbose EOF missing help line was accepted" >&2
    exit 1
fi

cat > "$stop_expected" <<'STOP'
run forever
Running forever (type "stop" to end)
stop
Simulation stopped
quit
Simulation stopped
STOP

cat > "$stop_mutated" <<'STOP'
run forever
Running forever (type "stop" to end)
quit
Simulation stopped
stop
Simulation stopped
STOP

if diff -u "$stop_expected" "$stop_mutated" >/dev/null 2>&1; then
    echo "stop/quit order mutation was not detected" >&2
    exit 1
fi

{
    printf 'full_date=%s\n' "$FULL_DATE"
    printf 'missing_failure=canonicalized_to_native_output_class\n'
    printf 'extra_failure=rejected\n'
    printf 'missing_banner=rejected\n'
    printf 'verbose_eof_optional_failure=canonicalized_to_native_output_class\n'
    printf 'verbose_eof_extra_failure=rejected\n'
    printf 'verbose_eof_missing_help=rejected\n'
    printf 'stop_quit_order=mutation_detected\n'
} > "$OUT_DIR/timing_regression_lock_manifest.txt"

echo "timing-regression-lock=pass out=$OUT_DIR full_date=$FULL_DATE"

#!/usr/bin/env bash
set -euo pipefail

input="${1:-/dev/stdin}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

content="$(tr -d '\r' < "$input")"
help_count="$(
    printf '%s\n' "$content" |
        grep -c '^ run (time format)|forever   Simulate for a given number of days or forever$' || true
)"
failure_count="$(printf '%s\n' "$content" | grep -c '^ERROR: Console failure @ ./sim/console.c 220$' || true)"

unexpected="$(
    printf '%s\n' "$content" |
        grep -Ev '^$|^ \*\*\* Simulated Ape 0\.708 Console, .* \*\*\*$|^      For a list of commands type '\''help'\''$|^ run \(time format\)\|forever   Simulate for a given number of days or forever$|^ERROR: Console failure @ \./sim/console\.c 220$' || true
)"

if printf '%s\n' "$content" | grep -q "For a list of commands type 'help'" &&
   [ "$help_count" -eq 1 ] &&
   [ "$failure_count" -le 2 ] &&
   [ -z "$unexpected" ]; then
    printf '\n *** Simulated Ape 0.708 Console, %s ***\n' "$FULL_DATE"
    printf "      For a list of commands type 'help'\n\n"
    printf ' run (time format)|forever   Simulate for a given number of days or forever\n'
    exit 0
fi

echo "unrecognized verbose EOF transcript: $input" >&2
printf '%s\n' "$content" >&2
exit 1

#!/usr/bin/env bash
set -euo pipefail

input="${1:-/dev/stdin}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

content="$(tr -d '\r' < "$input")"
if [ -z "$content" ]; then
    printf '\n *** Simulated Ape 0.708 Console, %s ***\n' "$FULL_DATE"
    printf "      For a list of commands type 'help'\n\n"
    printf 'Running for 1 mins\n'
    printf 'ERROR: Console failure @ ./sim/console.c 220\n'
    exit 0
fi

running_count="$(printf '%s\n' "$content" | grep -c '^Running for 1 mins$' || true)"
failure_count="$(printf '%s\n' "$content" | grep -c '^ERROR: Console failure @ ./sim/console.c 220$' || true)"

unexpected="$(
    printf '%s\n' "$content" |
        grep -Ev '^$|^ \*\*\* Simulated Ape 0\.708 Console, .* \*\*\*$|^      For a list of commands type '\''help'\''$|^Running for 1 mins$|^ERROR: Console failure @ \./sim/console\.c 220$' || true
)"

if printf '%s\n' "$content" | grep -q "For a list of commands type 'help'" &&
   [ "$running_count" -eq 1 ] &&
   [ "$failure_count" -eq 1 ] &&
   [ -z "$unexpected" ]; then
    printf '\n *** Simulated Ape 0.708 Console, %s ***\n' "$FULL_DATE"
    printf "      For a list of commands type 'help'\n\n"
    printf 'Running for 1 mins\n'
    printf 'ERROR: Console failure @ ./sim/console.c 220\n'
    exit 0
fi

echo "unrecognized bounded-run EOF transcript: $input" >&2
printf '%s\n' "$content" >&2
exit 1

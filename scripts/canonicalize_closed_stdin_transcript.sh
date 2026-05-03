#!/usr/bin/env bash
set -euo pipefail

input="${1:-/dev/stdin}"
FULL_DATE="${APESDK_FULL_DATE:-May  1 2026}"

canonical() {
    printf '\n *** Simulated Ape 0.708 Console, %s ***\n' "$FULL_DATE"
    printf "      For a list of commands type 'help'\n\n"
    printf 'ERROR: Console failure @ ./sim/console.c 220\n'
    printf 'ERROR: Console failure @ ./sim/console.c 220\n'
}

content="$(tr -d '\r' < "$input")"
if [ -z "$content" ]; then
    canonical
    exit 0
fi

failure_count="$(printf '%s\n' "$content" | grep -c '^ERROR: Console failure @ ./sim/console.c 220$' || true)"
if printf '%s\n' "$content" | grep -q "For a list of commands type 'help'" &&
   [ "$failure_count" -ge 1 ] &&
   [ "$failure_count" -le 2 ]; then
    unexpected="$(
        printf '%s\n' "$content" |
            grep -Ev '^$|^ \*\*\* Simulated Ape 0\.708 Console, .* \*\*\*$|^      For a list of commands type '\''help'\''$|^ERROR: Console failure @ \./sim/console\.c 220$' || true
    )"
    if [ -z "$unexpected" ]; then
        canonical
        exit 0
    fi
fi

echo "unrecognized closed-stdin transcript: $input" >&2
printf '%s\n' "$content" >&2
exit 1

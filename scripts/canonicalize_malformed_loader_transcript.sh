#!/usr/bin/env bash
set -euo pipefail

input="${1:-/dev/stdin}"

sed -E \
    -e '/^simape\([0-9]+,0x[0-9a-fA-F]+\) malloc: \*\*\*/d' \
    -e 's/String length : [0-9]+/String length : <N>/g' \
    -e 's/Actual size : [0-9]+/Actual size : <N>/g' \
    -e 's#/private/tmp/[^[:space:]"'"'"']+#<TMP>#g' \
    -e 's#/tmp/[^[:space:]"'"'"']+#<TMP>#g' \
    "$input" |
    tr -d '\r'

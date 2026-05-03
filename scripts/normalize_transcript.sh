#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -gt 1 ]; then
    echo "usage: normalize_transcript.sh [file]" >&2
    exit 64
fi

if [ "$#" -eq 1 ]; then
    input="$1"
else
    input="-"
fi

perl -pe 's/\r\n?/\n/g; s#\S*/apesdk-rs/#./#g; s#/private/tmp/[^[:space:]"'\''"]+#<TMP>#g; s#/tmp/[^[:space:]"'\''"]+#<TMP>#g; s/^String length : [0-9]+$/String length : <N>/; s/^Actual size : [0-9]+$/Actual size : <N>/' "$input"

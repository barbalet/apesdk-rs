#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 2 ]; then
    echo "usage: trace_diff.sh <expected.trace> <actual.trace>" >&2
    exit 64
fi

expected="$1"
actual="$2"

if cmp -s "$expected" "$actual"; then
    echo "trace diff: pass"
    exit 0
fi

set +e
line="$(diff -u "$expected" "$actual" | awk '/^-TRACE|^\\+TRACE|^-/{print; count++; if (count == 4) exit}')"
set -e
echo "trace diff: mismatch"
echo "$line"
exit 1

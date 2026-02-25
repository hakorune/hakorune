#!/bin/bash
set -euo pipefail

HAKORUNE_BIN="${HAKORUNE_BIN:-./target/release/hakorune}"
HAKO_PATH="apps/tests/phase274_p1_typeop_is_as_min.hako"
EXPECTED_EXIT=3

set +e
$HAKORUNE_BIN --backend vm "$HAKO_PATH" >/dev/null 2>&1
actual_exit=$?
set -e

if [[ $actual_exit -eq $EXPECTED_EXIT ]]; then
    echo "✅ phase274_p1_typeop_is_as_vm: PASS (exit=$actual_exit)"
    exit 0
else
    echo "❌ phase274_p1_typeop_is_as_vm: FAIL (expected=$EXPECTED_EXIT, got=$actual_exit)"
    exit 1
fi


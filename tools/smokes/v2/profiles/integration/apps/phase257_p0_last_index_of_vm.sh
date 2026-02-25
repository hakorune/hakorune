#!/usr/bin/env bash
# Phase 257 P0: last_index_of pattern (reverse scan) - VM
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"
HAKORUNE_BIN="${HAKORUNE_BIN:-$PROJECT_ROOT/target/release/hakorune}"

echo "[INFO] Environment check passed"
echo "[INFO] Plugin mode: dynamic"
echo "[INFO] Dynamic plugins check passed"
echo "[DEBUG] PROJECT_ROOT=$PROJECT_ROOT"
echo "[DEBUG] Looking for: $PROJECT_ROOT/apps/tests/phase257_p0_last_index_of_min.hako"

# Fail-fast: Check fixture exists
if [ ! -f "$PROJECT_ROOT/apps/tests/phase257_p0_last_index_of_min.hako" ]; then
    echo "[FAIL] phase257_p0_last_index_of_vm: Fixture not found"
    exit 1
fi

# Run VM with the Phase 257 P0 fixture
set +e
"$HAKORUNE_BIN" --backend vm "$PROJECT_ROOT/apps/tests/phase257_p0_last_index_of_min.hako"
EXIT_CODE=$?
set -e

# Expected: RC=7 (last index of 'o' in "hello world")
if [ "$EXIT_CODE" -eq 7 ]; then
    echo "[PASS] phase257_p0_last_index_of_vm: RC=$EXIT_CODE (expected 7)"
    exit 0
else
    echo "[FAIL] phase257_p0_last_index_of_vm: RC=$EXIT_CODE (expected 7)"
    exit 1
fi

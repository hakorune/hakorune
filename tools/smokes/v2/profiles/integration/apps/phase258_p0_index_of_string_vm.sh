#!/usr/bin/env bash
# Phase 258 P0: index_of_string pattern (dynamic needle) - VM
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../../../../.." && pwd)"
HAKORUNE_BIN="${HAKORUNE_BIN:-$PROJECT_ROOT/target/release/hakorune}"
HAKO_PATH="$PROJECT_ROOT/apps/tests/phase258_p0_index_of_string_min.hako"

echo "[INFO] Environment check passed"
echo "[INFO] Plugin mode: dynamic"
echo "[INFO] Dynamic plugins check passed"

# Phase 258 P0: Step 1 - Add --verify flag (fail-fast on MIR errors)
set +e
VERIFY_OUTPUT=$("$HAKORUNE_BIN" --backend vm --verify "$HAKO_PATH" 2>&1)
VERIFY_EXIT=$?
set -e

if [ "$VERIFY_EXIT" -ne 0 ]; then
    echo "❌ phase258_p0_index_of_string_vm: FAIL (MIR verification failed)"
    echo "$VERIFY_OUTPUT"
    exit 1
fi

# Phase 258 P0: Step 2 - Run VM with error detection
set +e
OUTPUT=$("$HAKORUNE_BIN" --backend vm "$HAKO_PATH" 2>&1)
EXIT_CODE=$?
set -e

# Check for VM errors in output (regardless of exit code)
if echo "$OUTPUT" | grep -Ei "error|panic|undefined|phi pred mismatch"; then
    echo "❌ phase258_p0_index_of_string_vm: FAIL (VM runtime error detected)"
    echo "$OUTPUT"
    exit 1
fi

# Validate expected exit code (now safe - we've ruled out errors)
# Expected: 6 (index of "world" in "hello world")
EXPECTED_EXIT=6
if [ "$EXIT_CODE" -eq "$EXPECTED_EXIT" ]; then
    echo "✅ phase258_p0_index_of_string_vm: PASS (exit=$EXIT_CODE, no errors)"
    exit 0
else
    echo "❌ phase258_p0_index_of_string_vm: FAIL (exit=$EXIT_CODE, expected $EXPECTED_EXIT)"
    echo "$OUTPUT"
    exit 1
fi

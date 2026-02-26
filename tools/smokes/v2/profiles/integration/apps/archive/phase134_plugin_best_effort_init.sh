#!/bin/bash
# Phase 134 P0: Plugin best-effort loading smoke test
# Tests: Even with plugin failures, plugins are not disabled entirely
# Acceptance: "plugins disabled (config=nyash.toml)" should NOT appear in output

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

mkdir -p "$NYASH_ROOT/tmp"

PASS_COUNT=0
FAIL_COUNT=0

# ===== Test 1: --dump-mir does not show "plugins disabled" =====
echo "[INFO] Phase 134 P0: Checking plugins are NOT disabled on --dump-mir"

INPUT="$NYASH_ROOT/apps/tests/phase132_return_loop_var_min.hako"

# Run with --dump-mir and capture output
set +e
"$NYASH_BIN" --dump-mir "$INPUT" > /tmp/phase134_dump.log 2>&1
EXIT_CODE=$?
set -e

# Check if "plugins disabled (config=nyash.toml)" appears
if grep -q "plugins disabled (config=nyash.toml)" /tmp/phase134_dump.log; then
    echo "[FAIL] Phase 134 P0: plugins disabled found in output (best-effort failed)"
    FAIL_COUNT=$((FAIL_COUNT + 1))
    # Show the problematic line for debugging
    grep "plugins disabled" /tmp/phase134_dump.log
else
    echo "[PASS] Phase 134 P0: plugins NOT disabled (best-effort loading successful)"
    PASS_COUNT=$((PASS_COUNT + 1))
fi

# ===== Test 2: --dump-mir succeeds with exit code 0 =====
echo "[INFO] Phase 134 P0: Checking MIR compilation succeeds"

if [ "$EXIT_CODE" -eq 0 ]; then
    echo "[PASS] Phase 134 P0: --dump-mir succeeded (exit code 0)"
    PASS_COUNT=$((PASS_COUNT + 1))
else
    echo "[FAIL] Phase 134 P0: --dump-mir failed (exit code $EXIT_CODE)"
    FAIL_COUNT=$((FAIL_COUNT + 1))
fi

# ===== Summary =====
echo ""
echo "[RESULT] Phase 134 P0: $PASS_COUNT passed, $FAIL_COUNT failed"
if [ "$FAIL_COUNT" -eq 0 ]; then
    exit 0
else
    exit 1
fi

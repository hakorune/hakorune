#!/bin/bash
# Phase 132: Exit PHI value parity smoke test
# Tests: Loop variable return via exit PHI (VM/LLVM parity)

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

# Check LLVM availability (SKIP if not found)
if ! command -v llvm-config-18 &> /dev/null; then
    test_skip "llvm-config-18 not found"; exit 0
fi

# Check hakorune LLVM backend availability
if ! "$NYASH_BIN" --help 2>&1 | grep -q "llvm"; then
    test_skip "hakorune --backend llvm not available"; exit 0
fi

# Check Python llvmlite
if ! python3 -c "import llvmlite" 2>/dev/null; then
    test_skip "Python llvmlite not found"; exit 0
fi

# Create tmp directory
mkdir -p "$NYASH_ROOT/tmp"

PASS_COUNT=0
FAIL_COUNT=0
BUILD_TIMEOUT_SECS=${BUILD_TIMEOUT_SECS:-300}
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

# ===== Case A: Simple loop return =====
echo "[INFO] Case A: phase132_return_loop_var_min.hako"

INPUT_A="$NYASH_ROOT/apps/tests/phase132_return_loop_var_min.hako"
OUTPUT_A="$NYASH_ROOT/tmp/phase132_return_loop_var_min"

if timeout "$BUILD_TIMEOUT_SECS" "$NYASH_ROOT/tools/build_llvm.sh" "$INPUT_A" -o "$OUTPUT_A" > /tmp/phase132_a_build.log 2>&1; then
    if [ -x "$OUTPUT_A" ]; then
        set +e
        timeout "$RUN_TIMEOUT_SECS" "$OUTPUT_A" > /dev/null 2>&1
        EXIT_CODE=$?  # 124=timeout
        set -e

        if [ "$EXIT_CODE" -eq 124 ]; then
            echo "[FAIL] Case A: executable timed out (>${RUN_TIMEOUT_SECS}s)"
            FAIL_COUNT=$((FAIL_COUNT + 1))
        elif [ "$EXIT_CODE" -eq 3 ]; then
            echo "[PASS] Case A: exit code 3 verified"
            PASS_COUNT=$((PASS_COUNT + 1))
        else
            echo "[FAIL] Case A: Expected exit code 3, got $EXIT_CODE"
            FAIL_COUNT=$((FAIL_COUNT + 1))
        fi
    else
        echo "[FAIL] Case A: Executable not created"
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
else
    if [ "$?" -eq 124 ]; then
        echo "[FAIL] Case A: build_llvm.sh timed out (>${BUILD_TIMEOUT_SECS}s)"
    else
        echo "[FAIL] Case A: build_llvm.sh failed"
    fi
    echo "[INFO] Case A build log (tail):"
    tail -n 80 /tmp/phase132_a_build.log || true
    FAIL_COUNT=$((FAIL_COUNT + 1))
fi

# ===== Case B: Complex loop with early exit =====
echo "[INFO] Case B: llvm_stage3_loop_only.hako"

INPUT_B="$NYASH_ROOT/apps/tests/llvm_stage3_loop_only.hako"
OUTPUT_B="$NYASH_ROOT/tmp/llvm_stage3_loop_only"

# Case B can reuse the runtime built by Case A; skip rebuilding NyRT to reduce runtime.
if timeout "$BUILD_TIMEOUT_SECS" env NYASH_LLVM_SKIP_NYRT_BUILD=1 "$NYASH_ROOT/tools/build_llvm.sh" "$INPUT_B" -o "$OUTPUT_B" > /tmp/phase132_b_build.log 2>&1; then
    if [ -x "$OUTPUT_B" ]; then
        set +e
        OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" "$OUTPUT_B" 2>&1)
        EXIT_CODE=$?  # 124=timeout
        set -e

        if [ "$EXIT_CODE" -eq 124 ]; then
            echo "[FAIL] Case B: executable timed out (>${RUN_TIMEOUT_SECS}s)"
            FAIL_COUNT=$((FAIL_COUNT + 1))
        elif echo "$OUTPUT" | grep -q "Result: 3"; then
            echo "[PASS] Case B: stdout contains 'Result: 3'"
            PASS_COUNT=$((PASS_COUNT + 1))
        else
            echo "[FAIL] Case B: Expected stdout to contain 'Result: 3'"
            echo "[INFO] Case B stdout (tail):"
            echo "$OUTPUT" | tail -n 80 || true
            FAIL_COUNT=$((FAIL_COUNT + 1))
        fi
    else
        echo "[FAIL] Case B: Executable not created"
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
else
    if [ "$?" -eq 124 ]; then
        echo "[FAIL] Case B: build_llvm.sh timed out (>${BUILD_TIMEOUT_SECS}s)"
    else
        echo "[FAIL] Case B: build_llvm.sh failed"
    fi
    echo "[INFO] Case B build log (tail):"
    tail -n 80 /tmp/phase132_b_build.log || true
    FAIL_COUNT=$((FAIL_COUNT + 1))
fi

# ===== Case C: Multi-function isolation (Rust VM) =====
echo "[INFO] Case C: phase132_multifunc_isolation_min.hako (Rust VM context isolation)"

INPUT_C="$NYASH_ROOT/apps/tests/phase132_multifunc_isolation_min.hako"

set +e
OUTPUT_C=$(timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" "$INPUT_C" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    echo "[FAIL] Case C: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    ((FAIL_COUNT++))
elif echo "$OUTPUT_C" | grep -q "RC: 6"; then
    echo "[PASS] Case C: Rust VM context isolation verified (RC: 6)"
    ((PASS_COUNT++))
else
    echo "[FAIL] Case C: Expected 'RC: 6' in output"
    echo "[INFO] Case C output (tail):"
    echo "$OUTPUT_C" | tail -n 20 || true
    ((FAIL_COUNT++))
fi

# ===== Summary =====
echo "[INFO] PASS: $PASS_COUNT, FAIL: $FAIL_COUNT"

if [ "$FAIL_COUNT" -eq 0 ]; then
    test_pass "phase132_exit_phi_parity: All tests passed"
    exit 0
else
    test_fail "phase132_exit_phi_parity: $FAIL_COUNT test(s) failed"
    exit 1
fi

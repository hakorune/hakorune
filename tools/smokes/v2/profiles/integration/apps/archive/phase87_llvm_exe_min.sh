#!/bin/bash
# Phase 87: LLVM exe line SSOT smoke test
# Tests: .hako → .o → executable → execution

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

# Build .hako → executable
INPUT_HAKO="$NYASH_ROOT/apps/tests/phase87_llvm_exe_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase87_llvm_exe_min"

echo "[INFO] Building: $INPUT_HAKO → $OUTPUT_EXE"

if ! "$NYASH_ROOT/tools/build_llvm.sh" "$INPUT_HAKO" -o "$OUTPUT_EXE" 2>&1 | tee /tmp/phase87_build.log; then
    echo "[FAIL] build_llvm.sh failed"
    cat /tmp/phase87_build.log
    exit 1
fi

# Verify executable created
if [ ! -x "$OUTPUT_EXE" ]; then
    echo "[FAIL] Executable not created or not executable: $OUTPUT_EXE"
    ls -la "$OUTPUT_EXE" 2>/dev/null || echo "File does not exist"
    exit 1
fi

# Execute and verify exit code
echo "[INFO] Executing: $OUTPUT_EXE"

set +e  # Allow non-zero exit
"$OUTPUT_EXE"
EXIT_CODE=$?
set -e

echo "[INFO] Exit code: $EXIT_CODE"

# Verify exit code == 42
if [ "$EXIT_CODE" -eq 42 ]; then
    test_pass "phase87_llvm_exe_min: exit code 42 verified"
else
    test_fail "Expected exit code 42, got $EXIT_CODE"
fi

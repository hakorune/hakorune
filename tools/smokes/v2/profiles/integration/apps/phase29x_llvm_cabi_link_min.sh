#!/bin/bash
# Phase 29x X37: LLVM + C ABI link minimal gate
#
# Contract pin:
# 1) Minimal `.hako` fixture links to native executable via LLVM line.
# 2) Linked executable runs with exit code 0.
# 3) Core C ABI surface guard stays in sync.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if ! can_run_llvm; then
    test_skip "phase29x_llvm_cabi_link_min: LLVM backend not available"
    exit 0
fi

if ! command -v llvm-config-18 >/dev/null 2>&1; then
    test_skip "phase29x_llvm_cabi_link_min: llvm-config-18 not found"
    exit 0
fi

if ! python3 -c "import llvmlite" >/dev/null 2>&1; then
    test_skip "phase29x_llvm_cabi_link_min: Python llvmlite not found"
    exit 0
fi

if ! bash "$NYASH_ROOT/tools/checks/nyrt_core_cabi_surface_guard.sh"; then
    test_fail "phase29x_llvm_cabi_link_min: core C ABI surface guard failed"
    exit 1
fi

INPUT="$NYASH_ROOT/apps/tests/hello_simple_llvm.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"
OUTPUT_EXE="${TMPDIR:-/tmp}/phase29x_llvm_cabi_link_min_$$"
BUILD_LOG="${TMPDIR:-/tmp}/phase29x_llvm_cabi_link_min_build_$$.log"

cleanup() {
    rm -f "$OUTPUT_EXE" "$BUILD_LOG"
}
trap cleanup EXIT

if [ ! -f "$INPUT" ]; then
    test_fail "phase29x_llvm_cabi_link_min: fixture missing: $INPUT"
    exit 1
fi

set +e
BUILD_OUT=$(
  NYASH_DISABLE_PLUGINS=1 \
  timeout "$RUN_TIMEOUT_SECS" \
  "$NYASH_ROOT/tools/build_llvm.sh" "$INPUT" -o "$OUTPUT_EXE" 2>&1
)
BUILD_RC=$?
set -e

echo "$BUILD_OUT" >"$BUILD_LOG"

if [ "$BUILD_RC" -eq 124 ]; then
    test_fail "phase29x_llvm_cabi_link_min: build timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$BUILD_RC" -ne 0 ]; then
    echo "[INFO] build output tail:"
    tail -n 80 "$BUILD_LOG" || true
    test_fail "phase29x_llvm_cabi_link_min: LLVM link build failed (rc=$BUILD_RC)"
    exit 1
fi
if [ ! -x "$OUTPUT_EXE" ]; then
    test_fail "phase29x_llvm_cabi_link_min: linked executable missing: $OUTPUT_EXE"
    exit 1
fi

set +e
RUN_OUT=$(timeout "$RUN_TIMEOUT_SECS" "$OUTPUT_EXE" 2>&1)
RUN_RC=$?
set -e

if [ "$RUN_RC" -eq 124 ]; then
    test_fail "phase29x_llvm_cabi_link_min: executable run timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$RUN_RC" -ne 0 ]; then
    echo "[INFO] executable output:"
    echo "$RUN_OUT" | tail -n 80 || true
    test_fail "phase29x_llvm_cabi_link_min: executable exited non-zero (rc=$RUN_RC)"
    exit 1
fi
if ! echo "$RUN_OUT" | rg -q '^42$'; then
    echo "[INFO] executable output:"
    echo "$RUN_OUT" | tail -n 80 || true
    test_fail "phase29x_llvm_cabi_link_min: expected output line '42' not observed"
    exit 1
fi

test_pass "phase29x_llvm_cabi_link_min: PASS (.hako -> LLVM -> C ABI link minimal gate)"

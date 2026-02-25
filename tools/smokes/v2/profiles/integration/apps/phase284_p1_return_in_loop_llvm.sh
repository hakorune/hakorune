#!/bin/bash
# Phase 284 P1: Return in loop test (LLVM EXE)
# Expected: Exit code 7 from early return

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/llvm_exe_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

llvm_exe_preflight_or_skip || exit 0

# No plugins required for this minimal test
INPUT_HAKO="$NYASH_ROOT/apps/tests/phase284_p1_return_in_loop_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase284_p1_return_in_loop_llvm_exe"

EXPECTED_EXIT_CODE=7
LLVM_BUILD_LOG="/tmp/phase284_p1_return_in_loop_build.log"

if llvm_exe_build_and_run_expect_exit_code; then
  test_pass "phase284_p1_return_in_loop_llvm: exit code 7 (early return from loop)"
else
  exit 1
fi

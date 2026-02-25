#!/bin/bash
# Phase 146 P0: If condition unified lowering (LLVM EXE parity)
#
# Expected: exit code 7

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/llvm_exe_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

llvm_exe_preflight_or_skip || exit 0

# Minimal plugins (Integer comparisons)
INTEGERBOX_SO="$NYASH_ROOT/target/release/libnyash_integer_plugin.so"
LLVM_REQUIRED_PLUGINS=(
  "IntCellBox|$INTEGERBOX_SO|nyash-integer-plugin"
)
LLVM_PLUGIN_BUILD_LOG="/tmp/phase146_p0_if_cond_unified_llvm_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

INPUT_HAKO="$NYASH_ROOT/apps/tests/phase146_p0_if_cond_unified_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase146_p0_if_cond_unified_llvm_exe"

EXPECTED_EXIT_CODE=7
LLVM_BUILD_LOG="/tmp/phase146_p0_if_cond_unified_llvm_build.log"

if llvm_exe_build_and_run_expect_exit_code; then
  test_pass "phase146_p0_if_cond_unified_llvm_exe: exit code matches (7)"
else
  exit 1
fi


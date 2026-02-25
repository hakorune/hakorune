#!/bin/bash
# Phase 143 P0: loop(true) + if + break Normalized lowering (LLVM EXE parity)
#
# Verifies LLVM EXE execution produces same result as VM for Phase 143 P0 pattern.
# Expected: exit code 7 (parity with VM test)
#
# Dev-only: NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/llvm_exe_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

# Preflight check (SKIP gate)
llvm_exe_preflight_or_skip || exit 0

# JoinIR dev mode (Phase 130+ gate)
require_joinir_dev

# Minimal plugins (Integer ops for comparisons)
INTEGERBOX_SO="$NYASH_ROOT/plugins/nyash-integer-plugin/libnyash_integer_plugin.so"
LLVM_REQUIRED_PLUGINS=(
  "IntegerBox|$INTEGERBOX_SO|nyash-integer-plugin"
)
LLVM_PLUGIN_BUILD_LOG="/tmp/phase143_loop_true_if_break_llvm_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

# Test configuration
INPUT_HAKO="$NYASH_ROOT/apps/tests/phase143_loop_true_if_break_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase143_loop_true_if_break_llvm_exe"

# Execute (exit code contract)
EXPECTED_EXIT_CODE=7
LLVM_BUILD_LOG="/tmp/phase143_loop_true_if_break_llvm_build.log"
if llvm_exe_build_and_run_expect_exit_code; then
    test_pass "phase143_loop_true_if_break_llvm_exe: exit code matches (7)"
else
    exit 1
fi

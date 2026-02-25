#!/bin/bash
# Phase 143 P1: loop(true) + if + continue Normalized lowering (LLVM EXE parity)
#
# Verifies LLVM EXE build+run for Phase 143 P1 pattern.
#
# This fixture is intentionally non-terminating in Phase 143 P1, so we use
# a timeout-based contract (expected timeout exit code 124).
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
INTCELLBOX_SO="$NYASH_ROOT/target/release/libnyash_integer_plugin.so"
LLVM_REQUIRED_PLUGINS=(
  "IntCellBox|$INTCELLBOX_SO|nyash-integer-plugin"
)
LLVM_PLUGIN_BUILD_LOG="/tmp/phase143_loop_true_if_continue_llvm_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

# Test configuration
INPUT_HAKO="$NYASH_ROOT/apps/tests/phase143_loop_true_if_continue_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase143_loop_true_if_continue_llvm_exe"

# Execute (timeout contract)
RUN_TIMEOUT_SECS="${SMOKES_P143_CONTINUE_TIMEOUT_SECS:-1}"
EXPECTED_EXIT_CODE=124
LLVM_BUILD_LOG="/tmp/phase143_loop_true_if_continue_llvm_build.log"
if llvm_exe_build_and_run_expect_exit_code; then
    test_pass "phase143_loop_true_if_continue_llvm_exe: timed out as expected (${RUN_TIMEOUT_SECS}s)"
else
    exit 1
fi

#!/bin/bash
# Phase 143 P2: loop(true) if break-continue (B-C, Normalized shadow, LLVM EXE parity)
#
# Verifies Phase 143 P2 else symmetry:
# - loop(true) { if flag==1 {break} else {continue} } ; return 8 → exit code 8
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

# Minimal plugins (String + Integer for comparisons and arithmetic)
STRINGBOX_SO="$NYASH_ROOT/plugins/nyash-string-plugin/libnyash_string_plugin.so"
INTCELLBOX_SO="$NYASH_ROOT/target/release/libnyash_integer_plugin.so"
LLVM_REQUIRED_PLUGINS=(
  "StringBox|$STRINGBOX_SO|nyash-string-plugin"
  "IntCellBox|$INTCELLBOX_SO|nyash-integer-plugin"
)
LLVM_PLUGIN_BUILD_LOG="/tmp/phase143_p2_loop_true_if_bc_llvm_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

INPUT_HAKO="$NYASH_ROOT/apps/tests/phase143_p2_loop_true_if_bc_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase143_p2_loop_true_if_bc_llvm_exe"

EXPECTED_EXIT_CODE=8
LLVM_BUILD_LOG="/tmp/phase143_p2_loop_true_if_bc_llvm_build.log"

if llvm_exe_build_and_run_expect_exit_code; then
  test_pass "phase143_p2_loop_true_if_bc_llvm_exe: exit code matches (8)"
else
  exit 1
fi


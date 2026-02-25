#!/bin/bash
# Phase 141 P1: known intrinsic method call in ExprLowerer (LLVM EXE parity)
# Pattern: s="abc"; if flag==1 {s=s} else {s=s}; return s.length() (should be 3)

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/llvm_exe_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

llvm_exe_preflight_or_skip || exit 0

# Phase 141 P1 is a dev-only Normalized shadow case.
require_joinir_dev

# Minimal plugin set (StringBox + IntegerBox)
STRINGBOX_SO="$NYASH_ROOT/plugins/nyash-string-plugin/libnyash_string_plugin.so"
INTEGERBOX_SO="$NYASH_ROOT/target/release/libnyash_integer_plugin.so"

LLVM_REQUIRED_PLUGINS=(
  "StringBox|$STRINGBOX_SO|nyash-string-plugin"
  "IntCellBox|$INTEGERBOX_SO|nyash-integer-plugin"
)
LLVM_PLUGIN_BUILD_LOG="/tmp/phase141_p1_if_only_post_k_return_length_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

INPUT_HAKO="$NYASH_ROOT/apps/tests/phase141_p1_if_only_post_k_return_length_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase141_p1_if_only_post_k_return_length_llvm_exe"

EXPECTED_EXIT_CODE=3
LLVM_BUILD_LOG="/tmp/phase141_p1_if_only_post_k_return_length_build.log"
if llvm_exe_build_and_run_expect_exit_code; then
  test_pass "phase141_p1_if_only_post_k_return_length_llvm_exe: exit code matches expected (3)"
else
  exit 1
fi

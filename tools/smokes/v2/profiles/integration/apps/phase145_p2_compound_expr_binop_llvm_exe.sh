#!/bin/bash
# Phase 145 P2: Recursive ANF with nested BinaryOp + MethodCall (LLVM EXE parity)
#
# Pattern: x + s.length() + z → exit code 18 (10 + 5 + 3)
# Dev-only: HAKO_ANF_DEV=1

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/llvm_exe_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

# Preflight check (SKIP gate)
llvm_exe_preflight_or_skip || exit 0

# Minimal plugins (String + Integer for s.length() and arithmetic)
STRINGBOX_SO="$NYASH_ROOT/plugins/nyash-string-plugin/libnyash_string_plugin.so"
INTEGERBOX_SO="$NYASH_ROOT/plugins/nyash-integer-plugin/libnyash_integer_plugin.so"
LLVM_REQUIRED_PLUGINS=(
  "StringBox|$STRINGBOX_SO|nyash-string-plugin"
  "IntegerBox|$INTEGERBOX_SO|nyash-integer-plugin"
)
LLVM_PLUGIN_BUILD_LOG="/tmp/phase145_p2_compound_expr_binop_llvm_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

INPUT_HAKO="$NYASH_ROOT/apps/tests/phase145_p2_compound_expr_binop_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase145_p2_compound_expr_binop_llvm_exe"

EXPECTED_EXIT_CODE=18
LLVM_BUILD_LOG="/tmp/phase145_p2_compound_expr_binop_llvm_build.log"

export HAKO_ANF_DEV=1

if llvm_exe_build_and_run_expect_exit_code; then
  test_pass "phase145_p2_compound_expr_binop_llvm_exe: exit code matches (18)"
else
  exit 1
fi


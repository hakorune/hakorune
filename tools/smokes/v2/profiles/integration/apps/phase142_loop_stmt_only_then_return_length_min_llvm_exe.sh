#!/bin/bash
# Phase 142 P0: Loop normalization as single statement (LLVM EXE parity)
#
# Verifies that loop(true) normalization returns consumed=1, allowing
# subsequent statements (return s.length()) to be processed normally.
# Expected: exit code 3 (s="abc" → s.length() → 3)
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

# Minimal plugins (String + Integer for s.length())
STRINGBOX_SO="$NYASH_ROOT/plugins/nyash-string-plugin/libnyash_string_plugin.so"
INTCELLBOX_SO="$NYASH_ROOT/target/release/libnyash_integer_plugin.so"
LLVM_REQUIRED_PLUGINS=(
  "StringBox|$STRINGBOX_SO|nyash-string-plugin"
  "IntCellBox|$INTCELLBOX_SO|nyash-integer-plugin"
)
LLVM_PLUGIN_BUILD_LOG="/tmp/phase142_loop_stmt_llvm_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

# Test configuration
INPUT_HAKO="$NYASH_ROOT/apps/tests/phase142_loop_stmt_only_then_return_length_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase142_loop_stmt_only_then_return_length_min_llvm_exe"

# Execute (exit code contract)
EXPECTED_EXIT_CODE=3
LLVM_BUILD_LOG="/tmp/phase142_loop_stmt_llvm_build.log"
if llvm_exe_build_and_run_expect_exit_code; then
  test_pass "phase142_loop_stmt_llvm_exe: exit code matches (3)"
else
  exit 1
fi

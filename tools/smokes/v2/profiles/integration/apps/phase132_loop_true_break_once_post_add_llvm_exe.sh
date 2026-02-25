#!/bin/bash
# Phase 132-P4: loop(true) break-once with post-loop computation (LLVM EXE parity)
# Pattern: loop(true) { x = 1; break }; x = x + 2; return x (should be 3)

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/llvm_exe_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

llvm_exe_preflight_or_skip || exit 0

# Phase 132-P4 is a dev-only Normalized shadow loop + post case.
# LLVM EXE emission must run with JoinIR dev/strict enabled, otherwise it will freeze.
require_joinir_dev

# Phase 132-P4: minimal plugin set (StringBox, ConsoleBox, IntegerBox only)
STRINGBOX_SO="$NYASH_ROOT/plugins/nyash-string-plugin/libnyash_string_plugin.so"
CONSOLEBOX_SO="$NYASH_ROOT/plugins/nyash-console-plugin/libnyash_console_plugin.so"
INTEGERBOX_SO="$NYASH_ROOT/plugins/nyash-integer-plugin/libnyash_integer_plugin.so"

LLVM_REQUIRED_PLUGINS=(
  "StringBox|$STRINGBOX_SO|nyash-string-plugin"
  "ConsoleBox|$CONSOLEBOX_SO|nyash-console-plugin"
  "IntegerBox|$INTEGERBOX_SO|nyash-integer-plugin"
)
LLVM_PLUGIN_BUILD_LOG="/tmp/phase132_loop_true_break_once_post_add_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

INPUT_HAKO="$NYASH_ROOT/apps/tests/phase132_loop_true_break_once_post_add_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase132_loop_true_break_once_post_add_llvm_exe"

EXPECTED_EXIT_CODE=3
LLVM_BUILD_LOG="/tmp/phase132_loop_true_break_once_post_add_build.log"
if llvm_exe_build_and_run_expect_exit_code; then
  test_pass "phase132_loop_true_break_once_post_add_llvm_exe: exit code matches expected (3)"
else
  exit 1
fi

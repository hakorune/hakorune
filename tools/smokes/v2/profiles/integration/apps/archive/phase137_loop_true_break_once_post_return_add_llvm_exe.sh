#!/bin/bash
# Phase 137 P0: loop(true) break-once with post-loop assigns + return add (LLVM EXE parity)
# Pattern: x = 0 → loop(true) { x = 1; break } → x = x + 10 → return x + 2 (should be 13)

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/llvm_exe_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

llvm_exe_preflight_or_skip || exit 0
require_joinir_dev

# Phase 137: minimal plugin set (StringBox, ConsoleBox, IntCellBox only)
STRINGBOX_SO="$NYASH_ROOT/plugins/nyash-string-plugin/libnyash_string_plugin.so"
CONSOLEBOX_SO="$NYASH_ROOT/plugins/nyash-console-plugin/libnyash_console_plugin.so"
INTCELLBOX_SO="$NYASH_ROOT/target/release/libnyash_integer_plugin.so"

LLVM_REQUIRED_PLUGINS=(
  "StringBox|$STRINGBOX_SO|nyash-string-plugin"
  "ConsoleBox|$CONSOLEBOX_SO|nyash-console-plugin"
  "IntCellBox|$INTCELLBOX_SO|nyash-integer-plugin"
)
LLVM_PLUGIN_BUILD_LOG="/tmp/phase137_loop_true_break_once_post_return_add_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

INPUT_HAKO="$NYASH_ROOT/apps/tests/phase137_loop_true_break_once_post_return_add_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase137_loop_true_break_once_post_return_add_llvm_exe"

EXPECTED_EXIT_CODE=13
LLVM_BUILD_LOG="/tmp/phase137_loop_true_break_once_post_return_add_build.log"
if llvm_exe_build_and_run_expect_exit_code; then
  test_pass "phase137_loop_true_break_once_post_return_add_llvm_exe: exit code matches expected (13)"
else
  exit 1
fi

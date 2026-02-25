#!/bin/bash
# Phase 131 P0: loop(true) break-once (LLVM EXE parity)
# Pattern: loop(true) { x = 1; break } → return x (should be 1)

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/llvm_exe_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

llvm_exe_preflight_or_skip || exit 0

# Phase 131 is a dev-only Normalized shadow loop case.
# LLVM EXE emission must run with JoinIR dev/strict enabled, otherwise it will freeze.
require_joinir_dev

# Phase 131: minimal plugin set (StringBox, ConsoleBox, IntegerBox only)
STRINGBOX_SO="$NYASH_ROOT/plugins/nyash-string-plugin/libnyash_string_plugin.so"
CONSOLEBOX_SO="$NYASH_ROOT/plugins/nyash-console-plugin/libnyash_console_plugin.so"
INTEGERBOX_SO="$NYASH_ROOT/target/release/libnyash_integer_plugin.so"

LLVM_REQUIRED_PLUGINS=(
  "StringBox|$STRINGBOX_SO|nyash-string-plugin"
  "ConsoleBox|$CONSOLEBOX_SO|nyash-console-plugin"
  "IntCellBox|$INTEGERBOX_SO|nyash-integer-plugin"
)
LLVM_PLUGIN_BUILD_LOG="/tmp/phase131_loop_true_break_once_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

INPUT_HAKO="$NYASH_ROOT/apps/tests/phase131_loop_true_break_once_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase131_loop_true_break_once_llvm_exe"

EXPECTED_EXIT_CODE=1
LLVM_BUILD_LOG="/tmp/phase131_loop_true_break_once_build.log"
if llvm_exe_build_and_run_expect_exit_code; then
  test_pass "phase131_loop_true_break_once_llvm_exe: exit code matches expected (1)"
else
  exit 1
fi

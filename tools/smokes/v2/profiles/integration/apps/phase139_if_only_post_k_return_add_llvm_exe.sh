#!/bin/bash
# Phase 139 P0: if-only post_k return add (LLVM EXE parity)
# Pattern: x=1; if flag==1 { x=2 } else { x=1 }; return x+2 (should be 4)

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/llvm_exe_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

llvm_exe_preflight_or_skip || exit 0

# Phase 139 is a dev-only Normalized shadow post-if case.
require_joinir_dev

# Minimal plugin set (StringBox, ConsoleBox, IntegerBox only)
STRINGBOX_SO="$NYASH_ROOT/plugins/nyash-string-plugin/libnyash_string_plugin.so"
CONSOLEBOX_SO="$NYASH_ROOT/plugins/nyash-console-plugin/libnyash_console_plugin.so"
INTEGERBOX_SO="$NYASH_ROOT/target/release/libnyash_integer_plugin.so"

LLVM_REQUIRED_PLUGINS=(
  "StringBox|$STRINGBOX_SO|nyash-string-plugin"
  "ConsoleBox|$CONSOLEBOX_SO|nyash-console-plugin"
  "IntCellBox|$INTEGERBOX_SO|nyash-integer-plugin"
)
LLVM_PLUGIN_BUILD_LOG="/tmp/phase139_if_only_post_k_return_add_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

INPUT_HAKO="$NYASH_ROOT/apps/tests/phase139_if_only_post_k_return_add_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase139_if_only_post_k_return_add_llvm_exe"

EXPECTED_EXIT_CODE=4
LLVM_BUILD_LOG="/tmp/phase139_if_only_post_k_return_add_build.log"
if llvm_exe_build_and_run_expect_exit_code; then
  test_pass "phase139_if_only_post_k_return_add_llvm_exe: exit code matches expected (4)"
else
  exit 1
fi


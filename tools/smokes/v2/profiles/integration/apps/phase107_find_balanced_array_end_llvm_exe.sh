#!/bin/bash
# Phase 107: find_balanced_array_end real-app derived (LLVM EXE parity)

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/llvm_exe_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

llvm_exe_preflight_or_skip || exit 0

# Phase 97/98/100 SSOT: plugin dlopen check → build only if needed → dlopen recheck.
FILEBOX_SO="$NYASH_ROOT/plugins/nyash-filebox-plugin/libnyash_filebox_plugin.so"
MAPBOX_SO="$NYASH_ROOT/plugins/nyash-map-plugin/libnyash_map_plugin.so"
STRINGBOX_SO="$NYASH_ROOT/plugins/nyash-string-plugin/libnyash_string_plugin.so"
CONSOLEBOX_SO="$NYASH_ROOT/plugins/nyash-console-plugin/libnyash_console_plugin.so"
INTEGERBOX_SO="$NYASH_ROOT/plugins/nyash-integer-plugin/libnyash_integer_plugin.so"

LLVM_REQUIRED_PLUGINS=(
  "FileBox|$FILEBOX_SO|nyash-filebox-plugin"
  "MapBox|$MAPBOX_SO|nyash-map-plugin"
  "StringBox|$STRINGBOX_SO|nyash-string-plugin"
  "ConsoleBox|$CONSOLEBOX_SO|nyash-console-plugin"
  "IntegerBox|$INTEGERBOX_SO|nyash-integer-plugin"
)
LLVM_PLUGIN_BUILD_LOG="/tmp/phase107_find_balanced_array_end_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

INPUT_HAKO="$NYASH_ROOT/apps/tests/phase107_find_balanced_array_end_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase107_find_balanced_array_end_llvm_exe"

EXPECTED=$'1\n3'
EXPECTED_LINES=2
LLVM_BUILD_LOG="/tmp/phase107_find_balanced_array_end_build.log"
if llvm_exe_build_and_run_numeric_smoke; then
  test_pass "phase107_find_balanced_array_end_llvm_exe: output matches expected (1, 3)"
else
  exit 1
fi


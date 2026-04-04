#!/bin/bash
# Phase 103 P1: if-only early return regression (LLVM EXE parity)

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
INTCELLBOX_SO="$NYASH_ROOT/target/release/libnyash_integer_plugin.so"

LLVM_REQUIRED_PLUGINS=(
  "FileBox|$FILEBOX_SO|nyash-filebox-plugin"
  "MapBox|$MAPBOX_SO|nyash-map-plugin"
  "StringBox|$STRINGBOX_SO|nyash-string-plugin"
  "ConsoleBox|$CONSOLEBOX_SO|nyash-console-plugin"
  "IntCellBox|$INTCELLBOX_SO|nyash-integer-plugin"
)
LLVM_PLUGIN_BUILD_LOG="/tmp/phase103_if_only_early_return_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

INPUT_HAKO="$NYASH_ROOT/apps/tests/phase103_if_only_early_return_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase103_if_only_early_return_llvm_exe"

EXPECTED=$'7\n2'
EXPECTED_LINES=2
LLVM_BUILD_LOG="/tmp/phase103_if_only_early_return_build.log"
export HAKO_BACKEND_COMPAT_REPLAY=harness
if llvm_exe_build_and_run_numeric_smoke; then
  test_pass "phase103_if_only_early_return_llvm_exe: output matches expected (7, 2)"
else
  exit 1
fi

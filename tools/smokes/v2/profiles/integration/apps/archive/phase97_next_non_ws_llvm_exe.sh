#!/bin/bash
# Phase 97: next_non_ws fixture (LLVM EXE parity)
#
# Current boundary contract:
# - pure-first default still rejects this fixture shape
# - Phase 97 pins explicit compat replay (`harness`) so the remaining blocker is
#   LLVM EXE runtime parity, not recipe selection

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/llvm_exe_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

llvm_exe_preflight_or_skip || exit 0
export HAKO_JOINIR_STRICT=1

FILEBOX_SO="$NYASH_ROOT/plugins/nyash-filebox-plugin/libnyash_filebox_plugin.so"
MAPBOX_SO="$NYASH_ROOT/plugins/nyash-map-plugin/libnyash_map_plugin.so"

LLVM_REQUIRED_PLUGINS=(
  "FileBox|$FILEBOX_SO|nyash-filebox-plugin"
  "MapBox|$MAPBOX_SO|nyash-map-plugin"
)
LLVM_PLUGIN_BUILD_LOG="/tmp/phase97_next_non_ws_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

INPUT_HAKO="$NYASH_ROOT/apps/tests/phase96_json_loader_next_non_ws_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase97_next_non_ws_llvm_exe"

EXPECTED=$'2\n-1\n3'
EXPECTED_LINES=3
LLVM_BUILD_LOG="/tmp/phase97_next_non_ws_build.log"
export HAKO_BACKEND_COMPAT_REPLAY=harness
if llvm_exe_build_and_run_numeric_smoke; then
  test_pass "phase97_next_non_ws_llvm_exe: output matches expected (2, -1, 3)"
else
  exit 1
fi

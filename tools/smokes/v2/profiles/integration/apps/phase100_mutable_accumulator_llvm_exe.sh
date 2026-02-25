#!/bin/bash
# Phase 100 P2.1 - Mutable Accumulator (LLVM EXE backend)
# Tests: out = out + ch (string accumulator) and count = count + 1 (integer accumulator)

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/llvm_exe_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

llvm_exe_preflight_or_skip || exit 0

FILEBOX_SO="$NYASH_ROOT/plugins/nyash-filebox-plugin/libnyash_filebox_plugin.so"
MAPBOX_SO="$NYASH_ROOT/plugins/nyash-map-plugin/libnyash_map_plugin.so"
LLVM_REQUIRED_PLUGINS=(
  "FileBox|$FILEBOX_SO|nyash-filebox-plugin"
  "MapBox|$MAPBOX_SO|nyash-map-plugin"
)
LLVM_PLUGIN_BUILD_LOG="/tmp/phase100_mutable_accumulator_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

INPUT_HAKO="$NYASH_ROOT/apps/tests/phase100_mutable_accumulator_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase100_mutable_accumulator_min"

EXPECTED=$'3'
EXPECTED_LINES=1
LLVM_BUILD_LOG="/tmp/phase100_mutable_accumulator_build.log"
if llvm_exe_build_and_run_numeric_smoke; then
  test_pass "phase100_mutable_accumulator_llvm_exe: output matches expected (3)"
else
  exit 1
fi

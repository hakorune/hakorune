#!/bin/bash
# Phase 99: escape trailing backslash fixture (LLVM EXE parity)
#
# Current boundary contract:
# - pure-first default still rejects this fixture shape
# - Phase 97/98 parity lane pins explicit compat replay (`harness`) so the
#   remaining blocker stays in LLVM EXE runtime, not recipe selection

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
LLVM_PLUGIN_BUILD_LOG="/tmp/phase99_escape_trailing_backslash_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

mkdir -p "$NYASH_ROOT/tmp"

INPUT_HAKO="$NYASH_ROOT/apps/tests/phase99_json_loader_escape_trailing_backslash_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase99_escape_trailing_backslash_llvm_exe"

echo "[INFO] Building: $INPUT_HAKO → $OUTPUT_EXE"

BUILD_LOG="/tmp/phase99_escape_trailing_backslash_build.log"
if ! llvm_exe_with_build_lock env HAKO_BACKEND_COMPAT_REPLAY=harness NYASH_DISABLE_PLUGINS=0 "$NYASH_ROOT/tools/build_llvm.sh" "$INPUT_HAKO" -o "$OUTPUT_EXE" 2>&1 | tee "$BUILD_LOG"; then
    echo "[FAIL] build_llvm.sh failed"
    tail -n 80 "$BUILD_LOG"
    exit 1
fi

if [ ! -x "$OUTPUT_EXE" ]; then
    echo "[FAIL] Executable not created or not executable: $OUTPUT_EXE"
    ls -la "$OUTPUT_EXE" 2>/dev/null || echo "File does not exist"
    exit 1
fi

echo "[INFO] Executing: $OUTPUT_EXE"

set +e
OUTPUT=$(timeout "${RUN_TIMEOUT_SECS:-10}" env NYASH_DISABLE_PLUGINS=0 "$OUTPUT_EXE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -ne 0 ]; then
    echo "[FAIL] Execution failed with exit code $EXIT_CODE"
    echo "$OUTPUT" | tail -n 80
    exit 1
fi

# Current behavior: trailing backslash is preserved in output
CLEAN=$(llvm_exe_first_payload_line "$OUTPUT")
EXPECTED="hello\\"

echo "[INFO] CLEAN output:"
echo "$CLEAN"

if [ "$CLEAN" = "$EXPECTED" ]; then
    test_pass "phase99_escape_trailing_backslash_llvm_exe: output matches expected (hello\\)"
else
    echo "[FAIL] Output mismatch"
    echo "[INFO] Raw output (tail):"
    echo "$OUTPUT" | tail -n 80
    echo "[INFO] Expected:"
    printf "%s\n" "$EXPECTED"
    exit 1
fi

#!/bin/bash
# Phase 97: json_loader escape loop (LLVM EXE parity)

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
LLVM_PLUGIN_BUILD_LOG="/tmp/phase97_json_loader_escape_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

mkdir -p "$NYASH_ROOT/tmp"

INPUT_HAKO="$NYASH_ROOT/apps/tests/phase95_json_loader_escape_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase97_json_loader_escape_llvm_exe"

echo "[INFO] Building: $INPUT_HAKO → $OUTPUT_EXE"

BUILD_LOG="/tmp/phase97_json_loader_escape_build.log"
if ! llvm_exe_with_build_lock env NYASH_DISABLE_PLUGINS=0 "$NYASH_ROOT/tools/build_llvm.sh" "$INPUT_HAKO" -o "$OUTPUT_EXE" 2>&1 | tee "$BUILD_LOG"; then
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

if printf "%s\n" "$OUTPUT" | grep -q '^hello" world$'; then
    test_pass "phase97_json_loader_escape_llvm_exe: output verified (hello\" world)"
else
    echo "[FAIL] Unexpected output (expected line: hello\" world)"
    echo "[INFO] output (tail):"
    echo "$OUTPUT" | tail -n 80
    exit 1
fi

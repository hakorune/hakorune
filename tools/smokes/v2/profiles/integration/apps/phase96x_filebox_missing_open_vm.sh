#!/bin/bash
# Phase 96x C1e: FileBox missing-open witness (VM/plugin route)
#
# Contract:
# 1) emit-mir-json must contain FileBox.open(path, mode) mir_call args=2/3.
# 2) plugin-backed VM route must execute the missing-file open path with rc=0.
# 3) timeout, FileBox plugin-init failure, and Unknown Box type regressions are forbidden.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/plugin_pilot_common.sh"
require_env || exit 2

SMOKE_NAME="phase96x_filebox_missing_open_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase96x_filebox_missing_open_min.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-60}"
WORK_DIR="$(mktemp -d -t phase96x_filebox_missing_open.XXXXXX)"
OUTPUT_FILE="$WORK_DIR/output.log"
EMIT_LOG="$WORK_DIR/emit.log"
MIR_JSON="$WORK_DIR/output.mir.json"

cleanup() {
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

require_fixture_file "$SMOKE_NAME" "$FIXTURE" || exit 1

EXT="$(detect_lib_ext)"
FILEBOX_LIB_NAME="$(lib_name_for nyash_filebox_plugin "$EXT")"
FILEBOX_LIB_PATH="$NYASH_ROOT/plugins/nyash-filebox-plugin/$FILEBOX_LIB_NAME"

if [ ! -f "$FILEBOX_LIB_PATH" ]; then
  build_plugin_release_checked "$SMOKE_NAME" "filebox" "$FILEBOX_LIB_PATH" -p nyash-filebox-plugin || exit 1
fi

append_filebox_toml "$FILEBOX_LIB_NAME" "$FILEBOX_LIB_PATH" 0 > "$WORK_DIR/nyash.toml"
cp "$FIXTURE" "$WORK_DIR/main.hako"
rm -f "$WORK_DIR/missing_input.txt"

set +e
(
  cd "$WORK_DIR"
  "$NYASH_BIN" --emit-mir-json "$MIR_JSON" ./main.hako >"$EMIT_LOG" 2>&1
)
EMIT_RC=$?
set -e

if [ "$EMIT_RC" -ne 0 ]; then
  tail -n 80 "$EMIT_LOG" || true
  test_fail "$SMOKE_NAME: emit-mir-json failed rc=$EMIT_RC"
  exit 1
fi

if ! jq -e '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select((.op=="mir_call" and .mir_call.callee.type=="Method" and .mir_call.callee.box_name=="FileBox" and .mir_call.callee.name=="open" and (((.mir_call.args|length)==2) or ((.mir_call.args|length)==3))) or (.op=="boxcall" and .method=="open" and (((.args|length)==2) or ((.args|length)==3))))' "$MIR_JSON" >/dev/null; then
  test_fail "$SMOKE_NAME: MIR missing FileBox.open(path,mode) mir_call args=2/3 shape"
  exit 1
fi

set +e
(
  cd "$WORK_DIR"
  timeout "$RUN_TIMEOUT_SECS" env \
    NYASH_DISABLE_PLUGINS=0 \
    NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
    NYASH_VM_USE_FALLBACK=0 \
    "$NYASH_BIN" --backend vm ./main.hako >"$OUTPUT_FILE" 2>&1
)
RC=$?
set -e

if [ "$RC" -eq 124 ]; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: timed out (> ${RUN_TIMEOUT_SECS}s)"
  exit 1
fi
if [ "$RC" -ne 0 ]; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected rc=0 for missing-file open path (got rc=$RC)"
  exit 1
fi
if grep -q 'Unknown Box type: FileBox' "$OUTPUT_FILE"; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: stale Unknown Box type: FileBox regression"
  exit 1
fi
if grep -q '\[plugin/init\]' "$OUTPUT_FILE"; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: plugin init failed"
  exit 1
fi
if grep -q 'FileBox plugin is required' "$OUTPUT_FILE"; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: stale FileBox plugin required hint remained"
  exit 1
fi
if ! grep -q '^RC: 0$' "$OUTPUT_FILE"; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected RC: 0 marker"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (FileBox missing-open witness locked)"

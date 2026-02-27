#!/bin/bash
# Phase 29cc PLG-07-min3: FileBox binary parity smoke (.hako route / VM)
# Contract:
# - FileBox.readBytes route is callable from .hako-side parity fixture.
# - Route is pinned to strict-plugin-first provider policy.
# - Prepared payload bytes remain stable.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/plugin_pilot_common.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg07_filebox_binary_hako_route_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg07_filebox_binary_hako_route_min.hako"
WORK_DIR="$(mktemp -d -t phase29cc_plg07_hako_filebox.XXXXXX)"
OUTPUT_FILE="$WORK_DIR/output.log"

cleanup() {
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

if [ ! -f "$FIXTURE" ]; then
  test_fail "$SMOKE_NAME: fixture missing ($FIXTURE)"
  exit 1
fi

EXT="$(detect_lib_ext)"
FILEBOX_LIB_NAME="$(lib_name_for nyash_filebox_plugin "$EXT")"
FILEBOX_LIB_PATH="$NYASH_ROOT/plugins/nyash-filebox-plugin/$FILEBOX_LIB_NAME"

if [ ! -f "$FILEBOX_LIB_PATH" ]; then
  log_info "$SMOKE_NAME: filebox plugin not found, building release artifact"
  (cd "$NYASH_ROOT" && cargo build -p nyash-filebox-plugin --release >/dev/null)
fi

if [ ! -f "$FILEBOX_LIB_PATH" ]; then
  test_fail "$SMOKE_NAME: filebox plugin artifact missing ($FILEBOX_LIB_PATH)"
  exit 1
fi

append_filebox_toml "$FILEBOX_LIB_NAME" "$FILEBOX_LIB_PATH" 1 > "$WORK_DIR/nyash.toml"
cp "$FIXTURE" "$WORK_DIR/main.hako"
printf 'PLG07_BINARY_OK' > "$WORK_DIR/phase29cc_filebox_binary_payload.bin"

set +e
(
  cd "$WORK_DIR"
  NYASH_DISABLE_PLUGINS=0 \
  HAKO_PROVIDER_POLICY=strict-plugin-first \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  NYASH_VM_USE_FALLBACK=0 \
  "$NYASH_BIN" --backend vm ./main.hako >"$OUTPUT_FILE" 2>&1
)
RC=$?
set -e

if [ "$RC" -ne 0 ]; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: vm run failed rc=$RC"
  exit 1
fi

if ! grep -q 'file_bytes_hako=PLG07_BINARY_OK' "$OUTPUT_FILE"; then
  tail -n 80 "$OUTPUT_FILE" || true
  test_fail "$SMOKE_NAME: expected output 'file_bytes_hako=PLG07_BINARY_OK' not found"
  exit 1
fi

PAYLOAD_FILE="$WORK_DIR/phase29cc_filebox_binary_payload.bin"
if [ ! -f "$PAYLOAD_FILE" ]; then
  test_fail "$SMOKE_NAME: payload file not created ($PAYLOAD_FILE)"
  exit 1
fi

ACTUAL_PAYLOAD="$(cat "$PAYLOAD_FILE")"
if [ "$ACTUAL_PAYLOAD" != "PLG07_BINARY_OK" ]; then
  test_fail "$SMOKE_NAME: payload mismatch (expected PLG07_BINARY_OK, got '$ACTUAL_PAYLOAD')"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (.hako parity route locked)"

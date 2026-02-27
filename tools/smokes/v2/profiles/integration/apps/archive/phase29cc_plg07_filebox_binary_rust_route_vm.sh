#!/bin/bash
# Phase 29cc PLG-07-min2: FileBox binary route smoke (Rust plugin / VM)
# Contract:
# - FileBox.readBytes is wired in plugin method table.
# - VM keeps prepared payload bytes stable while readBytes route is exercised.
# - Route is pinned to strict-plugin-first provider policy.

set -euo pipefail

source "$(dirname "$0")/phase29cc_plg07_filebox_binary_common.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg07_filebox_binary_rust_route_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg07_filebox_binary_rust_route_min.hako"
EXPECTED_PAYLOAD="$(plg07_expected_payload)"

if [ ! -f "$FIXTURE" ]; then
  test_fail "$SMOKE_NAME: fixture missing ($FIXTURE)"
  exit 1
fi

resolved_artifact="$(plg07_ensure_filebox_plugin_artifact "$SMOKE_NAME")"
FILEBOX_LIB_NAME="${resolved_artifact%%|*}"
FILEBOX_LIB_PATH="${resolved_artifact#*|}"

actual_value="$(
  plg07_run_route_case \
    "$SMOKE_NAME" \
    "rust" \
    "$FIXTURE" \
    "file_bytes" \
    "$EXPECTED_PAYLOAD" \
    "$FILEBOX_LIB_NAME" \
    "$FILEBOX_LIB_PATH"
)"
if [ "$actual_value" != "$EXPECTED_PAYLOAD" ]; then
  test_fail "$SMOKE_NAME: expected output 'file_bytes=$EXPECTED_PAYLOAD', got '$actual_value'"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (FileBox binary route locked)"

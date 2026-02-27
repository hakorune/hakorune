#!/bin/bash
# Phase 29cc PLG-07-min3: FileBox binary parity smoke (.hako route / VM)
# Contract:
# - FileBox.readBytes route is callable from .hako-side parity fixture.
# - Route is pinned to strict-plugin-first provider policy.
# - Prepared payload bytes remain stable.

set -euo pipefail

source "$(dirname "$0")/phase29cc_plg07_filebox_binary_common.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg07_filebox_binary_hako_route_vm"
FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg07_filebox_binary_hako_route_min.hako"
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
    "hako" \
    "$FIXTURE" \
    "file_bytes_hako" \
    "$EXPECTED_PAYLOAD" \
    "$FILEBOX_LIB_NAME" \
    "$FILEBOX_LIB_PATH"
)"
if [ "$actual_value" != "$EXPECTED_PAYLOAD" ]; then
  test_fail "$SMOKE_NAME: expected output 'file_bytes_hako=$EXPECTED_PAYLOAD', got '$actual_value'"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (.hako parity route locked)"

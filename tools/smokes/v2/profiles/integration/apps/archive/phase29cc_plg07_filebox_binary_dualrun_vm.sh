#!/bin/bash
# Phase 29cc PLG-07-min4: FileBox binary dual-run parity smoke (VM)
# Contract:
# - Runs Rust-route fixture and .hako-route fixture with strict-plugin-first policy.
# - Locks output/payload parity between two routes.

set -euo pipefail

source "$(dirname "$0")/phase29cc_plg07_filebox_binary_common.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg07_filebox_binary_dualrun_vm"
RUST_FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg07_filebox_binary_rust_route_min.hako"
HAKO_FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg07_filebox_binary_hako_route_min.hako"
EXPECTED_PAYLOAD="$(plg07_expected_payload)"

require_fixture_file "$SMOKE_NAME" "$RUST_FIXTURE" || exit 1
require_fixture_file "$SMOKE_NAME" "$HAKO_FIXTURE" || exit 1
resolved_artifact="$(plg07_ensure_filebox_plugin_artifact "$SMOKE_NAME")"
FILEBOX_LIB_NAME="${resolved_artifact%%|*}"
FILEBOX_LIB_PATH="${resolved_artifact#*|}"

RUST_VALUE="$(
  plg07_run_route_case \
    "$SMOKE_NAME" \
    "rust" \
    "$RUST_FIXTURE" \
    "file_bytes" \
    "$EXPECTED_PAYLOAD" \
    "$FILEBOX_LIB_NAME" \
    "$FILEBOX_LIB_PATH"
)"
HAKO_VALUE="$(
  plg07_run_route_case \
    "$SMOKE_NAME" \
    "hako" \
    "$HAKO_FIXTURE" \
    "file_bytes_hako" \
    "$EXPECTED_PAYLOAD" \
    "$FILEBOX_LIB_NAME" \
    "$FILEBOX_LIB_PATH"
)"

if [ "$RUST_VALUE" != "$HAKO_VALUE" ]; then
  test_fail "$SMOKE_NAME: parity mismatch (rust=$RUST_VALUE, hako=$HAKO_VALUE)"
  exit 1
fi
if [ "$RUST_VALUE" != "$EXPECTED_PAYLOAD" ]; then
  test_fail "$SMOKE_NAME: unexpected parity value ($RUST_VALUE)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (dual-run parity locked)"

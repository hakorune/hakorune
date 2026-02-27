#!/bin/bash
# Phase 29cc PLG-07-min4: FileBox binary dual-run parity smoke (VM)
# Contract:
# - Runs Rust-route fixture and .hako-route fixture with strict-plugin-first policy.
# - Locks output/payload parity between two routes.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/plugin_pilot_common.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg07_filebox_binary_dualrun_vm"
RUST_FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg07_filebox_binary_rust_route_min.hako"
HAKO_FIXTURE="$NYASH_ROOT/apps/tests/phase29cc_plg07_filebox_binary_hako_route_min.hako"
EXPECTED_PAYLOAD="PLG07_BINARY_OK"

detect_artifact() {
  local ext
  ext="$(detect_lib_ext)"
  FILEBOX_LIB_NAME="$(lib_name_for nyash_filebox_plugin "$ext")"
  FILEBOX_LIB_PATH="$NYASH_ROOT/plugins/nyash-filebox-plugin/$FILEBOX_LIB_NAME"
}

ensure_plugin_artifact() {
  detect_artifact
  if [ ! -f "$FILEBOX_LIB_PATH" ]; then
    log_info "$SMOKE_NAME: filebox plugin not found, building release artifact"
    (cd "$NYASH_ROOT" && cargo build -p nyash-filebox-plugin --release >/dev/null)
  fi
  if [ ! -f "$FILEBOX_LIB_PATH" ]; then
    test_fail "$SMOKE_NAME: filebox plugin artifact missing ($FILEBOX_LIB_PATH)"
    exit 1
  fi
}

run_case() {
  local fixture="$1"
  local marker="$2"
  local case_name="$3"

  local work_dir output_file payload_file line value rc
  work_dir="$(mktemp -d -t "${SMOKE_NAME}.${case_name}.XXXXXX")"
  output_file="$work_dir/output.log"
  payload_file="$work_dir/phase29cc_filebox_binary_payload.bin"

  append_filebox_toml "$FILEBOX_LIB_NAME" "$FILEBOX_LIB_PATH" 1 > "$work_dir/nyash.toml"
  cp "$fixture" "$work_dir/main.hako"
  printf '%s' "$EXPECTED_PAYLOAD" > "$payload_file"

  set +e
  (
    cd "$work_dir"
    NYASH_DISABLE_PLUGINS=0 \
    HAKO_PROVIDER_POLICY=strict-plugin-first \
    NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
    NYASH_VM_USE_FALLBACK=0 \
    "$NYASH_BIN" --backend vm ./main.hako >"$output_file" 2>&1
  )
  rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then
    tail -n 80 "$output_file" || true
    rm -rf "$work_dir"
    test_fail "$SMOKE_NAME: ${case_name} run failed rc=$rc"
    exit 1
  fi

  if [ ! -f "$payload_file" ]; then
    rm -rf "$work_dir"
    test_fail "$SMOKE_NAME: ${case_name} payload file missing"
    exit 1
  fi
  if [ "$(cat "$payload_file")" != "$EXPECTED_PAYLOAD" ]; then
    rm -rf "$work_dir"
    test_fail "$SMOKE_NAME: ${case_name} payload mismatch"
    exit 1
  fi

  line="$(grep -E "^${marker}=" "$output_file" | tail -n 1 || true)"
  if [ -z "$line" ]; then
    tail -n 80 "$output_file" || true
    rm -rf "$work_dir"
    test_fail "$SMOKE_NAME: ${case_name} marker '${marker}=' not found"
    exit 1
  fi
  value="${line#*=}"
  rm -rf "$work_dir"
  printf '%s' "$value"
}

require_fixture_file "$SMOKE_NAME" "$RUST_FIXTURE" || exit 1
require_fixture_file "$SMOKE_NAME" "$HAKO_FIXTURE" || exit 1
ensure_plugin_artifact

RUST_VALUE="$(run_case "$RUST_FIXTURE" "file_bytes" "rust")"
HAKO_VALUE="$(run_case "$HAKO_FIXTURE" "file_bytes_hako" "hako")"

if [ "$RUST_VALUE" != "$HAKO_VALUE" ]; then
  test_fail "$SMOKE_NAME: parity mismatch (rust=$RUST_VALUE, hako=$HAKO_VALUE)"
  exit 1
fi
if [ "$RUST_VALUE" != "$EXPECTED_PAYLOAD" ]; then
  test_fail "$SMOKE_NAME: unexpected parity value ($RUST_VALUE)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (dual-run parity locked)"

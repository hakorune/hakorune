#!/bin/bash
set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/plugin_pilot_common.sh"

plg07_expected_payload() {
  printf '%s' "PLG07_BINARY_OK"
}

plg07_require_doc_keywords() {
  local smoke_name="$1"
  local doc_path="$2"
  shift 2

  if [ ! -f "$doc_path" ]; then
    test_fail "$smoke_name: lock doc missing ($doc_path)"
    exit 1
  fi

  local needle
  for needle in "$@"; do
    if ! grep -q "$needle" "$doc_path"; then
      test_fail "$smoke_name: missing keyword in lock doc: $needle"
      exit 1
    fi
  done
}

plg07_filebox_plugin_artifact_path() {
  local ext lib_name
  ext="$(detect_lib_ext)"
  lib_name="$(lib_name_for nyash_filebox_plugin "$ext")"
  printf '%s|%s' "$lib_name" "$NYASH_ROOT/plugins/nyash-filebox-plugin/$lib_name"
}

plg07_ensure_filebox_plugin_artifact() {
  local smoke_name="$1"
  local resolved lib_name lib_path
  resolved="$(plg07_filebox_plugin_artifact_path)"
  lib_name="${resolved%%|*}"
  lib_path="${resolved#*|}"

  if [ ! -f "$lib_path" ]; then
    log_info "$smoke_name: filebox plugin not found, building release artifact"
    (cd "$NYASH_ROOT" && cargo build -p nyash-filebox-plugin --release >/dev/null)
  fi
  if [ ! -f "$lib_path" ]; then
    test_fail "$smoke_name: filebox plugin artifact missing ($lib_path)"
    exit 1
  fi

  printf '%s|%s' "$lib_name" "$lib_path"
}

plg07_run_route_case() {
  local smoke_name="$1"
  local case_name="$2"
  local fixture="$3"
  local output_marker="$4"
  local expected_payload="$5"
  local filebox_lib_name="$6"
  local filebox_lib_path="$7"

  local work_dir output_file payload_file line value rc
  work_dir="$(mktemp -d -t "${smoke_name}.${case_name}.XXXXXX")"
  output_file="$work_dir/output.log"
  payload_file="$work_dir/phase29cc_filebox_binary_payload.bin"

  append_filebox_toml "$filebox_lib_name" "$filebox_lib_path" 1 > "$work_dir/nyash.toml"
  cp "$fixture" "$work_dir/main.hako"
  printf '%s' "$expected_payload" > "$payload_file"

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
    test_fail "$smoke_name: ${case_name} run failed rc=$rc"
    exit 1
  fi

  if [ ! -f "$payload_file" ]; then
    rm -rf "$work_dir"
    test_fail "$smoke_name: ${case_name} payload file missing"
    exit 1
  fi
  if [ "$(cat "$payload_file")" != "$expected_payload" ]; then
    rm -rf "$work_dir"
    test_fail "$smoke_name: ${case_name} payload mismatch"
    exit 1
  fi

  line="$(grep -E "^${output_marker}=" "$output_file" | tail -n 1 || true)"
  if [ -z "$line" ]; then
    tail -n 80 "$output_file" || true
    rm -rf "$work_dir"
    test_fail "$smoke_name: ${case_name} marker '${output_marker}=' not found"
    exit 1
  fi

  value="${line#*=}"
  rm -rf "$work_dir"
  printf '%s' "$value"
}

plg07_run_retire_readiness_evidence() {
  bash "$NYASH_ROOT/tools/checks/phase29cc_plg07_filebox_binary_default_switch_guard.sh"
  bash "$NYASH_ROOT/tools/checks/phase29cc_plg07_filebox_binary_dualrun_guard.sh"
  bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_hako_route_vm.sh"
}

#!/bin/bash
# joinir_route_report.sh - stable route report helper for JoinIR extension probes.

set -euo pipefail

joinir_route_report_emit() {
  local vm_lane="$1"
  local kernel_lane="$2"
  local joinir_lane="$3"
  local emit_lane="$4"
  local fallback="${NYASH_VM_USE_FALLBACK:-0}"

  echo "[route/joinir] vm=${vm_lane} kernel=${kernel_lane} joinir=${joinir_lane} fallback=${fallback} emit=${emit_lane}" >&2
}

joinir_route_report_require_no_fallback() {
  local fallback="${NYASH_VM_USE_FALLBACK:-0}"
  if [ "$fallback" != "0" ]; then
    echo "[FAIL] route report requires NYASH_VM_USE_FALLBACK=0 (actual=${fallback})" >&2
    return 1
  fi
  return 0
}

joinir_route_report_require_lane_tag() {
  local output="$1"
  local expected_lane="$2"

  if ! grep -q "^\\[vm-route/select\\] backend=vm lane=${expected_lane} " <<<"$output"; then
    echo "[FAIL] missing vm-route/select lane tag: lane=${expected_lane}" >&2
    return 1
  fi
  return 0
}

#!/usr/bin/env bash
set -euo pipefail

run_llvmlite_provider_stopline_case() {
  local smoke_name="$1"
  local input_mir="$2"

  local script_dir root root_git
  script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  if root_git=$(git -C "$script_dir" rev-parse --show-toplevel 2>/dev/null); then
    root="$root_git"
  else
    root="$(cd "$script_dir/../../../../../../../../.." && pwd)"
  fi

  # shellcheck source=/dev/null
  source "$root/tools/smokes/v2/lib/test_runner.sh"
  require_env || return 2

  if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "$smoke_name: LLVM backend not available"
    return 0
  fi

  if [ ! -f "$input_mir" ]; then
    test_fail "$smoke_name: fixture missing: $input_mir"
    return 1
  fi

  if ! command -v jq >/dev/null 2>&1; then
    test_skip "$smoke_name: jq not available"
    return 0
  fi

  local tmp_hako run_out run_rc obj_path mir_json_q
  tmp_hako="$(mktemp --suffix .hako)"

  mir_json_q="$(jq -Rs . "$input_mir")"

  cat >"$tmp_hako" <<HAKO
using selfhost.shared.backend.llvm_backend_evidence_adapter as LlvmBackendEvidenceAdapterBox

static box Main {
  method main(args) {
    local obj = LlvmBackendEvidenceAdapterBox.compile_obj_provider_stopline($mir_json_q)
    if obj == null || obj == "" { return 91 }
    print("OBJ=" + obj)
    return 0
  }
}
HAKO

  set +e
  run_out=$(
    NYASH_LLVM_USE_CAPI=1 \
    HAKO_V1_EXTERN_PROVIDER_C_ABI=1 \
    HAKO_LLVM_EMIT_PROVIDER=llvmlite \
    timeout 120 \
    "$NYASH_ROOT/target/release/hakorune" --backend vm-hako "$tmp_hako" 2>&1
  )
  run_rc=$?
  set -e

  if [ "$run_rc" -eq 124 ]; then
    rm -f "$tmp_hako"
    test_fail "$smoke_name: vm-hako run timed out"
    return 1
  fi
  if [ "$run_rc" -ne 0 ]; then
    echo "[INFO] vm-hako output:"
    echo "$run_out" | tail -n 120 || true
    rm -f "$tmp_hako"
    test_fail "$smoke_name: vm-hako caller failed (rc=$run_rc)"
    return 1
  fi

  obj_path="$(printf '%s\n' "$run_out" | sed -n 's/^OBJ=//p' | tail -n1 | tr -d '\r')"
  if [ -z "$obj_path" ]; then
    echo "[INFO] vm-hako output:"
    echo "$run_out" | tail -n 120 || true
    rm -f "$tmp_hako"
    test_fail "$smoke_name: provider returned empty path"
    return 1
  fi
  if [ ! -f "$obj_path" ]; then
    echo "[INFO] vm-hako output:"
    echo "$run_out" | tail -n 120 || true
    rm -f "$tmp_hako"
    test_fail "$smoke_name: output object not found: $obj_path"
    return 1
  fi

  rm -f "$tmp_hako"
  test_pass "$smoke_name ($obj_path)"
  return 0
}

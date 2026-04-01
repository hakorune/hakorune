#!/bin/bash
# llvm_backend_runtime_proof_common.sh
# Shared helper for root-first `.hako VM -> LlvmBackendBox -> C-API -> exe` runtime proofs.
#
# This file is meant to be sourced from smoke scripts that already source:
#   tools/smokes/v2/lib/test_runner.sh

set -uo pipefail

llvm_backend_runtime_prepare_or_skip() {
  if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "$1: LLVM backend not available"
    return 1
  fi

  local build_log="${TMPDIR:-/tmp}/$1_ffi_build_$$.log"
  set +e
  local ffi_out
  ffi_out=$(bash "$NYASH_ROOT/tools/build_hako_llvmc_ffi.sh" 2>&1)
  local ffi_rc=$?
  set -e
  printf "%s\n" "$ffi_out" >"$build_log"
  if [ "$ffi_rc" -ne 0 ]; then
    echo "[INFO] ffi build output:"
    tail -n 120 "$build_log" || true
    test_fail "$1: FFI build failed (rc=$ffi_rc)"
    rm -f "$build_log"
    return 1
  fi
  rm -f "$build_log"
  return 0
}

llvm_backend_runtime_run_case() {
  local case_name="${1:-}"
  local input_mir="${2:-}"
  local expected_rc="${3:-}"

  if [ -z "$case_name" ] || [ -z "$input_mir" ] || [ -z "$expected_rc" ]; then
    test_fail "llvm_backend_runtime_run_case: missing case_name/input_mir/expected_rc"
    return 1
  fi
  if [ ! -f "$input_mir" ]; then
    test_fail "$case_name: fixture missing: $input_mir"
    return 1
  fi
  if ! llvm_backend_runtime_prepare_or_skip "$case_name"; then
    return 1
  fi

  local tmp_hako out_exe
  tmp_hako="$(mktemp --suffix ".hako")"
  out_exe="$(mktemp --suffix ".exe")"
  rm -f "$out_exe"

  cleanup_runtime_case() {
    rm -f "$tmp_hako" "$out_exe"
  }
  trap cleanup_runtime_case RETURN

  cat >"$tmp_hako" <<HAKO
using selfhost.shared.backend.llvm_backend as LlvmBackendBox

static box Main {
  method main(args) {
    local obj = LlvmBackendBox.compile_obj("$input_mir")
    if obj == null { return 91 }
    local ok = LlvmBackendBox.link_exe(obj, "$out_exe", null)
    if ok != 1 { return 92 }
    return 0
  }
}
HAKO

  set +e
  local run_out
  run_out=$(
    NYASH_LLVM_USE_CAPI=1 \
    HAKO_V1_EXTERN_PROVIDER_C_ABI=1 \
    timeout 120 \
    "$NYASH_ROOT/target/release/hakorune" --backend vm-hako "$tmp_hako" 2>&1
  )
  local run_rc=$?
  set -e

  if [ "$run_rc" -eq 124 ]; then
    test_fail "$case_name: vm-hako run timed out"
    return 1
  fi
  if [ "$run_rc" -ne 0 ]; then
    echo "[INFO] vm-hako output:"
    echo "$run_out" | tail -n 120 || true
    test_fail "$case_name: vm-hako caller failed (rc=$run_rc)"
    return 1
  fi
  if [ ! -x "$out_exe" ]; then
    echo "[INFO] vm-hako output:"
    echo "$run_out" | tail -n 120 || true
    test_fail "$case_name: expected exe missing: $out_exe"
    return 1
  fi

  set +e
  timeout 120 "$out_exe" >/dev/null 2>&1
  local exe_rc=$?
  set -e

  if [ "$exe_rc" -eq 124 ]; then
    test_fail "$case_name: executable run timed out"
    return 1
  fi
  if [ "$exe_rc" -ne "$expected_rc" ]; then
    test_fail "$case_name: executable exited $exe_rc (expected $expected_rc)"
    return 1
  fi

  test_pass "$case_name: PASS (.hako VM -> LlvmBackendBox -> C-API -> exe, rc=$expected_rc)"
  return 0
}

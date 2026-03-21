#!/bin/bash
# phase21_5_perf_kilo_result_parity_contract_vm.sh
#
# Contract pin:
# - kilo hk lane must always run with strict no-fallback route.
# - strict mode: either parity is already OK, or mismatch is detected fail-fast.
# - diagnostic mode: parity check can be intentionally skipped for timing-only probes.

set -euo pipefail

source "$(dirname "$0")/../../../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_kilo_result_parity_contract_vm"
SCRIPT="${NYASH_ROOT}/tools/perf/bench_compare_c_py_vs_hako.sh"
KEY="kilo_kernel_small_hk"

if [[ ! -x "${SCRIPT}" ]]; then
  test_fail "${SMOKE_NAME}: script not executable: ${SCRIPT}"
  exit 2
fi

STRICT_STATE="unknown"

set +e
STRICT_OUT="$(
  PERF_VM_TIMEOUT="${PERF_VM_TIMEOUT:-60s}" \
  HAKO_VM_MAX_STEPS="${HAKO_VM_MAX_STEPS:-300000000}" \
  NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
  PERF_VM_FORCE_NO_FALLBACK=1 \
  bash "${SCRIPT}" "${KEY}" 1 1 2>&1
)"
STRICT_RC=$?
set -e

if [[ "${STRICT_RC}" -eq 0 ]]; then
  STRICT_STATE="parity-ok"
  if ! printf '%s\n' "${STRICT_OUT}" | grep -q "\[bench4-route\] name=${KEY} "; then
    printf '%s\n' "${STRICT_OUT}" | tail -n 60 || true
    test_fail "${SMOKE_NAME}: strict mode missing bench4-route line"
    exit 1
  fi
  if ! printf '%s\n' "${STRICT_OUT}" | grep -q 'result_parity=ok'; then
    printf '%s\n' "${STRICT_OUT}" | tail -n 60 || true
    test_fail "${SMOKE_NAME}: strict mode must report result_parity=ok when rc=0"
    exit 1
  fi
  vm_result="$(printf '%s\n' "${STRICT_OUT}" | sed -n 's/.*vm_result=\(-\?[0-9]\+\).*/\1/p' | tail -n1)"
  aot_result="$(printf '%s\n' "${STRICT_OUT}" | sed -n 's/.*aot_result=\(-\?[0-9]\+\).*/\1/p' | tail -n1)"
  if [[ -z "${vm_result}" || -z "${aot_result}" ]]; then
    printf '%s\n' "${STRICT_OUT}" | tail -n 60 || true
    test_fail "${SMOKE_NAME}: strict mode missing vm_result/aot_result markers"
    exit 1
  fi
  if [[ "${vm_result}" != "${aot_result}" ]]; then
    printf '%s\n' "${STRICT_OUT}" | tail -n 60 || true
    test_fail "${SMOKE_NAME}: strict mode rc=0 but vm_result!=aot_result (${vm_result} vs ${aot_result})"
    exit 1
  fi
else
  STRICT_STATE="mismatch-guard"
  if ! printf '%s\n' "${STRICT_OUT}" | grep -q 'VM/AOT result mismatch'; then
    printf '%s\n' "${STRICT_OUT}" | tail -n 60 || true
    test_fail "${SMOKE_NAME}: strict mode failed without mismatch marker"
    exit 1
  fi
fi

set +e
DIAG_OUT="$(
  PERF_VM_TIMEOUT="${PERF_VM_TIMEOUT:-60s}" \
  HAKO_VM_MAX_STEPS="${HAKO_VM_MAX_STEPS:-300000000}" \
  NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
  PERF_VM_FORCE_NO_FALLBACK=1 \
  PERF_REQUIRE_AOT_RESULT_PARITY=0 \
  bash "${SCRIPT}" "${KEY}" 1 1 2>&1
)"
DIAG_RC=$?
set -e

if [[ "${DIAG_RC}" -ne 0 ]]; then
  printf '%s\n' "${DIAG_OUT}" | tail -n 60 || true
  test_fail "${SMOKE_NAME}: diagnostic mode must succeed (rc=${DIAG_RC})"
  exit 1
fi

if ! printf '%s\n' "${DIAG_OUT}" | grep -q "\[bench4-route\] name=${KEY} "; then
  printf '%s\n' "${DIAG_OUT}" | tail -n 60 || true
  test_fail "${SMOKE_NAME}: diagnostic mode missing bench4-route line"
  exit 1
fi
for lock in 'kernel_lane=hk' 'fallback_guard=strict-no-fallback' 'result_parity=skip'; do
  if ! printf '%s\n' "${DIAG_OUT}" | grep -q "${lock}"; then
    printf '%s\n' "${DIAG_OUT}" | tail -n 60 || true
    test_fail "${SMOKE_NAME}: diagnostic mode missing route lock: ${lock}"
    exit 1
  fi
done

printf '%s\n' "${DIAG_OUT}"
test_pass "${SMOKE_NAME}: PASS (strict=${STRICT_STATE}, diagnostic route lock pinned)"

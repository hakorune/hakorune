#!/bin/bash
# phase29y_hako_emit_mir_nested_ternary_unsupported_boundary_vm.sh
# B-TERNARY-02 boundary lock:
# - Rust route remains green for boundary fixture.
# - selfhost-first route must fail-fast with unsupported:ternary_no_lower.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29y_hako_emit_mir_nested_ternary_unsupported_boundary_vm"
INPUT_FIXTURE="${NYASH_ROOT}/apps/tests/phase29y_hako_emit_mir_nested_ternary_unsupported_boundary_min.hako"
TMP_RUST_MIR="$(mktemp /tmp/phase29y_nested_ternary_unsupported_rust.XXXXXX.json)"
TMP_HAKO_MIR="$(mktemp /tmp/phase29y_nested_ternary_unsupported_hako.XXXXXX.json)"

cleanup() {
  rm -f "$TMP_RUST_MIR" "$TMP_HAKO_MIR"
}
trap cleanup EXIT

if [ ! -f "${INPUT_FIXTURE}" ]; then
  test_fail "${SMOKE_NAME}: fixture missing: ${INPUT_FIXTURE}"
  exit 2
fi

set +e
RUST_OUT="$(env \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_VM_USE_FALLBACK=0 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  NYASH_JOINIR_DEV=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_STRICT="${HAKO_JOINIR_STRICT:-1}" \
  HAKO_JOINIR_PLANNER_REQUIRED="${HAKO_JOINIR_PLANNER_REQUIRED:-1}" \
  "${NYASH_BIN}" --emit-mir-json "${TMP_RUST_MIR}" "${INPUT_FIXTURE}" 2>&1)"
RUST_RC=$?

HAKO_OUT="$(env \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_VM_USE_FALLBACK=0 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  NYASH_JOINIR_DEV=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_STRICT="${HAKO_JOINIR_STRICT:-1}" \
  HAKO_JOINIR_PLANNER_REQUIRED="${HAKO_JOINIR_PLANNER_REQUIRED:-1}" \
  HAKO_SELFHOST_BUILDER_FIRST=1 \
  HAKO_SELFHOST_NO_DELEGATE=1 \
  HAKO_EMIT_MIR_MAINLINE_ONLY=1 \
  bash "${NYASH_ROOT}/tools/hakorune_emit_mir.sh" "${INPUT_FIXTURE}" "${TMP_HAKO_MIR}" 2>&1)"
HAKO_RC=$?
set -e

if [ "${RUST_RC}" -ne 0 ]; then
  printf '%s\n' "${RUST_OUT}" | tail -n 120 || true
  test_fail "${SMOKE_NAME}: rust route failed (rc=${RUST_RC})"
  exit 1
fi

if [ ! -s "${TMP_RUST_MIR}" ]; then
  test_fail "${SMOKE_NAME}: rust MIR output missing"
  exit 1
fi

if [ "${HAKO_RC}" -eq 0 ]; then
  printf '%s\n' "${HAKO_OUT}" | tail -n 120 || true
  test_fail "${SMOKE_NAME}: expected selfhost-first fail-fast, but route succeeded"
  exit 1
fi

if ! printf '%s\n' "${HAKO_OUT}" | grep -q '\[builder/selfhost-first:unsupported:ternary_no_lower\]'; then
  printf '%s\n' "${HAKO_OUT}" | tail -n 120 || true
  test_fail "${SMOKE_NAME}: ternary_no_lower marker missing"
  exit 1
fi

if ! printf '%s\n' "${HAKO_OUT}" | grep -q '\[FAIL\] selfhost-first failed and delegate disabled'; then
  printf '%s\n' "${HAKO_OUT}" | tail -n 120 || true
  test_fail "${SMOKE_NAME}: expected selfhost-first fail marker missing"
  exit 1
fi

test_pass "${SMOKE_NAME}: PASS"

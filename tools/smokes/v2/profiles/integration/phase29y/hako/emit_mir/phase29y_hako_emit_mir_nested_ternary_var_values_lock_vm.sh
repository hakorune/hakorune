#!/bin/bash
# phase29y_hako_emit_mir_nested_ternary_var_values_lock_vm.sh
# B-TERNARY-01 lock:
# - nested ternary with Var(Local Int) values must be accepted on selfhost-first route.
# - parity lock promotion is handled separately in B-TERNARY-03.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29y_hako_emit_mir_nested_ternary_var_values_lock_vm"
INPUT_FIXTURE="${NYASH_ROOT}/apps/tests/phase29y_hako_emit_mir_nested_ternary_var_values_min.hako"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
TMP_RUST_MIR="$(mktemp /tmp/phase29y_nested_ternary_var_values_rust.XXXXXX.json)"
TMP_HAKO_MIR="$(mktemp /tmp/phase29y_nested_ternary_var_values_hako.XXXXXX.json)"

cleanup() {
  rm -f "$TMP_RUST_MIR" "$TMP_HAKO_MIR"
}
trap cleanup EXIT

if [ ! -f "${INPUT_FIXTURE}" ]; then
  test_fail "${SMOKE_NAME}: fixture missing: ${INPUT_FIXTURE}"
  exit 2
fi
if [ ! -x "$EMIT_ROUTE" ]; then
  test_fail "${SMOKE_NAME}: emit route helper missing/executable: $EMIT_ROUTE"
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
  "$EMIT_ROUTE" --route direct --timeout-secs 0 --out "${TMP_RUST_MIR}" --input "${INPUT_FIXTURE}" 2>&1)"
RUST_RC=$?

HAKO_OUT="$(env \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_VM_USE_FALLBACK=0 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  NYASH_JOINIR_DEV=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_STRICT="${HAKO_JOINIR_STRICT:-1}" \
  HAKO_JOINIR_PLANNER_REQUIRED="${HAKO_JOINIR_PLANNER_REQUIRED:-1}" \
  "$EMIT_ROUTE" --route hako-mainline --timeout-secs 0 --out "${TMP_HAKO_MIR}" --input "${INPUT_FIXTURE}" 2>&1)"
HAKO_RC=$?
set -e

if [ "${RUST_RC}" -ne 0 ]; then
  printf '%s\n' "${RUST_OUT}" | tail -n 80 || true
  test_fail "${SMOKE_NAME}: rust route failed (rc=${RUST_RC})"
  exit 1
fi

if [ "${HAKO_RC}" -ne 0 ]; then
  printf '%s\n' "${HAKO_OUT}" | tail -n 120 || true
  test_fail "${SMOKE_NAME}: hako selfhost-first route failed (rc=${HAKO_RC})"
  exit 1
fi

if [ ! -s "${TMP_RUST_MIR}" ]; then
  test_fail "${SMOKE_NAME}: rust MIR output missing"
  exit 1
fi

if [ ! -s "${TMP_HAKO_MIR}" ]; then
  test_fail "${SMOKE_NAME}: hako MIR output missing"
  exit 1
fi

if printf '%s\n' "${HAKO_OUT}" | grep -q '\[builder/selfhost-first:unsupported:ternary_no_lower\]'; then
  printf '%s\n' "${HAKO_OUT}" | tail -n 120 || true
  test_fail "${SMOKE_NAME}: ternary_no_lower marker must not appear for var-values fixture"
  exit 1
fi

if ! printf '%s\n' "${HAKO_OUT}" | grep -q '\[OK\] MIR JSON written (selfhost-first):'; then
  printf '%s\n' "${HAKO_OUT}" | tail -n 120 || true
  test_fail "${SMOKE_NAME}: selfhost-first success marker missing"
  exit 1
fi

test_pass "${SMOKE_NAME}: PASS"

#!/bin/bash
# phase29bq_joinir_port04_phi_exit_invariant_lock_vm.sh
# JIR-PORT-04 contract:
# - route parity lock (rust/hako): both routes emit main with phi+compare+branch+ret.
# - hako route must be selfhost-first (no direct/delegate fallback).
# - hako-emitted MIR must satisfy PHI/exit invariants:
#   - every PHI has >=2 incoming edges
#   - every PHI incoming predecessor block exists
#   - at least one ret uses a PHI destination value (exit carrier)
# - executing hako-emitted MIR via --mir-json-file returns EXPECTED_RC.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/smokes/v2/lib/joinir_port_helpers.sh"
require_env || exit 2

SMOKE_NAME="phase29bq_joinir_port04_phi_exit_invariant_lock_vm"
FIXTURE="${1:-$NYASH_ROOT/apps/tests/phase29bq_joinir_port04_phi_exit_invariant_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
EXPECTED_RC="${EXPECTED_RC:-4}"

if ! [[ "$RUN_TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  test_fail "$SMOKE_NAME: timeout must be integer: $RUN_TIMEOUT_SECS"
  exit 2
fi
if ! [[ "$EXPECTED_RC" =~ ^-?[0-9]+$ ]]; then
  test_fail "$SMOKE_NAME: expected rc must be integer: $EXPECTED_RC"
  exit 2
fi

if [[ "$FIXTURE" != /* ]]; then
  FIXTURE="$NYASH_ROOT/$FIXTURE"
fi

EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
if [ ! -f "$FIXTURE" ]; then
  test_fail "$SMOKE_NAME: fixture missing: $FIXTURE"
  exit 2
fi
if [ ! -x "$EMIT_ROUTE" ]; then
  test_fail "$SMOKE_NAME: emit route helper missing/executable: $EMIT_ROUTE"
  exit 2
fi

TMP_RUST_MIR="$(mktemp /tmp/phase29bq_port04_rust_mir.XXXXXX.json)"
TMP_HAKO_MIR="$(mktemp /tmp/phase29bq_port04_hako_mir.XXXXXX.json)"
RUST_LOG="$(mktemp /tmp/phase29bq_port04_rust_log.XXXXXX.log)"
HAKO_LOG="$(mktemp /tmp/phase29bq_port04_hako_log.XXXXXX.log)"
KEEP_DEBUG_ARTIFACTS=0

cleanup() {
  if [ "$KEEP_DEBUG_ARTIFACTS" -eq 1 ]; then
    return 0
  fi
  rm -f "$TMP_RUST_MIR" "$TMP_HAKO_MIR" "$RUST_LOG" "$HAKO_LOG"
}
trap cleanup EXIT

retain_debug_artifacts() {
  KEEP_DEBUG_ARTIFACTS=1
  echo "[INFO] rust_log=$RUST_LOG"
  echo "[INFO] hako_log=$HAKO_LOG"
  echo "[INFO] rust_mir=$TMP_RUST_MIR"
  echo "[INFO] hako_mir=$TMP_HAKO_MIR"
}

TIMEOUT_MS=$((RUN_TIMEOUT_SECS * 1000))

set +e
timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_VM_USE_FALLBACK=0 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  HAKO_JOINIR_STRICT=1 \
  HAKO_JOINIR_PLANNER_REQUIRED=1 \
  NYASH_JOINIR_STRICT=1 \
  NYASH_JOINIR_DEV=1 \
  NYASH_NY_COMPILER_TIMEOUT_MS="$TIMEOUT_MS" \
  "$NYASH_BIN" --emit-mir-json "$TMP_RUST_MIR" "$FIXTURE" >"$RUST_LOG" 2>&1
rc_rust=$?

timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  HAKO_JOINIR_STRICT=1 \
  HAKO_JOINIR_PLANNER_REQUIRED=1 \
  NYASH_JOINIR_STRICT=1 \
  NYASH_JOINIR_DEV=1 \
  NYASH_NY_COMPILER_TIMEOUT_MS="$TIMEOUT_MS" \
  "$EMIT_ROUTE" --route hako-mainline --timeout-secs 0 --out "$TMP_HAKO_MIR" --input "$FIXTURE" >"$HAKO_LOG" 2>&1
rc_hako=$?
set -e

if [ "$rc_rust" -eq 124 ] || [ "$rc_hako" -eq 124 ]; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: timeout (rust=$rc_rust hako=$rc_hako)"
  exit 1
fi
if [ "$rc_rust" -ne 0 ] || [ "$rc_hako" -ne 0 ]; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: route failed (rust=$rc_rust hako=$rc_hako)"
  exit 1
fi
if [ ! -s "$TMP_RUST_MIR" ] || [ ! -s "$TMP_HAKO_MIR" ]; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: emitted MIR missing (rust/hako)"
  exit 1
fi

if ! joinir_port_check_selfhost_first_log "$SMOKE_NAME" "$HAKO_LOG"; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: selfhost-first route contract violated"
  exit 1
fi

if ! joinir_port_require_main_ops "$SMOKE_NAME[rust]" "$TMP_RUST_MIR" phi compare branch ret; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: rust parity shape (phi+compare+branch+ret) missing"
  exit 1
fi
if ! joinir_port_require_main_ops "$SMOKE_NAME[hako]" "$TMP_HAKO_MIR" phi compare branch ret; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: hako parity shape (phi+compare+branch+ret) missing"
  exit 1
fi

if ! jq -e '
  (.functions | map(select(.name=="main")) | .[0]) as $m
  | [ $m.blocks[].id ] as $block_ids
  | [ $m.blocks[] | .instructions[] | select(.op=="phi") ] as $phis
  | ($phis | length) >= 1
  and ([ $phis[] | ((.incoming // []) | length) ] | all(. >= 2))
  and ([ $phis[] | (.incoming // [])[] | .[1] ] | all(. as $pred | ($block_ids | index($pred)) != null))
  and (
    [ $m.blocks[] | .instructions[] | select(.op=="ret") | (.value // null) ] as $ret_values
    | [ $phis[] | .dst ] as $phi_dsts
    | ($ret_values | any(. as $v | ($phi_dsts | index($v)) != null))
  )
' "$TMP_HAKO_MIR" >/dev/null; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: hako PHI/exit invariants violated"
  exit 1
fi

set +e
NYASH_STAGEB_DEV_VERIFY=1 \
NYASH_VERIFY_ALLOW_NO_PHI=0 \
"$NYASH_BIN" --mir-json-file "$TMP_HAKO_MIR" >/dev/null 2>&1
rc_exec=$?
set -e
if [ "$rc_exec" -ne "$EXPECTED_RC" ]; then
  retain_debug_artifacts
  test_fail "$SMOKE_NAME: hako MIR exec rc mismatch (expected=$EXPECTED_RC actual=$rc_exec)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (parity shape + PHI/exit invariants locked)"

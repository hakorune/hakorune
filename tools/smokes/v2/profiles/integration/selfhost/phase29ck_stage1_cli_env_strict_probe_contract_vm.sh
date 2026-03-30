#!/bin/bash
# phase29ck_stage1_cli_env_strict_probe_contract_vm.sh
# Contract:
# - stage1_cli_env helper route must promote selfhost-first canonical MIR
# - strict stage1 dialect probe must observe no legacy boxcall/externcall

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

SMOKE_NAME="phase29ck_stage1_cli_env_strict_probe_contract_vm"
FIXTURE="$NYASH_ROOT/lang/src/runner/stage1_cli_env.hako"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
PROBE="$NYASH_ROOT/tools/dev/phase29ck_stage1_mir_dialect_probe.sh"
TMP_MIR="$(mktemp /tmp/phase29ck_stage1_cli_env.XXXXXX.json)"
TMP_LOG="$(mktemp /tmp/phase29ck_stage1_cli_env.XXXXXX.log)"

set +e
env HAKO_JOINIR_STRICT=1 \
    HAKO_JOINIR_PLANNER_REQUIRED=1 \
    "$EMIT_ROUTE" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-120}" --out "$TMP_MIR" --input "$FIXTURE" >"$TMP_LOG" 2>&1
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "$SMOKE_NAME: emit helper failed (rc=$rc)"
  tail -n 80 "$TMP_LOG" || true
  exit 1
fi

if ! grep -Fq "[OK] MIR JSON written (selfhost-first):" "$TMP_LOG"; then
  test_fail "$SMOKE_NAME: helper route did not stay selfhost-first"
  exit 1
fi
if grep -Fq "[OK] MIR JSON written (delegate:" "$TMP_LOG"; then
  test_fail "$SMOKE_NAME: delegate fallback detected"
  exit 1
fi
if grep -q '"functions_0"' "$TMP_MIR"; then
  test_fail "$SMOKE_NAME: collapsed functions keep leaked"
  exit 1
fi

if ! bash "$PROBE" --mir-json "$TMP_MIR" --strict-stage1 >/tmp/phase29ck_stage1_cli_env_probe.out 2>/tmp/phase29ck_stage1_cli_env_probe.err; then
  cat /tmp/phase29ck_stage1_cli_env_probe.out || true
  cat /tmp/phase29ck_stage1_cli_env_probe.err || true
  test_fail "$SMOKE_NAME: strict stage1 probe failed"
  exit 1
fi

test_pass "$SMOKE_NAME"

#!/bin/bash
# phase29ck_stage1_full_program_defs_promote_contract_vm.sh
# Contract:
# - defs-bearing Program(JSON) must still promote through selfhost-first helper route
# - collapsed keeps (`functions_0`) must not escape
# - default route does not require defs/helper visibility; explicit funcs-gate shape is a separate residual

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

SMOKE_NAME="phase29ck_stage1_full_program_defs_promote_contract_vm"
FIXTURE="$NYASH_ROOT/apps/tests/stage1_full_program_defs_return42_min.hako"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
TMP_MIR="$(mktemp /tmp/phase29ck_stage1_defs_promote.XXXXXX.json)"
TMP_LOG="$(mktemp /tmp/phase29ck_stage1_defs_promote.XXXXXX.log)"

set +e
env HAKO_JOINIR_STRICT=1 \
    HAKO_JOINIR_PLANNER_REQUIRED=1 \
    HAKO_MIR_BUILDER_FUNCS=1 \
    "$EMIT_ROUTE" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-30}" --out "$TMP_MIR" --input "$FIXTURE" >"$TMP_LOG" 2>&1
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "$SMOKE_NAME: emit helper failed (rc=$rc)"
  tail -n 40 "$TMP_LOG" || true
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
if grep -Fq "[OK] MIR JSON written (direct-emit" "$TMP_LOG"; then
  test_fail "$SMOKE_NAME: direct fallback detected"
  exit 1
fi

if ! grep -q '"functions"' "$TMP_MIR"; then
  test_fail "$SMOKE_NAME: canonical functions root missing"
  exit 1
fi
if grep -q '"functions_0"' "$TMP_MIR"; then
  test_fail "$SMOKE_NAME: collapsed functions keep leaked"
  exit 1
fi
if ! grep -q '"name":"main"' "$TMP_MIR"; then
  test_fail "$SMOKE_NAME: main function missing"
  exit 1
fi
if ! grep -q '"name":"Main.helper/0"' "$TMP_MIR"; then
  test_fail "$SMOKE_NAME: lowered helper def missing"
  exit 1
fi

test_pass "$SMOKE_NAME"

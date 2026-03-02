#!/usr/bin/env bash
# Phase 21.6 blocker canary:
# hako-mainline selfhost-first emit for simple loop currently fails with
# LowerLoopSimpleBox undefined ValueId. This canary passes while blocked.
set -euo pipefail

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd "$ROOT"

FIXTURE="$ROOT/apps/tests/phase216_mainline_loop_undefined_value_blocker_min.hako"
if [[ ! -f "$FIXTURE" ]]; then
  echo "[FAIL] missing fixture: $FIXTURE" >&2
  exit 1
fi

TMP_JSON=$(mktemp --suffix .json)
LOG_OUT=$(mktemp --suffix .log)

set +e
HAKO_SELFHOST_BUILDER_FIRST=1 \
NYASH_USE_NY_COMPILER=0 HAKO_DISABLE_NY_COMPILER=1 \
NYASH_FEATURES=stage3 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
  bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" \
    --route hako-mainline \
    --timeout-secs "${HAKO_BUILD_TIMEOUT:-60}" \
    --out "$TMP_JSON" \
    --input "$FIXTURE" >"$LOG_OUT" 2>&1
rc=$?
set -e

if [[ "$rc" -eq 0 ]]; then
  echo "[FAIL] phase216_loop_undefined_block: blocker resolved unexpectedly (promote to green canary)" >&2
  rm -f "$TMP_JSON" "$LOG_OUT" 2>/dev/null || true
  exit 1
fi

if ! rg -q "LowerLoopSimpleBox\\.try_lower/1" "$LOG_OUT"; then
  echo "[FAIL] phase216_loop_undefined_block: fail reason drift (missing LowerLoopSimpleBox tag)" >&2
  rm -f "$TMP_JSON" "$LOG_OUT" 2>/dev/null || true
  exit 1
fi

if ! rg -q "use of undefined value ValueId|Invalid value" "$LOG_OUT"; then
  echo "[FAIL] phase216_loop_undefined_block: fail reason drift (missing undefined value marker)" >&2
  rm -f "$TMP_JSON" "$LOG_OUT" 2>/dev/null || true
  exit 1
fi

echo "[PASS] phase216_loop_undefined_block observed (LowerLoopSimpleBox undefined ValueId)"
rm -f "$TMP_JSON" "$LOG_OUT" 2>/dev/null || true
exit 0

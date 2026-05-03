#!/usr/bin/env bash
# Phase 21.6 blocker regression canary:
# LowerLoopSimpleBox undefined ValueId が再発しないことを確認する。
set -euo pipefail

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../../../.." && pwd)
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

if [[ "$rc" -ne 0 ]]; then
  echo "[FAIL] phase216_loop_undefined_block: emit failed unexpectedly" >&2
  tail -n 80 "$LOG_OUT" >&2 || true
  rm -f "$TMP_JSON" "$LOG_OUT" 2>/dev/null || true
  exit 1
fi

if ! [[ -s "$TMP_JSON" ]]; then
  echo "[FAIL] phase216_loop_undefined_block: MIR output missing" >&2
  rm -f "$TMP_JSON" "$LOG_OUT" 2>/dev/null || true
  exit 1
fi

if ! rg -q "\"functions\"" "$TMP_JSON"; then
  echo "[FAIL] phase216_loop_undefined_block: MIR payload invalid (missing functions)" >&2
  rm -f "$TMP_JSON" "$LOG_OUT" 2>/dev/null || true
  exit 1
fi

if rg -q "LowerLoopSimpleBox\\.(try_lower|_lower_from_cmp|_emit_or_build_with_limit)/.*(use of undefined value ValueId|Invalid value)" "$LOG_OUT"; then
  echo "[FAIL] phase216_loop_undefined_block: LowerLoopSimple undefined ValueId regressed" >&2
  tail -n 80 "$LOG_OUT" >&2 || true
  rm -f "$TMP_JSON" "$LOG_OUT" 2>/dev/null || true
  exit 1
fi

echo "[PASS] phase216_loop_undefined_block regression guard green"
rm -f "$TMP_JSON" "$LOG_OUT" 2>/dev/null || true
exit 0

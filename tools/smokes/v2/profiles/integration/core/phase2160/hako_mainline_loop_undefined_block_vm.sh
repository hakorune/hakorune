#!/usr/bin/env bash
# Block canary: hako-mainline selfhost-first loop emit currently fails with
# LowerLoopSimpleBox undefined ValueId. PASS while blocked, FAIL if resolved.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
BIN="${ROOT_DIR}/target/release/hakorune"
if [[ ! -x "${BIN}" ]]; then
  echo "[SKIP] hakorune not built"
  exit 0
fi

FIXTURE="${ROOT_DIR}/apps/tests/phase216_mainline_loop_undefined_value_blocker_min.hako"
if [[ ! -f "${FIXTURE}" ]]; then
  echo "[SKIP] missing fixture: ${FIXTURE}"
  exit 0
fi

TMP_JSON=$(mktemp --suffix .json)
TMP_LOG=$(mktemp --suffix .log)
trap 'rm -f "${TMP_JSON}" "${TMP_LOG}" || true' EXIT

set +e
HAKO_SELFHOST_BUILDER_FIRST=1 \
NYASH_USE_NY_COMPILER=0 HAKO_DISABLE_NY_COMPILER=1 \
NYASH_FEATURES=stage3 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
  bash "${ROOT_DIR}/tools/smokes/v2/lib/emit_mir_route.sh" \
    --route hako-mainline \
    --timeout-secs "${HAKO_BUILD_TIMEOUT:-10}" \
    --out "${TMP_JSON}" \
    --input "${FIXTURE}" >"${TMP_LOG}" 2>&1
rc=$?
set -e

if [[ "$rc" -eq 0 ]]; then
  echo "[FAIL] hako_mainline_loop_undefined_block: blocker resolved unexpectedly"
  exit 1
fi

if ! rg -q "LowerLoopSimpleBox\\.(try_lower|_lower_from_cmp|_emit_or_build_with_limit)/" "${TMP_LOG}"; then
  echo "[SKIP] fail reason drift (missing LowerLoopSimpleBox tag)"
  exit 0
fi
if ! rg -q "use of undefined value ValueId|Invalid value" "${TMP_LOG}"; then
  echo "[SKIP] fail reason drift (missing undefined value marker)"
  exit 0
fi

echo "[PASS] hako_mainline_loop_undefined_block observed"
exit 0

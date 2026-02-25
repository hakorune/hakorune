#!/bin/bash
# phase29bq_hako_mirbuilder_multi_local_reject_vm.sh
# Contract pin: reject unsupported Program(JSON v0) multi-Local variant
# where print uses a non-last local var.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

export HAKO_JOINIR_PLANNER_REQUIRED=1

ENTRY="$NYASH_ROOT/lang/src/compiler/mirbuilder/emit_mir_json_v0_from_program_json_v0.hako"
FIXTURE="$NYASH_ROOT/apps/tests/phase29bq_hako_mirbuilder_phase10_multi_local_reject_min.hako"

TMP_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"
RUN_ID="phase29bq_hako_mirbuilder_multi_local_reject_${$}"
PJSON="$TMP_DIR/${RUN_ID}_program.json"
OUT="$TMP_DIR/${RUN_ID}.out"

rm -f "$PJSON" "$OUT"

"$NYASH_BIN" --emit-program-json-v0 "$PJSON" "$FIXTURE" >/dev/null

set +e
HAKO_PROGRAM_JSON_FILE="$PJSON" "$NYASH_BIN" --backend vm "$ENTRY" >"$OUT" 2>&1
RC=$?
set -e

if [ "$RC" -eq 0 ]; then
  echo "[FAIL] hako_mirbuilder multi_local reject pin: expected reject but emit succeeded" >&2
  echo "[FAIL] fixture=$FIXTURE" >&2
  echo "[FAIL] entry=$ENTRY" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] output=$OUT" >&2
  exit 1
fi

EXPECTED_TAG="\\[freeze:contract\\]\\[hako_mirbuilder\\]\\[cap_missing/expr:Var\\]"
if ! rg -n "$EXPECTED_TAG" "$OUT" >/dev/null; then
  echo "[FAIL] hako_mirbuilder multi_local reject pin: missing expected reject tag" >&2
  echo "[FAIL] expected tag: $EXPECTED_TAG" >&2
  echo "[FAIL] rc=$RC" >&2
  echo "[FAIL] fixture=$FIXTURE" >&2
  echo "[FAIL] entry=$ENTRY" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] output=$OUT" >&2
  exit 1
fi

if ! rg -n "Print var must be the Local var" "$OUT" >/dev/null; then
  echo "[FAIL] hako_mirbuilder multi_local reject pin: missing reject reason text" >&2
  echo "[FAIL] rc=$RC" >&2
  echo "[FAIL] fixture=$FIXTURE" >&2
  echo "[FAIL] entry=$ENTRY" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] output=$OUT" >&2
  exit 1
fi

echo "[PASS] hako_mirbuilder multi_local reject pin: PASS"

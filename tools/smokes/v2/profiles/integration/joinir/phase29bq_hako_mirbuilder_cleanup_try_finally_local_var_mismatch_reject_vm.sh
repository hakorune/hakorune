#!/bin/bash
# phase29bq_hako_mirbuilder_cleanup_try_finally_local_var_mismatch_reject_vm.sh
# M7-min-4 pin: reject var-mismatch between Try body update and finally Local update.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/smokes/v2/lib/stageb_helpers.sh"
require_env || exit 2

export HAKO_JOINIR_PLANNER_REQUIRED=1

BASE_FIXTURE="$NYASH_ROOT/apps/tests/phase29bq_selfhost_cleanup_only_min.hako"
ENTRY="$NYASH_ROOT/lang/src/compiler/mirbuilder/emit_mir_json_v0_from_program_json_v0.hako"

TMP_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"
RUN_ID="phase29bq_hako_mirbuilder_cleanup_try_finally_local_var_mismatch_reject_${$}"
BASE_PJSON="$TMP_DIR/${RUN_ID}_base_program.json"
PJSON="$TMP_DIR/${RUN_ID}_program.json"
OUT="$TMP_DIR/${RUN_ID}_emit.out"

rm -f "$BASE_PJSON" "$PJSON" "$OUT"

stageb_emit_program_json_v0_fixture "$BASE_PJSON" "$BASE_FIXTURE"

# Keep Try body as `x = x + 10`, mutate finally to `y = y + 2`.
jq -c '
  (.body[] | select(.type=="Try") | .finally) = [
    {"expr":{"lhs":{"name":"y","type":"Var"},"op":"+","rhs":{"type":"Int","value":2},"type":"Binary"},"name":"y","type":"Local"}
  ]
  | .body = [
      .body[0],
      .body[1],
      {"expr":{"args":[{"name":"x","type":"Var"}],"name":"env.console.log","type":"Call"},"type":"Expr"},
      .body[2]
    ]
' "$BASE_PJSON" > "$PJSON"

set +e
HAKO_PROGRAM_JSON_FILE="$PJSON" "$NYASH_BIN" --backend vm "$ENTRY" >"$OUT" 2>&1
RC=$?
set -e

if [ "$RC" -eq 0 ]; then
  echo "[FAIL] cleanup_try finally_local var_mismatch reject: expected failure but succeeded" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] output=$OUT" >&2
  exit 1
fi

if ! rg -n "\\[cap_missing/stmt:Try\\] Try finally Local must update the same var as Try body" "$OUT" >/dev/null; then
  echo "[FAIL] cleanup_try finally_local var_mismatch reject: missing expected freeze tag" >&2
  echo "[FAIL] rc=$RC" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] output=$OUT" >&2
  exit 1
fi

echo "[PASS] hako_mirbuilder cleanup_try finally_local var_mismatch reject pin: PASS"

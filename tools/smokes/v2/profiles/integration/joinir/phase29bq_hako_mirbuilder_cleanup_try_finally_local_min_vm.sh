#!/bin/bash
# phase29bq_hako_mirbuilder_cleanup_try_finally_local_min_vm.sh
# M7-min-3 pin: accept non-loop Try(cleanup) where finally has one Local stmt.
#
# Contract target:
# - Try.try = [Local x = x + 10]
# - Try.finally = [Local x = x + 2]
# - outer body includes Expr(Call env.console.log(x))
# Expected stdout: 13

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/smokes/v2/lib/stageb_helpers.sh"
require_env || exit 2

export HAKO_JOINIR_PLANNER_REQUIRED=1

BASE_FIXTURE="$NYASH_ROOT/apps/tests/phase29bq_selfhost_cleanup_only_min.hako"
ENTRY="$NYASH_ROOT/lang/src/compiler/mirbuilder/emit_mir_json_v0_from_program_json_v0.hako"

TMP_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"
RUN_ID="phase29bq_hako_mirbuilder_cleanup_try_finally_local_min_${$}"
BASE_PJSON="$TMP_DIR/${RUN_ID}_base_program.json"
PJSON="$TMP_DIR/${RUN_ID}_program.json"
MJSON="$TMP_DIR/${RUN_ID}_mir.json"

rm -f "$BASE_PJSON" "$PJSON" "$MJSON"

stageb_emit_program_json_v0_fixture "$BASE_PJSON" "$BASE_FIXTURE"

jq -c '
  (.body[] | select(.type=="Try") | .finally) = [
    {"expr":{"lhs":{"name":"x","type":"Var"},"op":"+","rhs":{"type":"Int","value":2},"type":"Binary"},"name":"x","type":"Local"}
  ]
  | .body = [
      .body[0],
      .body[1],
      {"expr":{"args":[{"name":"x","type":"Var"}],"name":"env.console.log","type":"Call"},"type":"Expr"},
      .body[2]
    ]
' "$BASE_PJSON" > "$PJSON"

set +e
HAKO_PROGRAM_JSON_FILE="$PJSON" "$NYASH_BIN" --backend vm "$ENTRY" >"$MJSON"
EMIT_RC=$?
set -e
if [ "$EMIT_RC" -ne 0 ]; then
  echo "[FAIL] hako_mirbuilder cleanup_try_finally_local pin: emit route failed (rc=$EMIT_RC)" >&2
  echo "[FAIL] base_program_json=$BASE_PJSON" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] mir_json=$MJSON" >&2
  exit 1
fi

set +e
RAW_OUT="$("$NYASH_BIN" --mir-json-file "$MJSON" 2>&1)"
EXEC_RC=$?
set -e
OUT="$(printf "%s" "$RAW_OUT" | filter_noise || true)"

if [ "$EXEC_RC" -ne 0 ]; then
  echo "[FAIL] hako_mirbuilder cleanup_try_finally_local pin: --mir-json-file failed (rc=$EXEC_RC)" >&2
  echo "[FAIL] expected rc=0" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] mir_json=$MJSON" >&2
  exit 1
fi

if [ "$OUT" != "13" ]; then
  echo "[FAIL] hako_mirbuilder cleanup_try_finally_local pin: unexpected stdout (expected='13' got='$OUT')" >&2
  echo "[FAIL] program_json=$PJSON" >&2
  echo "[FAIL] mir_json=$MJSON" >&2
  exit 1
fi

echo "[PASS] hako_mirbuilder cleanup_try_finally_local pin: PASS"

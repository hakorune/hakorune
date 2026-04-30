#!/bin/bash
# phase29bq_hako_mirbuilder_cleanup_try_reject_nonminimal_vm.sh
# M7-min-2 pin: reject non-minimal Try(cleanup) shapes in .hako mirbuilder route.
#
# Contract targets:
# - multi-stmt try body -> reject [cap_missing/stmt:Try]
# - non-empty catches -> reject [cap_missing/stmt:Try]
# - loop+cleanup mix -> reject [cap_missing/stmt:Loop]

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/smokes/v2/lib/stageb_helpers.sh"
require_env || exit 2

export HAKO_JOINIR_PLANNER_REQUIRED=1

ENTRY="$NYASH_ROOT/lang/src/compiler/mirbuilder/emit_mir_json_v0_from_program_json_v0.hako"
BASE_FIXTURE="$NYASH_ROOT/apps/tests/phase29bq_selfhost_cleanup_only_min.hako"

TMP_DIR="${PHASE29BQ_FAST_LOG_DIR:-/tmp}"
RUN_ID="phase29bq_hako_mirbuilder_cleanup_try_reject_nonminimal_${$}"
BASE_PJSON="$TMP_DIR/${RUN_ID}_base_program.json"

stageb_emit_program_json_v0_fixture "$BASE_PJSON" "$BASE_FIXTURE"

run_reject_case() {
  local case_name="$1"
  local program_json="$2"
  local expected_tag="$3"
  local out="$TMP_DIR/${RUN_ID}_${case_name}.out"

  set +e
  HAKO_PROGRAM_JSON_FILE="$program_json" "$NYASH_BIN" --backend vm "$ENTRY" >"$out" 2>&1
  local rc=$?
  set -e

  if [ "$rc" -eq 0 ]; then
    echo "[FAIL] $case_name: expected reject but emit succeeded" >&2
    echo "[FAIL] expected tag: $expected_tag" >&2
    echo "[FAIL] program_json=$program_json" >&2
    echo "[FAIL] output=$out" >&2
    exit 1
  fi

  if ! rg -n "$expected_tag" "$out" >/dev/null; then
    echo "[FAIL] $case_name: missing expected reject tag" >&2
    echo "[FAIL] expected tag: $expected_tag" >&2
    echo "[FAIL] rc=$rc" >&2
    echo "[FAIL] program_json=$program_json" >&2
    echo "[FAIL] output=$out" >&2
    exit 1
  fi
}

# 1) multi-stmt try body
PJSON_MULTI="$TMP_DIR/${RUN_ID}_multi_stmt_try.json"
jq -c '
  (.body[] | select(.type=="Try") | .try) = [
    {"type":"Local","name":"x","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"x"},"rhs":{"type":"Int","value":1}}},
    {"type":"Local","name":"x","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"x"},"rhs":{"type":"Int","value":2}}}
  ]
' "$BASE_PJSON" > "$PJSON_MULTI"
run_reject_case "multi_stmt_try" "$PJSON_MULTI" "\\[cap_missing/stmt:Try\\]"

# 2) non-empty catches
PJSON_CATCHES="$TMP_DIR/${RUN_ID}_nonempty_catches.json"
jq -c '
  (.body[] | select(.type=="Try") | .catches) = [
    {"type":"Catch","name":"e","body":[{"type":"Expr","expr":{"type":"Call","name":"env.console.log","args":[{"type":"Var","name":"e"}]}}]}
  ]
' "$BASE_PJSON" > "$PJSON_CATCHES"
run_reject_case "nonempty_catches" "$PJSON_CATCHES" "\\[cap_missing/stmt:Try\\]"

# 3) loop+cleanup mix
PJSON_LOOP="$TMP_DIR/${RUN_ID}_loop_cleanup_mix.json"
jq -c '
  (.body[] | select(.type=="Try") | .try) = [
    {"cond":{"lhs":{"type":"Var","name":"x"},"op":"<","rhs":{"type":"Int","value":20},"type":"Compare"},"body":[{"expr":{"lhs":{"type":"Var","name":"x"},"op":"+","rhs":{"type":"Int","value":1},"type":"Binary"},"name":"x","type":"Local"}],"type":"Loop"}
  ]
' "$BASE_PJSON" > "$PJSON_LOOP"
run_reject_case "loop_cleanup_mix" "$PJSON_LOOP" "\\[cap_missing/stmt:Loop\\]"

echo "[PASS] hako_mirbuilder cleanup_try reject_nonminimal pin: PASS"

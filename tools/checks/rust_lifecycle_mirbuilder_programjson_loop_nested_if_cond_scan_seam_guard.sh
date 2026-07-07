#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-loop-nested-if-cond-scan-seam-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-nested-if-cond-scan-seam-v0.json"
LOOP_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako"
SCAN="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_nested_if_cond_scan_box.hako"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_var_rhs_producer_followon_selection_guard.sh"
REL_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_loop_nested_if_cond_recipe_relational_row_batch_gate.sh"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$FIXTURE" "$LOOP_HANDLER" "$SCAN" "$PREV_GATE" "$REL_GATE" "$HAKO_BIN"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^loop_nested_if_cond_scan_seam_selected=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "Loop nested If scan seam selection prerequisite is not green"
fi

REL_OUT="$(guard_cached_run "$TAG" bash "$REL_GATE")"
if ! grep -q '^loop_nested_if_cond_recipe_relational_row_batch=1$' <<<"$REL_OUT"; then
  printf '%s\n' "$REL_OUT" >&2
  guard_fail "$TAG" "existing Loop nested If relational rows are not green"
fi

bash "$HAKO_BIN" --backend mir --verify "$SCAN" >/dev/null

python3 - "$FIXTURE" "$LOOP_HANDLER" "$SCAN" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
loop_impl = Path(sys.argv[2]).read_text(encoding="utf-8")
scan = Path(sys.argv[3]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonLoopNestedIfCondScanSeamV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-SCAN-SEAM-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-VAR-RHS-PRODUCER-FOLLOWON-SELECTION-001", "bad prerequisite")
need(fixture.get("owner") == "LoopNestedIfCondScanBox", "bad owner")
need(fixture.get("moved_from") == "LoopStmtHandler", "bad moved_from")

contract = fixture.get("contract") or {}
need(contract.get("accepted_row_added") is False, "accepted row must not be added")
need(contract.get("var_rhs_support") is False, "historical fixture must remain pre-Var-rhs")
need(contract.get("legacy_output") == "if_cond_rhs_int and if_cond_start", "bad legacy output")

claims = fixture.get("claims") or {}
for key in [
    "loop_nested_if_cond_scan_seam",
    "loop_nested_if_cond_scan_owner_extracted",
    "loop_stmt_handler_below_800_lines",
    "existing_loop_nested_if_rows_preserved",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "accepted_row_added",
    "loop_nested_if_var_rhs_row_implemented",
    "top_level_loop_var_rhs_row_selected",
    "length_bound_producer_selected",
    "bool_recipe_lowering_executed",
    "mir_cmp_emission",
    "branch_emission",
    "basic_block_mutation",
    "value_id_allocation",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_route_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need("LoopNestedIfCondScanBox.read_int_rhs(" in loop_impl, "LoopStmtHandler must call scan seam")
need("_read_loop_if_cond_var_lt_int" not in loop_impl, "old local cond scan helper must be removed")
need("Loop If Compare rhs must be Int" not in loop_impl, "Int rhs condition policy must move out of LoopStmtHandler")
for needle in [
    "read_int_rhs(program_json, stmt_start, local_name, tag, label): MapBox",
    "Loop If",
    "cond Compare op is unsupported",
    "if_cond_rhs_kind_code",
    '"if_cond_rhs_int"',
    '"if_cond_start"',
]:
    need(needle in scan, f"scan owner missing token: {needle}")
PY

loop_lines="$(wc -l < "$LOOP_HANDLER" | tr -d ' ')"
scan_lines="$(wc -l < "$SCAN" | tr -d ' ')"
if [ "$loop_lines" -ge 800 ]; then
  guard_fail "$TAG" "LoopStmtHandler must remain below 800 lines"
fi

cat <<REPORT
output_contract=rust-lifecycle-mirbuilder-programjson-loop-nested-if-cond-scan-seam-guard-v0
token=MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-SCAN-SEAM-001
owner=LoopNestedIfCondScanBox
loop_nested_if_cond_scan_seam=1
loop_nested_if_cond_scan_owner_extracted=1
loop_stmt_handler_lines=$loop_lines
scan_owner_lines=$scan_lines
loop_stmt_handler_below_800_lines=1
existing_loop_nested_if_rows_preserved=1
accepted_row_added=0
loop_nested_if_var_rhs_row_implemented=0
top_level_loop_var_rhs_row_selected=0
length_bound_producer_selected=0
bool_recipe_lowering_executed=0
mir_cmp_emission=0
branch_emission=0
basic_block_mutation=0
value_id_allocation=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-VAR-RHS-BOUND-ROW-001
summary=ok
REPORT

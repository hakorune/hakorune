#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-var-rhs-producer-followon-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-var-rhs-producer-followon-selection-v0.json"
LOOP_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako"
BRIDGE="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_nested_if_cond_recipe_bridge_box.hako"
COND_SCAN="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_nested_if_cond_scan_box.hako"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_if_cond_recipe_var_rhs_bound_row_gate.sh"

guard_require_command "$TAG" python3
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$FIXTURE" "$LOOP_HANDLER" "$BRIDGE" "$PREV_GATE"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^if_cond_recipe_var_rhs_bound_row=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "If Var rhs producer row prerequisite is not green"
fi

python3 - "$FIXTURE" "$LOOP_HANDLER" "$BRIDGE" "$COND_SCAN" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
loop_impl = Path(sys.argv[2]).read_text(encoding="utf-8")
bridge = Path(sys.argv[3]).read_text(encoding="utf-8")
cond_scan = ""
if len(sys.argv) > 4 and Path(sys.argv[4]).exists():
    cond_scan = Path(sys.argv[4]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderProgramJsonVarRhsProducerFollowonSelectionV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-PROGRAMJSON-VAR-RHS-PRODUCER-FOLLOWON-SELECTION-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-VAR-RHS-BOUND-ROW-001", "bad prerequisite")

state = fixture.get("input_state") or {}
need(state.get("if_var_rhs_row_green") is True, "If Var rhs row must be green")
need(state.get("loop_stmt_handler_line_count_near_limit") is True, "line limit pressure must be recorded")
need(state.get("loop_nested_if_bridge_exists") is True, "nested If bridge state missing")
need(state.get("top_level_loop_route_sensitive") is True, "top-level Loop sensitivity missing")

candidates = {row.get("name"): row for row in fixture.get("candidates") or []}
need(candidates["LoopNestedIfCondScanSeam"].get("selected") is True, "Loop nested If scan seam must be selected")
need(candidates["LoopNestedIfCondScanSeam"].get("selected_next_card") == "MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-SCAN-SEAM-001", "bad selected next")
for name in [
    "LoopNestedIfVarRhsRowDirect",
    "TopLevelLoopVarRhsRow",
    "LengthBoundProducerRows",
    "StopForExternalConsultation",
]:
    need(candidates[name].get("selected") is False, f"{name} must not be selected")

boundary = fixture.get("selected_boundary") or {}
need(boundary.get("kind") == "BoxShape cleanup before BoxCount row", "bad selected boundary kind")
need(boundary.get("accepted_row_added") is False, "selection must not add accepted row")
need(boundary.get("lowering") is False, "lowering must stay false")
need(boundary.get("route_selection") is False, "route selection must stay false")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectLoopNestedIfCondScanSeamBeforeVarRhsRow", "bad decision")
need(decision.get("selected_next_card") == "MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-SCAN-SEAM-001", "bad decision next")

claims = fixture.get("claims") or {}
for key in ["var_rhs_producer_followon_selection", "loop_nested_if_cond_scan_seam_selected"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "loop_nested_if_var_rhs_row_implemented",
    "top_level_loop_var_rhs_row_selected",
    "length_bound_producer_selected",
    "accepted_row_added",
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

need("Loop If Compare rhs must be Int" in loop_impl or "cond rhs must be Int" in cond_scan, "Loop nested If still must be Int-bound before row card")
need("Loop Compare rhs must be Int" in loop_impl, "Top-level Loop must remain Int-bound")
need("if_cond_rhs_int" in loop_impl or "if_cond_rhs_int" in cond_scan, "Loop body scan int rhs surface must still be visible")
need("LoopNestedIfCondRecipeBridgeBox.if_item(" in loop_impl, "Loop nested If bridge call missing")
need("ProgramJsonCompareReaderBox.read_var_int_compare(program_json, cond_start)" in bridge, "bridge must use shared reader")
PY

loop_lines="$(wc -l < "$LOOP_HANDLER" | tr -d ' ')"
if [ "$loop_lines" -ge 800 ]; then
  guard_fail "$TAG" "LoopStmtHandler must remain below 800 lines before seam card"
fi

cat <<REPORT
output_contract=rust-lifecycle-mirbuilder-programjson-var-rhs-producer-followon-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-VAR-RHS-PRODUCER-FOLLOWON-SELECTION-001
decision=SelectLoopNestedIfCondScanSeamBeforeVarRhsRow
var_rhs_producer_followon_selection=1
loop_nested_if_cond_scan_seam_selected=1
loop_stmt_handler_lines=$loop_lines
loop_nested_if_var_rhs_row_implemented=0
top_level_loop_var_rhs_row_selected=0
length_bound_producer_selected=0
accepted_row_added=0
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
selected_next_card=MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-SCAN-SEAM-001
summary=ok
REPORT

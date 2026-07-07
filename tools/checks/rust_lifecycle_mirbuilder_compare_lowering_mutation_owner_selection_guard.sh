#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-lowering-mutation-owner-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-lowering-mutation-owner-selection-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3300-MIRBUILDER-COMPARE-LOWERING-MUTATION-OWNER-SELECTION-001.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
PREV_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-var-rhs-producer-closeout-v0.json"
INTENT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/bool_recipe_compare_lowering_intent_snapshot.hako"
COMPARE_RS="$ROOT_DIR/src/mir/builder/ops/comparison.rs"
EMIT_COMPARE_RS="$ROOT_DIR/src/mir/builder/emission/compare.rs"
EMIT_BRANCH_RS="$ROOT_DIR/src/mir/builder/emission/branch.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" "$PREV_FIXTURE" "$INTENT_IMPL" "$COMPARE_RS" "$EMIT_COMPARE_RS" "$EMIT_BRANCH_RS"

python3 - "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" "$PREV_FIXTURE" "$INTENT_IMPL" "$COMPARE_RS" "$EMIT_COMPARE_RS" "$EMIT_BRANCH_RS" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
current_state = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
prev_fixture = json.loads(Path(sys.argv[5]).read_text(encoding="utf-8"))
intent_impl = Path(sys.argv[6]).read_text(encoding="utf-8")
compare_rs = Path(sys.argv[7]).read_text(encoding="utf-8")
emit_compare = Path(sys.argv[8]).read_text(encoding="utf-8")
emit_branch = Path(sys.argv[9]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderCompareLoweringMutationOwnerSelectionV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-LOWERING-MUTATION-OWNER-SELECTION-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-PROGRAMJSON-VAR-RHS-PRODUCER-CLOSEOUT-001", "bad prerequisite")

prev_decision = prev_fixture.get("decision") or {}
need(prev_decision.get("selected_next_card") == "MIRBUILDER-COMPARE-LOWERING-MUTATION-OWNER-SELECTION-001", "previous closeout must select this card")

candidates = {row.get("name"): row for row in fixture.get("candidates") or []}
need(candidates["SymbolicCompareLoweringCommandPilot"].get("selected") is True, "symbolic pilot must be selected")
need(candidates["SymbolicCompareLoweringCommandPilot"].get("selected_next_card") == "MIRBUILDER-COMPARE-LOWERING-SYMBOLIC-COMMAND-PILOT-001", "bad selected next")
for name in [
    "EmitMirCompareNow",
    "EmitMirCompareAndBranchNow",
    "SelectMutationBearingOwnerNow",
    "ExternalDesignStop",
]:
    need(candidates[name].get("selected") is False, f"{name} must not be selected")

boundary = fixture.get("selected_boundary") or {}
need(boundary.get("name") == "CompareLoweringSymbolicCommandSnapshotV1", "bad selected boundary")
need(boundary.get("input") == "BoolRecipeCompareLoweringIntentSnapshotV1", "bad input")
need(boundary.get("output") == "read-only symbolic compare lowering command snapshot", "bad output")
for field in [
    "lhs_symbol_id",
    "mir_compare_op_code",
    "rhs_bound_kind_code",
    "rhs_bound_i64",
    "rhs_bound_symbol_id",
    "dst_policy",
    "branch_target_policy",
    "analysis_only",
]:
    need(field in boundary.get("allowed_fields", []), f"missing allowed field: {field}")
for action in [
    "operand ValueId resolution",
    "rhs runtime materialization",
    "MIR Compare emission",
    "MIR Branch emission",
    "BasicBlock mutation",
    "ValueId allocation",
    "route selection",
    "runtime route switch",
    "ProgramJSON runtime route authority",
    "runtime fallback",
]:
    need(action in boundary.get("forbidden_actions", []), f"missing forbidden action: {action}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectSymbolicCompareLoweringCommandPilot", "bad decision")
need(decision.get("selected_next_card") == "MIRBUILDER-COMPARE-LOWERING-SYMBOLIC-COMMAND-PILOT-001", "bad decision next")

claims = fixture.get("claims") or {}
for key in [
    "compare_lowering_mutation_owner_selection",
    "symbolic_compare_lowering_command_pilot_selected",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "mutation_bearing_owner_selected",
    "bool_recipe_lowering_executed",
    "operand_value_id_resolution",
    "rhs_runtime_materialization",
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

need(
    'latest_card = "MIRBUILDER-COMPARE-LOWERING-MUTATION-OWNER-SELECTION-001"' in current_state
    or 'latest_card = "MIRBUILDER-COMPARE-LOWERING-SYMBOLIC-COMMAND-PILOT-001"' in current_state
    or 'latest_card = "MIRBUILDER-COMPARE-LOWERING-SYMBOLIC-COMMAND-PARITY-001"' in current_state
    or 'latest_card = "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-OWNER-SELECTION-001"' in current_state
    or 'latest_card = "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PILOT-001"' in current_state
    or 'latest_card = "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PARITY-001"' in current_state,
    "CURRENT_STATE latest card must point to selection or a selected follow-on",
)
need(
    "MIRBUILDER-COMPARE-LOWERING-SYMBOLIC-COMMAND-PILOT-001" in task_order
    or "intent-map symbolic command pilot" in task_order,
    "task-order must retain selected pilot evidence",
)
need(
    "MIRBUILDER-COMPARE-LOWERING-MUTATION-OWNER-SELECTION-001; status=landed" in task_order
    or "symbolic compare lowering command owner selection" in task_order,
    "task-order must retain selection landed evidence",
)

for snippet in [
    "BoolRecipeCompareLoweringIntentSnapshotV1",
    "MIR Compare/Branch emission",
    '"mir_cmp_emission" => 0',
    '"branch_emission" => 0',
    '"value_id_allocation" => 0',
]:
    need(snippet in intent_impl, f"lowering intent boundary drift: {snippet}")
need("build_comparison_op" in compare_rs and "next_value_id" in compare_rs, "Rust comparison owner must still own ValueId allocation")
need("MirInstruction::Compare" in emit_compare, "Rust MIR Compare emission owner missing")
need("MirInstruction::Branch" in emit_branch, "Rust MIR Branch emission owner missing")
need("CompareLoweringSymbolicCommandSnapshotV1" in card, "card must describe selected boundary")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-lowering-mutation-owner-selection-guard-v0
token=MIRBUILDER-COMPARE-LOWERING-MUTATION-OWNER-SELECTION-001
decision=SelectSymbolicCompareLoweringCommandPilot
compare_lowering_mutation_owner_selection=1
symbolic_compare_lowering_command_pilot_selected=1
mutation_bearing_owner_selected=0
bool_recipe_lowering_executed=0
operand_value_id_resolution=0
rhs_runtime_materialization=0
mir_cmp_emission=0
branch_emission=0
basic_block_mutation=0
value_id_allocation=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-LOWERING-SYMBOLIC-COMMAND-PILOT-001
summary=ok
REPORT

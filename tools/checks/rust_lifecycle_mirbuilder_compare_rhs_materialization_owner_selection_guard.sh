#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-rhs-materialization-owner-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-materialization-owner-selection-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3303-MIRBUILDER-COMPARE-RHS-MATERIALIZATION-OWNER-SELECTION-001.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
SYMBOLIC_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/compare_lowering_symbolic_command_snapshot.hako"
COMPARE_RS="$ROOT_DIR/src/mir/builder/ops/comparison.rs"
SSA_LOCAL_RS="$ROOT_DIR/src/mir/builder/ssa/local.rs"
EMIT_COMPARE_RS="$ROOT_DIR/src/mir/builder/emission/compare.rs"
EMIT_BRANCH_RS="$ROOT_DIR/src/mir/builder/emission/branch.rs"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_lowering_symbolic_command_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" "$SYMBOLIC_IMPL" "$COMPARE_RS" "$SSA_LOCAL_RS" "$EMIT_COMPARE_RS" "$EMIT_BRANCH_RS" "$PREV_GATE"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^compare_lowering_symbolic_command_parity=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "symbolic command parity prerequisite is not green"
fi

python3 - "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" "$SYMBOLIC_IMPL" "$COMPARE_RS" "$SSA_LOCAL_RS" "$EMIT_COMPARE_RS" "$EMIT_BRANCH_RS" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
current_state = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
symbolic_impl = Path(sys.argv[5]).read_text(encoding="utf-8")
compare_rs = Path(sys.argv[6]).read_text(encoding="utf-8")
ssa_local = Path(sys.argv[7]).read_text(encoding="utf-8")
emit_compare = Path(sys.argv[8]).read_text(encoding="utf-8")
emit_branch = Path(sys.argv[9]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderCompareRhsMaterializationOwnerSelectionV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-OWNER-SELECTION-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-COMPARE-LOWERING-SYMBOLIC-COMMAND-PARITY-001", "bad prerequisite")

candidates = {row.get("name"): row for row in fixture.get("candidates") or []}
need(candidates["RhsMaterializationIntentPilot"].get("selected") is True, "intent pilot must be selected")
need(candidates["RhsMaterializationIntentPilot"].get("selected_next_card") == "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PILOT-001", "bad selected next")
for name in [
    "ResolveRhsValueIdNow",
    "EmitRhsMaterializationNow",
    "EmitCompareWithRhsNow",
    "ExternalDesignStop",
]:
    need(candidates[name].get("selected") is False, f"{name} must not be selected")

boundary = fixture.get("selected_boundary") or {}
need(boundary.get("name") == "CompareRhsMaterializationIntentSnapshotV1", "bad selected boundary")
need(boundary.get("input") == "CompareLoweringSymbolicCommandSnapshotV1", "bad input")
need(boundary.get("output") == "read-only RHS materialization intent snapshot", "bad output")
for field in [
    "rhs_bound_kind_code",
    "rhs_bound_i64",
    "rhs_bound_symbol_id",
    "rhs_materialization_kind_code",
    "literal_i64_required",
    "symbol_lookup_required",
    "analysis_only",
]:
    need(field in boundary.get("allowed_fields", []), f"missing allowed field: {field}")
for action in [
    "symbol to ValueId resolution",
    "constant MIR emission",
    "runtime helper emission",
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

claims = fixture.get("claims") or {}
for key in [
    "compare_rhs_materialization_owner_selection",
    "rhs_materialization_intent_pilot_selected",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "rhs_value_id_resolution_selected",
    "rhs_runtime_materialization_selected",
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
    "programjson_var_rhs_full_dispatcher_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

allowed_latest = [
    'latest_card = "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-OWNER-SELECTION-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PILOT-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PARITY-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-OWNER-SELECTION-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-READONLY-RESOLUTION-PILOT-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-READONLY-RESOLUTION-PARITY-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-ACTUAL-VALUEID-RESOLUTION-DESIGN-STOP-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-REQUEST-RESPONSE-ABI-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-CONSULTATION-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-CONTRACT-PARITY-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001"',
    'latest_card = "MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001"',
]
need(any(entry in current_state for entry in allowed_latest), "CURRENT_STATE latest card must point to selection or selected pilot")
need(
    "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PILOT-001" in task_order
    or "RHS materialization intent pilot" in task_order,
    "task-order must retain selected pilot evidence",
)
need("build_command_from_intent(intent): MapBox" in symbolic_impl, "symbolic command owner missing")
need("rhs_bound_kind_code" in symbolic_impl and "rhs_bound_symbol_id" in symbolic_impl, "symbolic command rhs fields missing")
need("build_comparison_op" in compare_rs and "next_value_id" in compare_rs, "Rust comparison owner must still allocate dst")
need("finalize_compare" in compare_rs and "finalize_compare" in ssa_local, "Rust LocalSSA compare finalization must remain owner")
need("MirInstruction::Compare" in emit_compare, "Rust compare emission owner missing")
need("MirInstruction::Branch" in emit_branch, "Rust branch emission owner missing")
need("CompareRhsMaterializationIntentSnapshotV1" in card, "card must describe selected boundary")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-rhs-materialization-owner-selection-guard-v0
token=MIRBUILDER-COMPARE-RHS-MATERIALIZATION-OWNER-SELECTION-001
decision=SelectCompareRhsMaterializationIntentPilot
compare_rhs_materialization_owner_selection=1
rhs_materialization_intent_pilot_selected=1
rhs_value_id_resolution_selected=0
rhs_runtime_materialization_selected=0
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
programjson_var_rhs_full_dispatcher_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PILOT-001
summary=ok
REPORT

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-rhs-valueid-resolution-owner-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-rhs-valueid-resolution-owner-selection-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3306-MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-OWNER-SELECTION-001.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INTENT_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/compare_rhs_materialization_intent_snapshot.hako"
COMPARE_RS="$ROOT_DIR/src/mir/builder/ops/comparison.rs"
SSA_LOCAL_RS="$ROOT_DIR/src/mir/builder/ssa/local.rs"
EMIT_COMPARE_RS="$ROOT_DIR/src/mir/builder/emission/compare.rs"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_materialization_intent_parity_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" "$INTENT_IMPL" "$COMPARE_RS" "$SSA_LOCAL_RS" "$EMIT_COMPARE_RS" "$PREV_GATE"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^compare_rhs_materialization_intent_parity=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "RHS materialization intent parity prerequisite is not green"
fi

python3 - "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" "$INTENT_IMPL" "$COMPARE_RS" "$SSA_LOCAL_RS" "$EMIT_COMPARE_RS" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
current_state = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
intent_impl = Path(sys.argv[5]).read_text(encoding="utf-8")
compare_rs = Path(sys.argv[6]).read_text(encoding="utf-8")
ssa_local = Path(sys.argv[7]).read_text(encoding="utf-8")
emit_compare = Path(sys.argv[8]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderCompareRhsValueIdResolutionOwnerSelectionV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-OWNER-SELECTION-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PARITY-001", "bad prerequisite")

candidates = {row.get("name"): row for row in fixture.get("candidates") or []}
need(candidates["ReadOnlyRhsValueIdResolutionPlanPilot"].get("selected") is True, "read-only plan must be selected")
need(candidates["ReadOnlyRhsValueIdResolutionPlanPilot"].get("selected_next_card") == "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-READONLY-RESOLUTION-PILOT-001", "bad selected next")
for name in [
    "ResolveRhsValueIdNow",
    "EmitLiteralConstantNow",
    "ExecuteSymbolLookupNow",
    "EmitCompareWithResolvedRhsNow",
]:
    need(candidates[name].get("selected") is False, f"{name} must not be selected")

boundary = fixture.get("selected_boundary") or {}
need(boundary.get("name") == "CompareRhsValueIdResolutionPlanSnapshotV1", "bad selected boundary")
need(boundary.get("input") == "CompareRhsMaterializationIntentSnapshotV1", "bad input")
need(boundary.get("output") == "read-only RHS ValueId resolution plan snapshot", "bad output")
for field in [
    "rhs_materialization_kind_code",
    "rhs_bound_kind_code",
    "rhs_bound_i64",
    "rhs_bound_symbol_id",
    "resolution_plan_kind_code",
    "literal_constant_required",
    "symbol_lookup_required",
    "analysis_only",
]:
    need(field in boundary.get("allowed_fields", []), f"missing allowed field: {field}")
for action in [
    "symbol to ValueId resolution",
    "literal constant ValueId allocation",
    "constant MIR emission",
    "runtime helper emission",
    "MIR Compare emission",
    "MIR Branch emission",
    "LocalSSA finalize_compare execution",
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
    "compare_rhs_valueid_resolution_owner_selection",
    "readonly_rhs_valueid_resolution_plan_pilot_selected",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "rhs_value_id_resolution_selected",
    "literal_constant_emission_selected",
    "symbol_lookup_execution_selected",
    "compare_emission_selected",
    "rhs_value_id_resolution",
    "literal_constant_value_id_allocation",
    "constant_mir_emission",
    "runtime_helper_emission",
    "local_ssa_finalize_compare_execution",
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
    'latest_card = "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PARITY-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-OWNER-SELECTION-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-READONLY-RESOLUTION-PILOT-001"',
    'latest_card = "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-READONLY-RESOLUTION-PARITY-001"',
]
need(any(entry in current_state for entry in allowed_latest), "CURRENT_STATE latest card must point to prerequisite parity or selection")
need(
    "MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-OWNER-SELECTION-001" in task_order
    or "RHS ValueId resolution owner selection" in task_order,
    "task-order must retain valueid owner selection evidence",
)
need(
    "MIRBUILDER-COMPARE-RHS-MATERIALIZATION-READONLY-RESOLUTION-PILOT-001" in task_order
    or "read-only RHS ValueId resolution plan pilot" in task_order,
    "task-order must retain selected read-only pilot evidence",
)
need("CompareRhsMaterializationIntentSnapshotV1" in intent_impl, "intent owner missing")
need("rhs_materialization_kind_code" in intent_impl, "intent owner missing materialization kind")
need("build_comparison_op" in compare_rs and "ValueId" in compare_rs, "Rust compare owner must remain ValueId consumer")
need("finalize_compare" in compare_rs and "finalize_compare" in ssa_local, "Rust LocalSSA compare finalization must remain owner")
need("MirInstruction::Compare" in emit_compare, "Rust compare emission owner missing")
need("CompareRhsValueIdResolutionPlanSnapshotV1" in card, "card must describe selected boundary")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-rhs-valueid-resolution-owner-selection-guard-v0
token=MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-OWNER-SELECTION-001
decision=SelectReadOnlyRhsValueIdResolutionPlanPilot
compare_rhs_valueid_resolution_owner_selection=1
readonly_rhs_valueid_resolution_plan_pilot_selected=1
rhs_value_id_resolution_selected=0
literal_constant_emission_selected=0
symbol_lookup_execution_selected=0
compare_emission_selected=0
rhs_value_id_resolution=0
literal_constant_value_id_allocation=0
constant_mir_emission=0
runtime_helper_emission=0
local_ssa_finalize_compare_execution=0
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
selected_next_card=MIRBUILDER-COMPARE-RHS-MATERIALIZATION-READONLY-RESOLUTION-PILOT-001
summary=ok
REPORT

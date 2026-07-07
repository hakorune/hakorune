#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-compare-localssa-finalize-compare-design-stop-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-compare-localssa-finalize-compare-design-stop-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3315-MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
PREV_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_compare_rhs_symbolref_lookup_bridge_gate.sh"
FINALIZE_RS="$ROOT_DIR/src/mir/builder/ssa/local/finalize.rs"
COMPARISON_RS="$ROOT_DIR/src/mir/builder/ops/comparison.rs"
EMIT_COMPARE_RS="$ROOT_DIR/src/mir/builder/emission/compare.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" \
  "$PREV_GATE" "$FINALIZE_RS" "$COMPARISON_RS" "$EMIT_COMPARE_RS"

PREV_OUT="$(guard_cached_run "$TAG" bash "$PREV_GATE")"
if ! grep -q '^symbol_ref_valueid_resolution_no_shadow_local=1$' <<<"$PREV_OUT"; then
  printf '%s\n' "$PREV_OUT" >&2
  guard_fail "$TAG" "SymbolRef lookup bridge prerequisite is not green"
fi

python3 - "$FIXTURE" "$CARD" "$CURRENT_STATE" "$TASK_ORDER" "$FINALIZE_RS" "$COMPARISON_RS" "$EMIT_COMPARE_RS" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
current_state = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
finalize_rs = Path(sys.argv[5]).read_text(encoding="utf-8")
comparison_rs = Path(sys.argv[6]).read_text(encoding="utf-8")
emit_compare_rs = Path(sys.argv[7]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderCompareLocalSsaFinalizeCompareDesignStopV1", "bad kind")
need(fixture.get("token") == "MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001", "bad token")
need(fixture.get("prerequisite") == "MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001", "bad prerequisite")

candidates = {row.get("name"): row for row in fixture.get("candidates") or []}
need(candidates["LocalSsaFinalizeCompareBridgeFirst"].get("selected") is True, "LocalSSA bridge must be selected")
need(
    candidates["LocalSsaFinalizeCompareBridgeFirst"].get("selected_next_card")
    == "MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-BRIDGE-001",
    "bad selected next",
)
for name in [
    "CombineFinalizeAndCompareEmission",
    "SkipLocalSsaForNoShadowValues",
    "OpenBranchEmissionNow",
]:
    need(candidates[name].get("selected") is False, f"{name} must not be selected")

claims = fixture.get("claims") or {}
for key in [
    "compare_localssa_finalize_compare_design_stop",
    "localssa_finalize_compare_bridge_selected",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "compare_emission_selected_now",
    "branch_emission_selected_now",
    "localssa_finalize_compare_execution",
    "mir_compare_emission",
    "mir_branch_emission",
    "bool_result_type_publication",
    "basicblock_mutation",
    "route_selection",
    "runtime_route_switch",
    "programjson_runtime_authority",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need("pub fn finalize_compare" in finalize_rs, "Rust LocalSSA finalize_compare owner missing")
need("*lhs = cmp_operand(builder, *lhs)" in finalize_rs, "lhs cmp_operand step missing")
need("*rhs = cmp_operand(builder, *rhs)" in finalize_rs, "rhs cmp_operand step missing")
need("finalize_compare(self, &mut lhs2, &mut rhs2)" in comparison_rs, "comparison owner must still call finalize_compare")
need("emission::compare::emit_to(self, dst, op, lhs2, rhs2)" in comparison_rs, "comparison owner must still call compare emission after finalize")
need("MirInstruction::Compare" in emit_compare_rs, "compare emission owner missing")
need("value_types.insert(dst, MirType::Bool)" in emit_compare_rs, "Bool type publication owner missing")

allowed_latest = [
    'latest_card = "MIRBUILDER-COMPARE-RHS-SYMBOLREF-LOOKUP-BRIDGE-001"',
    'latest_card = "MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001"',
    'latest_card = "MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-BRIDGE-001"',
    'latest_card = "MIRBUILDER-COMPARE-MIR-COMPARE-EMISSION-BRIDGE-001"',
    'latest_card = "MIRBUILDER-COMPARE-BRANCH-EMISSION-DESIGN-STOP-001"',
    'latest_card = "MIRBUILDER-COMPARE-BRANCH-EMISSION-BRIDGE-001"',
    'latest_card = "MIRBUILDER-COMPARE-BOOLRECIPE-TO-MIR-COMPARE-BRANCH-CLOSEOUT-001"',
]
need(any(entry in current_state for entry in allowed_latest), "CURRENT_STATE latest card must point to prerequisite or 3315")
need("MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-BRIDGE-001" in task_order, "task-order must name selected LocalSSA bridge")
need("LocalSSA bridge implementation: `0`" in card, "card must keep implementation unclaimed")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-localssa-finalize-compare-design-stop-v0
token=MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-DESIGN-STOP-001
decision=SelectLocalSsaFinalizeCompareBridgeFirst
compare_localssa_finalize_compare_design_stop=1
localssa_finalize_compare_bridge_selected=1
compare_emission_selected_now=0
branch_emission_selected_now=0
localssa_finalize_compare_execution=0
mir_compare_emission=0
mir_branch_emission=0
bool_result_type_publication=0
basicblock_mutation=0
route_selection=0
runtime_route_switch=0
programjson_runtime_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-LOCALSSA-FINALIZE-COMPARE-BRIDGE-001
summary=ok
REPORT

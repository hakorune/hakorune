#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-loop-break-plan-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_loop_break_plan_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-break-plan-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1933-MIRBUILDER-LOOP-BREAK-PLAN-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-break-plan-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1933-MIRBUILDER-LOOP-BREAK-PLAN-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-LOOP-BREAK-PLAN-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderLoopBreakPlanProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

state = fixture["input_state"]
if state["source_count"] != 7:
    raise SystemExit("source count drift")
if state["selected_cluster_id"] != "projection_policy::UnsupportedDirectShape::shape.loop_break_plan::FixtureMapped::LoopBreakPlanCluster::borrow=NoBorrow::control=StructuredLoop::type=Known::call=AllKnown::verifier=Present":
    raise SystemExit("selected cluster drift")
if set(state["selected_cluster_ids"]) != {
    "projection_policy::UnsupportedDirectShape::shape.loop_break_plan::FixtureMapped::LoopBreakPlanCluster::borrow=NoBorrow::control=StructuredLoop::type=Known::call=AllKnown::verifier=Present",
    "projection_policy::UnsupportedDirectShape::shape.loop_break_plan::FixtureMapped::LoopBreakPlanCluster::borrow=NoReturnedBorrow::control=StructuredLoop::type=Known::call=AllKnown::verifier=Present",
}:
    raise SystemExit("selected cluster ids drift")

expected_symbols = {
    "has_continue_statement",
    "has_return_statement",
    "matches_ge_zero",
    "matches_eq_empty_string",
    "has_assignment_after",
    "matches_substring_at_loop_var",
    "collect_whitespace_terms",
}
symbols = {surface["symbol"] for surface in fixture["source_surfaces"]}
if symbols != expected_symbols:
    raise SystemExit(f"source symbol drift: {sorted(symbols)}")
if any(surface["return_type"] != "bool" for surface in fixture["source_surfaces"]):
    raise SystemExit("all LoopBreakPlan descriptor surfaces must return bool")
if any(surface["owner_edge_confidence"] != "FixtureMapped" for surface in fixture["source_surfaces"]):
    raise SystemExit("owner confidence drift")

descriptor = fixture["loop_break_plan_predicate_descriptor"]
if descriptor["descriptor_id"] != "loop_break_plan_predicate_and_accumulator_helpers_v1":
    raise SystemExit("descriptor id drift")
if descriptor["predicate_count"] != 6:
    raise SystemExit("predicate count drift")
if descriptor["accumulator_helper_count"] != 1:
    raise SystemExit("accumulator helper count drift")
if descriptor["return_contract"] != "bool":
    raise SystemExit("return contract drift")
if descriptor["returned_borrow"] != 0:
    raise SystemExit("LoopBreakPlan descriptor must not return borrow")
if descriptor["mutation_frame"] != [
    "collect_whitespace_terms mutates caller-owned haystack_var Option",
    "collect_whitespace_terms mutates caller-owned direction Option",
    "collect_whitespace_terms appends to caller-owned delimiters Vec",
]:
    raise SystemExit("mutation frame drift")

policy = fixture["selected_policy"]
if policy["policy"] != "LoopBreakPlanPredicateAndAccumulatorDescriptor":
    raise SystemExit("selected policy drift")
if policy["descriptor_selected"] is not True:
    raise SystemExit("descriptor must be selected")
if policy["hako_projection_selected"] is not False:
    raise SystemExit("Hako projection must not be selected")

decision = fixture["decision"]
if decision["kind"] != "SelectProjectionPolicyDescriptor":
    raise SystemExit("decision kind drift")
if decision["selected_next_card"] != "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
if claims.get("descriptor_selected") != 1:
    raise SystemExit("descriptor selected claim must be 1")
for key in [
    "manual_family_selection",
    "hako_projection_selected",
    "hako_generation",
    "hako_adopted_decision",
    "native_seed_materialization",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"non-claim must be 0: {key}")

provenance = fixture["provenance"]
if provenance["tool_role"] != "FactsAdapterGuardOrchestrator":
    raise SystemExit("tool role drift")
if provenance["semantic_projection_inference"] != 0:
    raise SystemExit("tool must not infer semantic projection")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-loop-break-plan-projection-policy-v0
source_count=7
policy=LoopBreakPlanPredicateAndAccumulatorDescriptor
descriptor_selected=1
hako_projection_selected=0
selected_next_card=MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT

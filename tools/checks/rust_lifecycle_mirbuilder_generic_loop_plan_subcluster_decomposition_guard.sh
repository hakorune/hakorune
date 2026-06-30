#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-generic-loop-plan-subcluster-decomposition-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_generic_loop_plan_subcluster_decomposition.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-plan-subcluster-decomposition-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1907-MIRBUILDER-GENERIC-LOOP-PLAN-SUBCLUSTER-DECOMPOSITION-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-plan-subcluster-decomposition-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1907-MIRBUILDER-GENERIC-LOOP-PLAN-SUBCLUSTER-DECOMPOSITION-001.md").read_text()

token = "MIRBUILDER-GENERIC-LOOP-PLAN-SUBCLUSTER-DECOMPOSITION-001"
if fixture.get("kind") != "MirBuilderGenericLoopPlanSubclusterDecompositionV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

state = fixture["input_state"]
if state["source_cluster"] != "GenericLoopPlanCluster":
    raise SystemExit("source cluster drift")
if state["input_candidate_count"] != 66:
    raise SystemExit("input candidate count drift")
if state["source_module_count"] != 17:
    raise SystemExit("source module count drift")
if state["scanned_function_count"] != 88:
    raise SystemExit("scanned function count drift")

expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.generic_loop_plan",
    "borrow_axis": "NoBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if fixture["selection_axes"] != expected_axes:
    raise SystemExit(f"selection axes drift: {fixture['selection_axes']}")

expected_counts = {
    "BodyCheckExprMatchers": 12,
    "BodyCheckExtractors": 13,
    "BodyCheckShapeDetectorUtils": 7,
    "BodyCheckShapeDetectors": 34,
    "BodyCheckStepValidation": 5,
    "StatementClassifierPredicates": 17,
}
if fixture["subcluster_counts"] != expected_counts:
    raise SystemExit(f"subcluster count drift: {fixture['subcluster_counts']}")
if len(fixture["source_surfaces"]) != 88:
    raise SystemExit("source surface count drift")
if len({surface["source_id"] for surface in fixture["source_surfaces"]}) != 88:
    raise SystemExit("source surfaces must be classified exactly once")

subclusters = {item["subcluster_id"]: item for item in fixture["subclusters"]}
if set(subclusters) != set(expected_counts):
    raise SystemExit(f"subcluster id drift: {sorted(subclusters)}")
if subclusters["BodyCheckExprMatchers"]["selection_eligible"] is not True:
    raise SystemExit("BodyCheckExprMatchers must be selected first")
for name, item in subclusters.items():
    if name != "BodyCheckExprMatchers" and item["selection_eligible"] is not False:
        raise SystemExit(f"only BodyCheckExprMatchers may be selection eligible: {name}")

policy = fixture["decomposition_policy"]
if policy["whole_cluster_projection_policy_selected"] is not False:
    raise SystemExit("whole GenericLoopPlan projection policy must not be selected")
if policy["whole_cluster_keep_parent_owner_selected"] is not False:
    raise SystemExit("whole GenericLoopPlan keep-parent decision must not be selected")
if policy["path_role_decomposition"] is not True:
    raise SystemExit("path-role decomposition claim missing")
if policy["candidate_count_as_proof"] != 0:
    raise SystemExit("candidate count must not be proof")

decision = fixture["decision"]
if decision["kind"] != "SelectSubclusterProjectionPolicy":
    raise SystemExit("decision kind drift")
if decision["selected_subcluster_id"] != "BodyCheckExprMatchers":
    raise SystemExit("selected subcluster drift")
if decision["selected_next_card"] != "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-EXPR-MATCHERS-PROJECTION-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "manual_family_selection",
    "whole_cluster_projection_policy",
    "whole_cluster_keep_parent_owner",
    "candidate_count_as_proof",
    "runtime_or_projection_policy_by_name",
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
    raise SystemExit("decomposition tool must not infer semantic projection")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-generic-loop-plan-subcluster-decomposition-v0
source_cluster=GenericLoopPlanCluster
input_candidate_count=66
scanned_function_count=88
subcluster_count=6
selected_subcluster=BodyCheckExprMatchers
selected_next_card=MIRBUILDER-GENERIC-LOOP-BODY-CHECK-EXPR-MATCHERS-PROJECTION-POLICY-001
whole_cluster_projection_policy=0
candidate_count_as_proof=0
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT

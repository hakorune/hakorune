#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-generic-loop-body-check-expr-matchers-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_generic_loop_body_check_expr_matchers_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-expr-matchers-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1908-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-EXPR-MATCHERS-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generic-loop-body-check-expr-matchers-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1908-MIRBUILDER-GENERIC-LOOP-BODY-CHECK-EXPR-MATCHERS-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-EXPR-MATCHERS-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderGenericLoopBodyCheckExprMatchersProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

state = fixture["input_state"]
if state["source_parent_cluster"] != "GenericLoopPlanCluster":
    raise SystemExit("source parent cluster drift")
if state["source_subcluster_id"] != "BodyCheckExprMatchers":
    raise SystemExit("source subcluster drift")
if state["source_count"] != 12:
    raise SystemExit("source count drift")

expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.generic_loop_body_check_expr_matchers",
    "borrow_axis": "NoBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if fixture["selection_axes"] != expected_axes:
    raise SystemExit(f"selection axes drift: {fixture['selection_axes']}")

expected_counts = {
    "CallExprMatchers": 2,
    "CompareExprMatchers": 3,
    "CompositeTrimConditionMatcher": 1,
    "ControlReturnExprMatchers": 6,
}
if fixture["expr_matcher_subcluster_counts"] != expected_counts:
    raise SystemExit(f"expr matcher count drift: {fixture['expr_matcher_subcluster_counts']}")
if len(fixture["source_surfaces"]) != 12:
    raise SystemExit("source surface count drift")
if len({surface["source_id"] for surface in fixture["source_surfaces"]}) != 12:
    raise SystemExit("source surfaces must be classified exactly once")

subclusters = {item["expr_matcher_subcluster_id"]: item for item in fixture["expr_matcher_subclusters"]}
if set(subclusters) != set(expected_counts):
    raise SystemExit(f"subcluster id drift: {sorted(subclusters)}")
if subclusters["CallExprMatchers"]["selection_eligible"] is not True:
    raise SystemExit("CallExprMatchers must be selected first")
for name, item in subclusters.items():
    if name != "CallExprMatchers" and item["selection_eligible"] is not False:
        raise SystemExit(f"only CallExprMatchers may be selection eligible: {name}")
if subclusters["CompositeTrimConditionMatcher"]["next_owner_kind"] != "CompositeDecomposition":
    raise SystemExit("trim-condition matcher must remain composite decomposition")

policy = fixture["decomposition_policy"]
if policy["whole_expr_matcher_projection_selected"] is not False:
    raise SystemExit("whole expression matcher projection must not be selected")
if policy["module_role_decomposition"] is not True:
    raise SystemExit("module-role decomposition claim missing")
if policy["candidate_count_as_proof"] != 0:
    raise SystemExit("candidate count must not be proof")

decision = fixture["decision"]
if decision["kind"] != "SelectExpressionMatcherSubcluster":
    raise SystemExit("decision kind drift")
if decision["selected_expr_matcher_subcluster_id"] != "CallExprMatchers":
    raise SystemExit("selected expression matcher subcluster drift")
if decision["selected_next_card"] != "MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CALL-EXPR-MATCHERS-PROJECTION-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "manual_family_selection",
    "whole_expr_matcher_projection",
    "projection_surface_selected",
    "composite_trim_policy_selected",
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
    raise SystemExit("policy tool must not infer semantic projection")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-generic-loop-body-check-expr-matchers-projection-policy-v0
source_subcluster=BodyCheckExprMatchers
source_count=12
subcluster_count=4
selected_subcluster=CallExprMatchers
selected_next_card=MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CALL-EXPR-MATCHERS-PROJECTION-POLICY-001
whole_expr_matcher_projection=0
candidate_count_as_proof=0
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT

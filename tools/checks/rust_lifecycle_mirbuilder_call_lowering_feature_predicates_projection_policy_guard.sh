#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-call-lowering-feature-predicates-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_call_lowering_feature_predicates_projection_policy.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-feature-predicates-projection-policy-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1901-MIRBUILDER-CALL-LOWERING-FEATURE-PREDICATES-PROJECTION-POLICY-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-feature-predicates-projection-policy-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1901-MIRBUILDER-CALL-LOWERING-FEATURE-PREDICATES-PROJECTION-POLICY-001.md").read_text()

token = "MIRBUILDER-CALL-LOWERING-FEATURE-PREDICATES-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderCallLoweringFeaturePredicatesProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["selected_subcluster_id"] != "CallFeaturePredicates":
    raise SystemExit("selected subcluster drift")
if fixture["input_state"]["source_count"] != 3:
    raise SystemExit("source count drift")

symbols = [surface["symbol"] for surface in fixture["source_surfaces"]]
expected_symbols = ["is_unified_call_enabled", "is_pure_method", "contains_value_return"]
if symbols != expected_symbols:
    raise SystemExit(f"selected surface drift: {symbols}")

expected_counts = {
    "PureMethodCatalog": 1,
    "UnifiedCallModeGate": 1,
    "ValueReturnAstScan": 1,
}
if fixture["feature_subcluster_counts"] != expected_counts:
    raise SystemExit(f"feature subcluster count drift: {fixture['feature_subcluster_counts']}")

subclusters = {item["feature_subcluster_id"]: item for item in fixture["feature_subclusters"]}
if set(subclusters) != set(expected_counts):
    raise SystemExit(f"feature subcluster id drift: {sorted(subclusters)}")
if subclusters["UnifiedCallModeGate"]["selection_eligible"] is not True:
    raise SystemExit("UnifiedCallModeGate must be selected first")
for name, item in subclusters.items():
    if name != "UnifiedCallModeGate" and item["selection_eligible"] is not False:
        raise SystemExit(f"only UnifiedCallModeGate may be selection eligible: {name}")

axes = fixture["selection_axes"]
expected_axes = {
    "owner_edge_confidence": "FixtureMapped",
    "stable_deny_reason": "UnsupportedDirectShape",
    "shape_signature": "shape.call_lowering",
    "borrow_axis": "NoBorrow",
    "type_transport_axis": "Known",
    "verifier_or_oracle_state": "Present",
}
if axes != expected_axes:
    raise SystemExit(f"selection axes drift: {axes}")

markers_by_symbol = {
    "is_unified_call_enabled": [
        "builder_unified_call_mode",
        "default ON during development; explicit opt-out supported",
    ],
    "is_pure_method": ["StringBox", "IntegerBox", "FloatBox", "BoolBox"],
    "contains_value_return": [
        "ASTNode::Return { value: Some(_), .. }",
        "ASTNode::If",
        "ASTNode::Loop",
        "ASTNode::TryCatch",
        "ASTNode::Program",
        "ASTNode::ScopeBox",
        "ASTNode::FunctionDeclaration",
    ],
}
for surface in fixture["source_surfaces"]:
    expected = markers_by_symbol[surface["symbol"]]
    if surface["source_markers"] != expected:
        raise SystemExit(f"source marker drift for {surface['symbol']}: {surface['source_markers']}")

policy = fixture["decomposition_policy"]
if policy["whole_feature_predicate_projection_selected"] is not False:
    raise SystemExit("whole feature predicate projection must not be selected")
if policy["unified_call_mode_gate_first"] is not True:
    raise SystemExit("UnifiedCallModeGate must be first")

decision = fixture["decision"]
if decision["kind"] != "SelectFeaturePredicateSubcluster":
    raise SystemExit("decision kind drift")
if decision["selected_feature_subcluster_id"] != "UnifiedCallModeGate":
    raise SystemExit("selected feature subcluster drift")
if decision["selected_next_card"] != "MIRBUILDER-CALL-LOWERING-UNIFIED-CALL-MODE-GATE-PROJECTION-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "manual_family_selection",
    "whole_feature_predicate_projection",
    "projection_surface_selected",
    "registry_descriptor_selected",
    "ast_traversal_projection_selected",
    "ad_hoc_by_name_policy",
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
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-call-lowering-feature-predicates-projection-policy-v0
subcluster=CallFeaturePredicates
feature_subcluster_count=3
selected_feature_subcluster=UnifiedCallModeGate
whole_feature_predicate_projection=0
projection_surface_selected=0
registry_descriptor_selected=0
ast_traversal_projection_selected=0
selected_next_card=MIRBUILDER-CALL-LOWERING-UNIFIED-CALL-MODE-GATE-PROJECTION-POLICY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT

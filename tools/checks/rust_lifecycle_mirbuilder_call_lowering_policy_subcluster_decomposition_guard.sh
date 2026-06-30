#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-call-lowering-policy-subcluster-decomposition-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_call_lowering_policy_subcluster_decomposition.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-policy-subcluster-decomposition-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1896-MIRBUILDER-CALL-LOWERING-POLICY-SUBCLUSTER-DECOMPOSITION-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-call-lowering-policy-subcluster-decomposition-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1896-MIRBUILDER-CALL-LOWERING-POLICY-SUBCLUSTER-DECOMPOSITION-001.md").read_text()

token = "MIRBUILDER-CALL-LOWERING-POLICY-SUBCLUSTER-DECOMPOSITION-001"
if fixture.get("kind") != "MirBuilderCallLoweringPolicySubclusterDecompositionV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

if fixture["input_state"]["source_count"] != 12:
    raise SystemExit("source count drift")
if fixture["input_state"]["source_cluster_rank"] != 1:
    raise SystemExit("source cluster rank drift")

expected_symbols = [
    "is_unified_call_enabled",
    "is_pure_method",
    "is_env_interface",
    "generate_method_function_name",
    "generate_self_recursion_warning",
    "has_method",
    "is_builtin_function",
    "is_commonly_shadowed_method",
    "is_extern_function",
    "suggest_resolution",
    "contains_value_return",
    "is_math_function",
]
symbols = [surface["symbol"] for surface in fixture["source_surfaces"]]
if symbols != expected_symbols:
    raise SystemExit(f"selected surface drift: {symbols}")
if len(set(surface["source_id"] for surface in fixture["source_surfaces"])) != 12:
    raise SystemExit("source surfaces must be classified exactly once")

expected_counts = {
    "BuiltinGlobalFunctionRegistry": 2,
    "CallFeaturePredicates": 3,
    "CallNameCanonicalizationHelpers": 1,
    "DiagnosticStringHelpers": 3,
    "ExternInterfaceRegistry": 2,
    "StaticReceiverMethodCatalog": 1,
}
if fixture["subcluster_counts"] != expected_counts:
    raise SystemExit(f"subcluster count drift: {fixture['subcluster_counts']}")

subclusters = {item["subcluster_id"]: item for item in fixture["subclusters"]}
if set(subclusters) != set(expected_counts):
    raise SystemExit(f"subcluster id drift: {sorted(subclusters)}")
if subclusters["DiagnosticStringHelpers"]["selection_eligible"] is not True:
    raise SystemExit("diagnostic helper subcluster must be selected first")
for name, item in subclusters.items():
    if name != "DiagnosticStringHelpers" and item["selection_eligible"] is not False:
        raise SystemExit(f"only diagnostic helpers may be selection eligible: {name}")

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

policy = fixture["decomposition_policy"]
if policy["whole_cluster_projection_policy_selected"] is not False:
    raise SystemExit("whole CallLowering projection policy must not be selected")
if policy["whole_cluster_keep_parent_owner_selected"] is not False:
    raise SystemExit("whole CallLowering keep-parent decision must not be selected")
if policy["registry_tables_require_descriptor_fixture"] is not True:
    raise SystemExit("registry subclusters must require descriptor fixtures")
if policy["diagnostic_helpers_first"] is not True:
    raise SystemExit("diagnostic helpers must be first")

decision = fixture["decision"]
if decision["kind"] != "SelectSubclusterProjectionPolicy":
    raise SystemExit("decision kind drift")
if decision["selected_subcluster_id"] != "DiagnosticStringHelpers":
    raise SystemExit("selected subcluster drift")
if decision["selected_next_card"] != "MIRBUILDER-CALL-LOWERING-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "manual_family_selection",
    "whole_cluster_projection_policy",
    "whole_cluster_keep_parent_owner",
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
output_contract=rust-lifecycle-mirbuilder-call-lowering-policy-subcluster-decomposition-v0
source_cluster=CallLoweringCluster
source_count=12
subcluster_count=6
selected_subcluster=DiagnosticStringHelpers
selected_next_card=MIRBUILDER-CALL-LOWERING-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001
whole_cluster_projection_policy=0
whole_cluster_keep_parent_owner=0
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT

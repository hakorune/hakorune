#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-missing-projection-policy-joinir-route-verify-cluster-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_missing_projection_policy_joinir_route_verify_cluster.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-joinir-route-verify-cluster-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1937-MIRBUILDER-MISSING-PROJECTION-POLICY-JOINIR-ROUTE-VERIFY-CLUSTER-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-joinir-route-verify-cluster-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1937-MIRBUILDER-MISSING-PROJECTION-POLICY-JOINIR-ROUTE-VERIFY-CLUSTER-001.md").read_text()

token = "MIRBUILDER-MISSING-PROJECTION-POLICY-JOINIR-ROUTE-VERIFY-CLUSTER-001"
if fixture.get("kind") != "MirBuilderMissingProjectionPolicyJoinIRRouteVerifyClusterV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

state = fixture["input_state"]
if state["input_joinir_route_verify_cluster_count"] != 206:
    raise SystemExit("JoinIRRouteVerifyCluster input count drift")
if state["current_blocker"] != "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001":
    raise SystemExit("current blocker drift")

summary = fixture["summary"]
if summary["input_joinir_route_verify_cluster_count"] != 206:
    raise SystemExit("summary input count drift")
if summary["subcluster_count"] != 81:
    raise SystemExit("subcluster count drift")
if summary["selection_eligible_subcluster_count"] != 42:
    raise SystemExit("eligible subcluster count drift")
if summary["selected_subcluster_id"] is not None:
    raise SystemExit("selected subcluster must remain null")
if summary["role_counts"]["facts_or_recognizer"] != 16:
    raise SystemExit("facts role count drift")
if summary["role_counts"]["joinir_merge_helper"] != 13:
    raise SystemExit("joinir merge helper role count drift")
if summary["role_counts"]["joinir_merge_rewriter"] != 12:
    raise SystemExit("joinir merge rewriter role count drift")
if summary["type_transport_axis_counts"]["Known"] != 82:
    raise SystemExit("known type transport count drift")
if summary["type_transport_axis_counts"]["ResultCarrierNeedsVerifier"] != 62:
    raise SystemExit("result carrier count drift")
if summary["return_family_counts"]["result"] != 62:
    raise SystemExit("result return family count drift")

subclusters = fixture["subclusters"]
if len(subclusters) != 81:
    raise SystemExit("subcluster list length drift")
if len({item["subcluster_id"] for item in subclusters}) != 81:
    raise SystemExit("subcluster IDs must be unique")
if sum(item["source_id_count"] for item in subclusters) != 206:
    raise SystemExit("subcluster partition count drift")

decision = fixture["decision"]
if decision["kind"] != "KeepStopped":
    raise SystemExit("decision kind drift")
if decision["reason_token"] != "AmbiguousJoinIRRouteVerifyProjectionSubclusters":
    raise SystemExit("decision reason drift")
if decision["selected_next_card"] != "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001":
    raise SystemExit("decision next card drift")
if decision["selected_subcluster_id"] is not None:
    raise SystemExit("selected subcluster must be null")

claims = fixture["claims"]
for key in [
    "source_report_consumed",
    "projection_priority_consumed",
    "previous_parent_owned_policy_consumed",
    "all_joinir_route_verify_items_partitioned_exactly_once",
    "subcluster_ids_are_stable",
    "subcluster_reason_tokens_are_stable",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"required claim must be 1: {key}")
if claims.get("input_joinir_route_verify_cluster_count") != 206:
    raise SystemExit("claim input count drift")
for key in [
    "manual_family_selection",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_edit_authority",
    "hako_generation",
    "hako_adopted_decision",
    "native_source_seed_materialization",
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
output_contract=rust-lifecycle-mirbuilder-missing-projection-policy-joinir-route-verify-cluster-v0
input_joinir_route_verify_cluster_count=206
subcluster_count=81
selection_eligible_subcluster_count=42
selected_next_card=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT

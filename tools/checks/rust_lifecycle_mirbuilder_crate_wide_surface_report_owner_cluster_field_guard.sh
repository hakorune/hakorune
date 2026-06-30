#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-crate-wide-surface-report-owner-cluster-field-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_crate_wide_surface_report_owner_cluster_field.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-surface-report-owner-cluster-field-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1940-MIRBUILDER-CRATE-WIDE-SURFACE-REPORT-OWNER-CLUSTER-FIELD-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-surface-report-owner-cluster-field-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1940-MIRBUILDER-CRATE-WIDE-SURFACE-REPORT-OWNER-CLUSTER-FIELD-001.md").read_text()

token = "MIRBUILDER-CRATE-WIDE-SURFACE-REPORT-OWNER-CLUSTER-FIELD-001"
if fixture.get("kind") != "MirBuilderCrateWideSurfaceReportOwnerClusterFieldV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

state = fixture["input_state"]
if state["reported_item_count"] != 1584:
    raise SystemExit("reported item count drift")
if state["missing_projection_policy_count"] != 1384:
    raise SystemExit("missing projection count drift")
if state["current_blocker"] != "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001":
    raise SystemExit("current blocker drift")
if fixture["field_gaps"] != []:
    raise SystemExit("owner cluster field gaps must be empty")

summary = fixture["owner_field_summary"]
if summary["missing_projection_cluster_counts"]["OtherMissingProjectionPolicyCluster"] != 185:
    raise SystemExit("OtherMissingProjectionPolicyCluster count drift")
if summary["missing_projection_cluster_counts"]["JoinIRRouteRegistryCluster"] != 37:
    raise SystemExit("JoinIRRouteRegistryCluster count drift")
if summary["missing_projection_cluster_counts"]["FastMemCluster"] != 19:
    raise SystemExit("FastMemCluster count drift")
if summary["owner_edge_confidence_counts"]["FixtureMapped"] != 1199:
    raise SystemExit("FixtureMapped count drift")
if summary["owner_edge_confidence_counts"]["None"] != 364:
    raise SystemExit("None confidence count drift")
if summary["known_owner_edge_missing_by_classification"]["MissingProjectionPolicy"] != 185:
    raise SystemExit("missing owner edge projection count drift")

residual = fixture["residual_owner_clusters"]
if residual[0]["cluster"] != "OtherMissingProjectionPolicyCluster":
    raise SystemExit("OtherMissingProjectionPolicyCluster must be first residual field gap")
if residual[0]["count"] != 185:
    raise SystemExit("OtherMissingProjectionPolicyCluster residual count drift")

decision = fixture["decision"]
if decision["kind"] != "SelectOtherOwnerClusterDecomposition":
    raise SystemExit("decision kind drift")
if decision["reason_token"] != "OtherOwnerClusterFieldRequiresDecomposition":
    raise SystemExit("decision reason drift")
if decision["selected_cluster"] != "OtherMissingProjectionPolicyCluster":
    raise SystemExit("selected cluster drift")
if decision["selected_next_card"] != "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-CLUSTER-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "source_report_consumed",
    "projection_cluster_resolution_consumed",
    "projection_priority_consumed",
    "owner_cluster_field_audited",
    "likely_owner_cluster_present_for_every_item",
    "owner_edge_confidence_present_for_every_item",
    "known_owner_edge_field_present_for_every_item",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"required claim must be 1: {key}")
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
output_contract=rust-lifecycle-mirbuilder-crate-wide-surface-report-owner-cluster-field-v0
reported_item_count=1584
missing_projection_policy_count=1384
field_gaps=0
residual_cluster=OtherMissingProjectionPolicyCluster
residual_cluster_count=185
selected_next_card=MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-CLUSTER-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT

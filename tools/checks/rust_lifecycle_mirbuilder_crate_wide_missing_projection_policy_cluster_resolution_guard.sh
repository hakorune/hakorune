#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_crate_wide_missing_projection_policy_cluster_resolution.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1875-MIRBUILDER-CRATE-WIDE-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-001.md"
REPORT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
NEXT_OWNER="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-unconverted-surface-next-owner-resolution-v0.json"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD" "$REPORT" "$NEXT_OWNER"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json").read_text())
report = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json").read_text())
next_owner = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-unconverted-surface-next-owner-resolution-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1875-MIRBUILDER-CRATE-WIDE-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-001.md").read_text()

token = "MIRBUILDER-CRATE-WIDE-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-001"
if fixture.get("kind") != "MirBuilderCrateWideMissingProjectionPolicyClusterResolutionV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if f"# 1875 - {token}" not in card:
    raise SystemExit("card token mismatch")

if fixture["input_state"]["selected_priority"] != "MissingProjectionPolicy":
    raise SystemExit("selected priority drift")
if fixture["input_state"]["selected_priority_candidate_count"] != 1396:
    raise SystemExit("selected priority count drift")
if next_owner["candidate_pool"]["selected_priority_candidate_count"] != 1396:
    raise SystemExit("next-owner selected count drift")
if report["summary"]["missing_projection_policy_count"] != 1396:
    raise SystemExit("report missing projection count drift")

summary = fixture["summary"]
if summary["input_candidate_count"] != 1396:
    raise SystemExit("input candidate count drift")
if summary["cluster_count"] != len(fixture["clusters"]):
    raise SystemExit("cluster count mismatch")
if summary["duplicate_cluster_id_count"] != 0:
    raise SystemExit("cluster ids must be unique")
if summary["legacy_cluster_id_collision_count"] < 1:
    raise SystemExit("legacy cluster id collisions should remain visible")
if summary["selection_eligible_cluster_count"] != 42:
    raise SystemExit("selection eligible cluster count drift")
if summary["owner_edge_confidence_counts"] != {"FixtureMapped": 1211, "None": 185}:
    raise SystemExit("owner edge confidence counts drift")
if summary["heuristic_or_unmapped_count"] != 185:
    raise SystemExit("heuristic/unmapped count drift")
if summary["exact_owner_confidence_count"] != 0:
    raise SystemExit("exact owner confidence must be zero")
if summary["fixture_mapped_count"] != 1211:
    raise SystemExit("fixture mapped count drift")
if summary["missing_stable_deny_reason_count"] != 0:
    raise SystemExit("stable deny reason count drift")
if summary["missing_verifier_or_oracle_count"] != 0:
    raise SystemExit("verifier/oracle count drift")
if summary["missing_shape_signature_count"] < 1:
    raise SystemExit("missing shape signatures should be visible")
if summary["mapped_unknown_shape_count"] != 0:
    raise SystemExit("mapped unknown shape count drift")

for cluster in fixture["clusters"]:
    if "legacy_cluster_id" not in cluster:
        raise SystemExit("cluster missing legacy_cluster_id")
    if cluster["cluster_id"] == cluster["legacy_cluster_id"]:
        raise SystemExit("cluster_id must be axis-qualified beyond legacy_cluster_id")
    if cluster["owner_edge_confidence"] not in {"FixtureMapped", "None"}:
        raise SystemExit("unexpected owner confidence")
    if cluster["selection_eligible"] is True:
        if cluster["owner_edge_confidence"] != "FixtureMapped":
            raise SystemExit("eligible cluster must be FixtureMapped")
        if cluster["blocked_by"]:
            raise SystemExit("eligible cluster must not be blocked")
        if cluster["next_owner_kind"] != "ProjectionPolicy":
            raise SystemExit("eligible cluster must select projection policy owner kind")
        if not cluster["next_card"]:
            raise SystemExit("eligible cluster must expose next card")
        continue
    if cluster["owner_edge_confidence"] == "None" and "NoExactOrFixtureMappedOwnerEdge" not in cluster["blocked_by"]:
        raise SystemExit("cluster missing owner-edge confidence blocker")
    if cluster["next_owner_kind"] != "None":
        raise SystemExit("cluster must not select projection owner")
    if cluster["next_card"] is not None:
        raise SystemExit("cluster next card must be null")

decision = fixture["decision"]
if decision["kind"] != "SelectProjectionPolicyClusterPriorityResolution":
    raise SystemExit("decision kind drift")
if decision["reason_token"] != "AmbiguousProjectionPolicyClusters":
    raise SystemExit("reason token drift")
if decision["selected_next_card"] != "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001":
    raise SystemExit("selected next card drift")
if decision["selected_cluster_id"] is not None:
    raise SystemExit("selected cluster must be null")

claims = fixture["claims"]
if claims["input_missing_projection_policy_count"] != 1396:
    raise SystemExit("claim input count drift")
for key in [
    "all_missing_projection_policy_items_clustered_exactly_once",
    "cluster_id_is_stable",
    "cluster_id_is_unique",
    "legacy_cluster_id_preserved",
    "owner_edge_confidence_recorded",
    "heuristic_or_none_owner_edge_not_selectable",
    "stable_deny_reason_required",
    "shape_signature_recorded",
    "unknown_shape_not_selected_as_projection_policy",
    "ambiguous_result_keeps_design_stop",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"positive claim must be 1: {key}")
for key in [
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_edit_authority",
    "manual_family_selection",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "family_name_based_policy",
    "hako_emission",
    "hako_adopted_decision",
    "native_source_seed_materialization",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"non-claim must be 0: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0
decision=SelectProjectionPolicyClusterPriorityResolution
reason_token=AmbiguousProjectionPolicyClusters
input_missing_projection_policy_count=1396
selection_eligible_cluster_count=42
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT

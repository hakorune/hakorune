#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-projection-policy-cluster-priority-resolution-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_projection_policy_cluster_priority_resolution.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-projection-policy-cluster-priority-resolution-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1879-MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-projection-policy-cluster-priority-resolution-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1879-MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001.md").read_text()

token = "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001"
if fixture.get("kind") != "MirBuilderProjectionPolicyClusterPriorityResolutionV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

summary = fixture["summary"]
if summary["eligible_cluster_count"] != 42:
    raise SystemExit("eligible cluster count drift")
if summary["excluded_existing_decision_cluster_count"] != 21:
    raise SystemExit("excluded existing decision count drift")
if summary["selectable_cluster_count"] != 21:
    raise SystemExit("selectable cluster count drift")
if summary["selected_candidate_count"] != 2:
    raise SystemExit("selected candidate count drift")
expected_cluster = "projection_policy::UnsupportedDirectShape::shape.plan_normalizer::FixtureMapped::PlanNormalizerCluster"
if summary["selected_cluster_id"] != expected_cluster:
    raise SystemExit("selected cluster drift")
if summary["cluster_size_as_proof"] != 0:
    raise SystemExit("cluster size must not be proof")

ranked = fixture["ranked_clusters"]
if len(ranked) != 21:
    raise SystemExit("ranked cluster count drift")
if ranked[0]["cluster_id"] != expected_cluster:
    raise SystemExit("rank 1 cluster drift")
if ranked[0]["next_card"] != "MIRBUILDER-PLAN-NORMALIZER-PROJECTION-POLICY-001":
    raise SystemExit("rank 1 next card drift")
excluded = fixture.get("excluded_existing_decision_clusters") or []
if len(excluded) != 21:
    raise SystemExit("excluded existing decision clusters drift")
if excluded[0]["next_card"] != "MIRBUILDER-LOOP-COND-BC-CARRIER-SYNC-PROJECTION-POLICY-001":
    raise SystemExit("expected carrier sync to be excluded first")

decision = fixture["decision"]
if decision["kind"] != "SelectProjectionPolicyCluster":
    raise SystemExit("decision kind drift")
if decision["selected_cluster_id"] != expected_cluster:
    raise SystemExit("decision cluster drift")
if decision["selected_next_card"] != "MIRBUILDER-PLAN-NORMALIZER-PROJECTION-POLICY-001":
    raise SystemExit("decision next card drift")

claims = fixture["claims"]
if claims.get("deterministic_priority_resolution") != 1:
    raise SystemExit("deterministic priority claim missing")
if claims.get("existing_decision_filter_enabled") != 1:
    raise SystemExit("existing decision filter claim missing")
if claims.get("excluded_existing_decision_cluster_count") != 21:
    raise SystemExit("excluded existing decision claim drift")
if claims.get("selectable_cluster_count") != 21:
    raise SystemExit("selectable cluster count claim drift")
if claims.get("cluster_size_tiebreaker_only") != 1:
    raise SystemExit("cluster size tiebreaker claim missing")
for key in [
    "cluster_size_as_proof",
    "manual_family_selection",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_edit_authority",
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
output_contract=rust-lifecycle-mirbuilder-projection-policy-cluster-priority-resolution-v0
eligible_cluster_count=42
selected_next_card=MIRBUILDER-PLAN-NORMALIZER-PROJECTION-POLICY-001
cluster_size_as_proof=0
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT

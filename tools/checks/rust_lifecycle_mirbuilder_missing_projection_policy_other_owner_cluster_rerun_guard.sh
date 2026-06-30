#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-missing-projection-policy-other-owner-cluster-rerun-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_missing_projection_policy_other_owner_cluster_rerun.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-other-owner-cluster-rerun-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1943-MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-CLUSTER-RERUN-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-other-owner-cluster-rerun-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1943-MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-CLUSTER-RERUN-001.md").read_text()

token = "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-CLUSTER-RERUN-001"
if fixture.get("kind") != "MirBuilderMissingProjectionPolicyOtherOwnerClusterRerunV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

state = fixture["input_state"]
if state["input_other_owner_cluster_count"] != 185:
    raise SystemExit("input count drift")
if state["current_blocker"] != "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001":
    raise SystemExit("current blocker drift")

summary = fixture["summary"]
if summary["input_other_owner_cluster_count"] != 185:
    raise SystemExit("summary input count drift")
if summary["subcluster_count"] != 123:
    raise SystemExit("subcluster count drift")
if summary["selection_eligible_subcluster_count"] != 0:
    raise SystemExit("shape-missing rows must not be projection-eligible")
if summary["owner_edge_confidence_counts"]["FileScoped"] != 185:
    raise SystemExit("FileScoped count drift")
if summary["shape_signature_counts"]["unknown_shape"] != 185:
    raise SystemExit("unknown shape count drift")
if summary["distinct_known_owner_edge_count"] != 85:
    raise SystemExit("distinct known owner edge count drift")

if sum(item["candidate_count"] for item in fixture["subclusters"]) != 185:
    raise SystemExit("subclusters must partition all rerun rows")
if any(item["shape_signature"] != "unknown_shape" for item in fixture["subclusters"]):
    raise SystemExit("all rerun subclusters must preserve unknown shape")
if any("MissingShapeSignature" not in item["blocked_by"] for item in fixture["subclusters"]):
    raise SystemExit("every rerun subcluster must be blocked by MissingShapeSignature")

decision = fixture["decision"]
if decision["kind"] != "SelectShapeSignatureInventory":
    raise SystemExit("decision kind drift")
if decision["reason_token"] != "OtherOwnerClusterShapeSignatureMissing":
    raise SystemExit("decision reason drift")
if decision["selected_next_card"] != "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-INVENTORY-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "source_report_consumed",
    "owner_edge_confidence_repair_consumed",
    "all_other_owner_cluster_items_partitioned_exactly_once",
    "owner_edge_confidence_repair_applied",
    "shape_signature_gap_selected",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"required claim must be 1: {key}")
if claims.get("input_other_owner_cluster_count") != 185:
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
output_contract=rust-lifecycle-mirbuilder-missing-projection-policy-other-owner-cluster-rerun-v0
input_other_owner_cluster_count=185
subcluster_count=123
owner_edge_confidence=FileScoped:185
shape_signature=unknown_shape:185
selected_next_card=MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-INVENTORY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT

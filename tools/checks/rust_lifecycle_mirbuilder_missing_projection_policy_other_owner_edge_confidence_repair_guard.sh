#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-missing-projection-policy-other-owner-edge-confidence-repair-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_missing_projection_policy_other_owner_edge_confidence_repair.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-other-owner-edge-confidence-repair-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1942-MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-EDGE-CONFIDENCE-REPAIR-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-other-owner-edge-confidence-repair-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1942-MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-EDGE-CONFIDENCE-REPAIR-001.md").read_text()

token = "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-EDGE-CONFIDENCE-REPAIR-001"
if fixture.get("kind") != "MirBuilderMissingProjectionPolicyOtherOwnerEdgeConfidenceRepairV1":
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

policy = fixture["repair_policy"]
if policy["policy"] != "FileScopedOwnerEdgeFromSourcePath":
    raise SystemExit("repair policy drift")
if policy["semantic_projection_inference"] != 0:
    raise SystemExit("repair must not infer projection semantics")

summary = fixture["summary"]
if summary["input_other_owner_cluster_count"] != 185:
    raise SystemExit("summary input count drift")
if summary["repaired_row_count"] != 185:
    raise SystemExit("repaired row count drift")
if summary["unrepaired_row_count"] != 0:
    raise SystemExit("unrepaired row count must stay zero")
if summary["distinct_repaired_owner_edge_count"] != 85:
    raise SystemExit("distinct owner edge count drift")
if summary["top_repaired_owner_edges"][0] != {
    "owner_edge": "hakorune_mir_builder::joinir_id_remapper",
    "count": 9,
}:
    raise SystemExit("top repaired owner edge drift")

if len(fixture["repaired_rows"]) != 185:
    raise SystemExit("repaired row list count drift")
if fixture["unrepaired_rows"] != []:
    raise SystemExit("unrepaired rows must be empty")
if any(row["repaired_owner_edge_confidence"] != "FileScoped" for row in fixture["repaired_rows"]):
    raise SystemExit("all repaired rows must be FileScoped")
if any(not row["repaired_known_owner_edge"] for row in fixture["repaired_rows"]):
    raise SystemExit("all repaired rows must have owner edge")

decision = fixture["decision"]
if decision["kind"] != "SelectOtherOwnerClusterRerun":
    raise SystemExit("decision kind drift")
if decision["reason_token"] != "OtherOwnerEdgeConfidenceRepairComplete":
    raise SystemExit("decision reason drift")
if decision["selected_next_card"] != "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-CLUSTER-RERUN-001":
    raise SystemExit("selected next card drift")

claims = fixture["claims"]
for key in [
    "source_report_consumed",
    "other_owner_cluster_consumed",
    "all_other_owner_rows_have_repair_attempt",
    "file_scoped_owner_edge_derived_from_source_path",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"required claim must be 1: {key}")
if claims.get("input_other_owner_cluster_count") != 185:
    raise SystemExit("claim input count drift")
for key in [
    "semantic_projection_inference",
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
output_contract=rust-lifecycle-mirbuilder-missing-projection-policy-other-owner-edge-confidence-repair-v0
input_other_owner_cluster_count=185
repaired_row_count=185
unrepaired_row_count=0
distinct_repaired_owner_edge_count=85
selected_next_card=MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-CLUSTER-RERUN-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-missing-projection-policy-post-type-exhaustion-selection-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_missing_projection_policy_post_type_exhaustion_selection_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2094-MIRBUILDER-MISSING-PROJECTION-POLICY-POST-TYPE-EXHAUSTION-SELECTION-RERUN-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[5], encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-MISSING-PROJECTION-POLICY-POST-TYPE-EXHAUSTION-SELECTION-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderMissingProjectionPolicyPostTypeExhaustionSelectionRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("post_type_exhaustion_selection_basis", "").endswith("mirbuilder-missing-projection-policy-post-type-exhaustion-selection-basis-v0.json"), "basis input drift")
need(inputs.get("missing_projection_policy_rerun_005", "").endswith("mirbuilder-missing-projection-policy-cluster-resolution-rerun-005-v0.json"), "rerun 005 input drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("name") == "MissingProjectionPolicyPostTypeExhaustionSelectorV1", "selector drift")
need(rule.get("basis_selects_projection_policy") is False, "basis projection rule drift")
need(rule.get("type_transport_missing_is_parked_not_deleted") is True, "TypeTransport rule drift")
need(rule.get("selection_requires_exactly_one_machine_derived_lane_or_card") is True, "exactly-one rule drift")

lanes = fixture.get("candidate_lanes") or []
expected_lanes = {
    "ResidualOwnerEdgeAndShapeSignatureBlockerInventory",
    "TypeOnlyProjectionPolicySelectorBasis",
    "ProjectionDescriptorOverlayFreshnessRerun",
    "KeepStopped",
}
need({row.get("lane_id") for row in lanes} == expected_lanes, "candidate lane set drift")
need(not [row for row in lanes if row.get("selection_eligible") is True], "no lane may be eligible")

summary = fixture.get("summary") or {}
need(summary.get("candidate_lane_count") == 4, "candidate lane count drift")
need(summary.get("selection_eligible_lane_count") == 0, "eligible lane count drift")
need(summary.get("remaining_blocker_cluster_count") == 5, "remaining cluster drift")
need(summary.get("remaining_blocker_candidate_count") == 185, "remaining row drift")
need(summary.get("type_only_cluster_count") == 73, "type-only cluster drift")
need(summary.get("type_only_candidate_count") == 819, "type-only row drift")
need(summary.get("new_projection_policy_selected") == 0, "projection policy selection drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "NoMachineDerivedMissingProjectionPolicyRerun005Lane", "reason drift")
need(decision.get("selected_lane") is None, "lane must not be selected")
need(decision.get("selected_projection_policy_cluster") is None, "cluster must not be selected")
need(decision.get("selected_next_card") == design_stop, "next card drift")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "new_projection_policy_selected",
    "generated_artifact_as_native_edit_authority",
    "manual_lane_selection",
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "row_count_as_proof",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "family_name_as_proof",
    "route_membership_alone_as_proof",
    "historical_preference_as_proof",
    "self_signed_fixture_as_proof",
    "basis_010_exactly_one_wider_lane_as_projection_policy_proof",
    "type_transport_exhausted_as_projection_policy_proof",
    "type_only_cluster_direct_selection",
    "owner_edge_repair_as_projection_policy_proof",
    "shape_signature_inventory_as_projection_policy_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows = {row.get("token"): row for row in manifest.get("rows") or []}
row = rows.get(token) or {}
need(row.get("card", "").endswith("2094-MIRBUILDER-MISSING-PROJECTION-POLICY-POST-TYPE-EXHAUSTION-SELECTION-RERUN-001.md"), "manifest card drift")
need(row.get("fixture", "").endswith("mirbuilder-missing-projection-policy-post-type-exhaustion-selection-rerun-v0.json"), "manifest fixture drift")
need(row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_missing_projection_policy_post_type_exhaustion_selection_rerun_guard.sh"), "manifest guard drift")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-missing-projection-policy-post-type-exhaustion-selection-rerun")
print("decision=KeepStopped")
print("reason_token=NoMachineDerivedMissingProjectionPolicyRerun005Lane")
print("selection_eligible_lane_count=0")
print("new_projection_policy_selected=0")
PY

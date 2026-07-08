#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-delete-closeout-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_remaining_subsurface_post_delete_closeout_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2130-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-POST-DELETE-CLOSEOUT-RERUN-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-delete-closeout-rerun"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-POST-DELETE-CLOSEOUT-RERUN-001"
next_card = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteRemainingSubsurfacePostDeleteCloseoutRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("delete_closeout_decision") == "SelectWriteRemainingSubsurfacePostDeleteCloseoutRerun", "delete closeout decision drift")
need(inputs.get("delete_closeout_selected_next_card") == token, "delete closeout next drift")
need(inputs.get("accepted_scoped_closeout_count") == 5, "accepted closeout count drift")

rows = fixture.get("remaining_subsurfaces") or []
need(len(rows) == 1, "remaining row count drift")
row = rows[0]
need(row.get("subsurface_id") == "SetSurfacePolicy", "remaining subsurface drift")
need(row.get("routes") == ["MapStoreI64", "MapStoreAny"], "set route drift")
need(row.get("normalized_result_class") == "NoneResult", "set result drift")
need(row.get("publication_class") == "NonePublication", "set publication drift")
need(row.get("mutation_class") == "MutatesReceiverOrContainer", "set mutation drift")
need(row.get("hako_adopted") is False, "set adoption drift")
need(row.get("basis_selection_eligible") is False, "set eligibility drift")
need(row.get("typed_non_typed_split_present") is True, "split drift")
need(row.get("candidate_routes_require_split_consultation") is True, "split consultation drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("if_no_hako_adopted_remaining_subsurface_keep_stopped") is True, "stop rule drift")
need(rule.get("next_pilot_requires_design_consultation") is True, "consultation rule drift")
need(rule.get("set_surface_direct_pilot_selection_allowed") is False, "direct selection rule drift")
need(rule.get("set_surface_split_selection_allowed") is False, "split selection rule drift")
for key in [
    "manual_subsurface_selection",
    "route_count_as_proof",
    "apparent_simplicity_as_proof",
    "accepted_read_contract_similarity_as_proof",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

summary = fixture.get("summary") or {}
need(summary.get("remaining_write_subsurface_count") == 1, "remaining count drift")
need(summary.get("remaining_subsurfaces") == ["SetSurfacePolicy"], "remaining summary drift")
need(summary.get("set_surface_policy_remaining") == 1, "set remaining drift")
need(summary.get("set_split_consultation_required") == 1, "consultation required drift")
for key in [
    "hako_adopted_remaining_write_subsurface_count",
    "basis_selection_eligible_subsurface_count",
    "selected_write_subsurface_count",
    "set_direct_hako_pilot_selected",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "NoConsultationApprovedSetSurfacePilotOrSplitProofAxis", "reason drift")
need(decision.get("recommended_consultation_topic") == "WriteSetSurfacePolicyPilotOrSplitSelection", "topic drift")
need(decision.get("selected_next_card") == next_card, "next drift")
need(decision.get("selected_subsurface") is None, "selected subsurface drift")

claims = fixture.get("claims") or {}
for key in ["write_remaining_subsurface_post_delete_closeout_rerun", "set_surface_policy_remaining"]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
need(claims.get("remaining_write_subsurface_count") == 1, "claim remaining count drift")
for key in [
    "hako_adopted_remaining_write_subsurface_count",
    "basis_selection_eligible_subsurface_count",
    "selected_write_subsurface_count",
    "set_direct_hako_pilot_selected",
    "set_split_unnecessary",
    "write_direct_closeout_materialized",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
    "hako_generation",
    "new_route_authority",
    "behavior_change",
    "runtime_mutation_authority",
    "publication_execution",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "native_seed_materialization",
    "new_python_semantic_projector",
    "manual_axis_selection",
    "manual_carrier_selection",
    "manual_subsurface_selection",
    "row_count_as_proof",
    "route_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
    "apparent_simplicity_as_proof",
    "accepted_read_contract_similarity_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2130-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-POST-DELETE-CLOSEOUT-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-delete-closeout-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_remaining_subsurface_post_delete_closeout_rerun_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need("recommended_consultation_topic=WriteSetSurfacePolicyPilotOrSplitSelection" in task_order, "task order topic drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-delete-closeout-rerun")
print("remaining_write_subsurface_count=1")
print("remaining_subsurfaces=SetSurfacePolicy")
print("set_split_consultation_required=1")
print("decision=KeepStopped")
print("recommended_consultation_topic=WriteSetSurfacePolicyPilotOrSplitSelection")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY

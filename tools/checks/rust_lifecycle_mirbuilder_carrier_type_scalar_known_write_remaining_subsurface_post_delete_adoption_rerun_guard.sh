#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-delete-adoption-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_remaining_subsurface_post_delete_adoption_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2127-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-POST-DELETE-ADOPTION-RERUN-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-delete-adoption-rerun"

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


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-POST-DELETE-ADOPTION-RERUN-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteRemainingSubsurfacePostDeleteAdoptionRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("adoption_decision") == "Adopt", "adoption decision drift")
need(inputs.get("adopted_surface") == "DeleteSurfacePolicy/MapDeleteAny", "adopted surface drift")
need(inputs.get("adopted_owner") == "write_delete_surface_policy_classifier", "adopted owner drift")

rows = {row.get("subsurface_id"): row for row in fixture.get("candidate_subsurfaces") or []}
need(set(rows) == {"PushSurfacePolicy", "DeleteSurfacePolicy", "SetSurfacePolicy"}, "candidate drift")
need(rows["PushSurfacePolicy"].get("hako_adopted") is True, "push adoption drift")
need(rows["PushSurfacePolicy"].get("direct_closeout_materialized") is True, "push closeout drift")
need(rows["PushSurfacePolicy"].get("basis_selection_eligible") is False, "push eligibility drift")
need(rows["DeleteSurfacePolicy"].get("hako_adopted") is True, "delete adoption drift")
need(rows["DeleteSurfacePolicy"].get("direct_closeout_materialized") is False, "delete closeout drift")
need(rows["DeleteSurfacePolicy"].get("basis_selection_eligible") is True, "delete eligibility drift")
need(rows["SetSurfacePolicy"].get("hako_adopted") is False, "set adoption drift")
need(rows["SetSurfacePolicy"].get("basis_selection_eligible") is False, "set eligibility drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("basis_selection_allowed_after_exactly_one_hako_adopted_write_pilot") is True, "selection rule drift")
need(rule.get("already_materialized_scoped_closeouts_not_eligible") is True, "materialized closeout rule drift")
need(rule.get("direct_closeout_materialization_allowed") is False, "direct materialization rule drift")
for key in [
    "manual_subsurface_selection",
    "route_count_as_proof",
    "apparent_simplicity_as_proof",
    "accepted_read_contract_similarity_as_proof",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

summary = fixture.get("summary") or {}
need(summary.get("write_delete_surface_hako_adopted") == 1, "delete adoption summary drift")
need(summary.get("hako_adopted_write_subsurface_count") == 2, "adopted count drift")
need(summary.get("basis_selection_eligible_subsurface_count") == 1, "eligible count drift")
need(summary.get("selected_write_subsurface") == "DeleteSurfacePolicy", "selected subsurface drift")
for key in [
    "write_delete_surface_direct_closeout_materialized",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWriteDeleteSurfaceTypedDirectCloseoutContractBasis", "decision kind drift")
need(decision.get("reason_token") == "ExactlyOneHakoAdoptedWriteSubsurfacePilotNeedsScopedCloseout", "reason drift")
need(decision.get("selected_subsurface") == "DeleteSurfacePolicy", "decision subsurface drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "write_remaining_subsurface_post_delete_adoption_rerun",
    "write_delete_surface_hako_adopted",
    "write_subsurface_selected",
]:
    need(claims.get(key) == 1, f"missing claim: {key}")
need(claims.get("hako_adopted_write_subsurface_count") == 2, "claim adopted count drift")
need(claims.get("basis_selection_eligible_subsurface_count") == 1, "claim eligible count drift")
for key in [
    "write_delete_surface_direct_closeout_materialized",
    "write_direct_closeout_materialized",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "component_specific_direct_contract_materialized",
    "source_selfhost_claim",
    "new_route_authority",
    "behavior_change",
    "runtime_mutation_authority",
    "publication_execution",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "native_seed_materialization",
    "hako_generation",
    "manual_subsurface_selection",
    "route_count_as_proof",
    "apparent_simplicity_as_proof",
    "accepted_read_contract_similarity_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2127-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-POST-DELETE-ADOPTION-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-delete-adoption-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_remaining_subsurface_post_delete_adoption_rerun_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-delete-adoption-rerun")
print("write_delete_surface_hako_adopted=1")
print("hako_adopted_write_subsurface_count=2")
print("basis_selection_eligible_subsurface_count=1")
print("selected_write_subsurface=DeleteSurfacePolicy")
print("write_delete_surface_direct_closeout_materialized=0")
print("scalar_known_transport_axis_closeout=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

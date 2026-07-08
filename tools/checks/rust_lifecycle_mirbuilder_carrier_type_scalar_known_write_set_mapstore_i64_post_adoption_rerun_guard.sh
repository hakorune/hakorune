#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-post-adoption-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_set_mapstore_i64_post_adoption_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2136-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-POST-ADOPTION-RERUN-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-post-adoption-rerun"

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


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-POST-ADOPTION-RERUN-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteSetMapStoreI64PostAdoptionRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("adoption_decision") == "Adopt", "adoption decision drift")
need(inputs.get("adopted_surface") == "SetSurfacePolicy/MapStoreI64", "adopted surface drift")
need(inputs.get("adopted_owner") == "write_set_mapstore_i64_policy_classifier", "adopted owner drift")

rows = {row.get("candidate_id"): row for row in fixture.get("candidate_surfaces") or []}
need(set(rows) == {"PushSurfacePolicy", "DeleteSurfacePolicy", "SetSurfacePolicy/MapStoreI64", "SetSurfacePolicy/MapStoreAny"}, "candidate drift")
need(rows["PushSurfacePolicy"].get("direct_closeout_materialized") is True, "push closeout drift")
need(rows["DeleteSurfacePolicy"].get("hako_adopted") is False, "delete adoption drift")
need(rows["DeleteSurfacePolicy"].get("direct_closeout_materialized") is False, "delete closeout drift")
need(rows["DeleteSurfacePolicy"].get("mirror_retired") is True, "delete mirror retire drift")
need("DeleteSurfaceMirrorRetired" in rows["DeleteSurfacePolicy"].get("blocked_by", []), "delete retire blocker drift")
need(rows["SetSurfacePolicy/MapStoreI64"].get("hako_adopted") is True, "MapStoreI64 adoption drift")
need(rows["SetSurfacePolicy/MapStoreI64"].get("basis_selection_eligible") is True, "MapStoreI64 eligibility drift")
need(rows["SetSurfacePolicy/MapStoreI64"].get("any_write_boundary_opened") is False, "Any boundary opened drift")
need(rows["SetSurfacePolicy/MapStoreAny"].get("hako_adopted") is False, "MapStoreAny adoption drift")
need(rows["SetSurfacePolicy/MapStoreAny"].get("basis_selection_eligible") is False, "MapStoreAny eligibility drift")
need("AnyWriteBoundaryRequired" in rows["SetSurfacePolicy/MapStoreAny"].get("blocked_by", []), "MapStoreAny blocker drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("basis_selection_allowed_after_exactly_one_hako_adopted_unmaterialized_scoped_surface") is True, "selection rule drift")
need(rule.get("already_materialized_scoped_closeouts_not_eligible") is True, "materialized closeout rule drift")
need(rule.get("any_write_boundary_not_eligible") is True, "Any boundary rule drift")
need(rule.get("direct_closeout_materialization_allowed") is False, "direct materialization rule drift")
for key in [
    "manual_subsurface_selection",
    "route_count_as_proof",
    "apparent_simplicity_as_proof",
    "accepted_read_contract_similarity_as_proof",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

summary = fixture.get("summary") or {}
need(summary.get("write_set_mapstore_i64_hako_adopted") == 1, "MapStoreI64 adoption summary drift")
need(summary.get("basis_selection_eligible_surface_count") == 1, "eligible count drift")
need(summary.get("selected_scoped_surface") == "SetSurfacePolicy/MapStoreI64", "selected surface drift")
need(summary.get("mapstore_any_deferred") == 1, "MapStoreAny defer drift")
for key in [
    "any_write_boundary_opened",
    "write_set_mapstore_i64_direct_closeout_materialized",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWriteSetMapStoreI64TypedDirectCloseoutContractBasis", "decision kind drift")
need(decision.get("reason_token") == "ExactlyOneHakoAdoptedSetMapStoreI64PilotNeedsScopedCloseout", "reason drift")
need(decision.get("selected_scoped_surface") == "SetSurfacePolicy/MapStoreI64", "decision surface drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "write_set_mapstore_i64_post_adoption_rerun",
    "write_set_mapstore_i64_hako_adopted",
    "write_scoped_surface_selected",
    "mapstore_any_deferred",
]:
    need(claims.get(key) == 1, f"missing claim: {key}")
need(claims.get("basis_selection_eligible_surface_count") == 1, "claim eligible count drift")
for key in [
    "any_write_boundary_opened",
    "write_set_mapstore_i64_direct_closeout_materialized",
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
need(manifest_row.get("card", "").endswith("2136-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-POST-ADOPTION-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-post-adoption-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_i64_post_adoption_rerun_guard.sh"), "manifest guard drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-post-adoption-rerun")
print("write_set_mapstore_i64_hako_adopted=1")
print("basis_selection_eligible_surface_count=1")
print("selected_scoped_surface=SetSurfacePolicy/MapStoreI64")
print("mapstore_any_deferred=1")
print("any_write_boundary_opened=0")
print("write_set_mapstore_i64_direct_closeout_materialized=0")
print("scalar_known_transport_axis_closeout=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-set-surface-typed-value-split-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_set_surface_typed_value_split_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2131-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-SURFACE-TYPED-VALUE-SPLIT-BASIS-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-surface-typed-value-split-basis"

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


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-SURFACE-TYPED-VALUE-SPLIT-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-RUST-ORACLE-PARITY-FIXTURE-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteSetSurfaceTypedValueSplitBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("post_delete_decision") == "KeepStopped", "post delete decision drift")
need(inputs.get("recommended_consultation_topic") == "WriteSetSurfacePolicyPilotOrSplitSelection", "topic drift")
need(inputs.get("remaining_subsurface") == "SetSurfacePolicy", "remaining subsurface drift")
need(inputs.get("remaining_routes") == ["MapStoreI64", "MapStoreAny"], "remaining routes drift")

axis = fixture.get("proof_axis") or {}
need(axis.get("prior_hako_adopted_write_surface_metadata_coverage") is True, "metadata coverage drift")
need(axis.get("set_surface_typed_value_boundary_split_proof_axis") is True, "split axis drift")
need(axis.get("typed_scalar_write_before_any_write") is True, "typed-first drift")
need(axis.get("already_covered_by_push_delete") == [
    "MutatesReceiverOrContainerMetadata",
    "NonePublicationMetadata",
], "covered metadata drift")
need(axis.get("new_for_set") == [
    "NoneResultMetadata",
    "TypedVsAnyWriteValueBoundary",
], "new set metadata drift")

split = fixture.get("split_plan") or {}
need(split.get("surface") == "SetSurfacePolicy", "split surface drift")
need(split.get("whole_set_hako_pilot_allowed") is False, "whole set allowed drift")
i64 = split.get("mapstore_i64") or {}
need(i64.get("route") == "MapStoreI64", "i64 route drift")
need(i64.get("first_candidate") is True, "i64 first drift")
need(i64.get("typed_scalar_write") is True, "i64 typed drift")
need(i64.get("write_value_boundary") == "ScalarI64", "i64 boundary drift")
need(i64.get("scalar_known_lane_local") is True, "i64 lane drift")
any_row = split.get("mapstore_any") or {}
need(any_row.get("route") == "MapStoreAny", "any route drift")
need(any_row.get("deferred") is True, "any deferred drift")
need(any_row.get("typed_scalar_write") is False, "any typed drift")
need(any_row.get("write_value_boundary") == "Any", "any boundary drift")
need(any_row.get("requires_any_write_boundary") is True, "any boundary requirement drift")

rule = fixture.get("selection_rule") or {}
for key in [
    "basis_only",
    "rerun_or_fixture_required_before_hako_pilot",
]:
    need(rule.get(key) is True, f"missing rule: {key}")
for key in [
    "set_hako_pilot_selection_allowed",
    "mapstore_i64_hako_pilot_selection_allowed",
    "mapstore_any_hako_pilot_selection_allowed",
    "route_count_as_proof",
    "apparent_simplicity_as_proof",
    "manual_subsurface_selection",
    "accepted_read_contract_similarity_as_proof",
    "owner_name_as_proof",
    "source_path_as_authority",
    "route_membership_alone_as_proof",
]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

summary = fixture.get("summary") or {}
for key in [
    "set_surface_typed_value_split_basis",
    "set_surface_policy_remaining",
    "mapstore_i64_first_candidate",
    "mapstore_any_deferred",
    "typed_scalar_write_before_any_write",
    "prior_hako_adopted_write_surface_metadata_coverage",
    "basis_only",
    "rerun_or_fixture_required_before_hako_pilot",
]:
    need(summary.get(key) == 1, f"missing summary claim: {key}")
for key in [
    "set_hako_pilot_selected",
    "mapstore_i64_hako_pilot_selected",
    "mapstore_any_hako_pilot_selected",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectSetMapStoreI64RustOracleParityFixture", "decision kind drift")
need(decision.get("reason_token") == "SetSurfaceTypedValueSplitBasisDefined", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "set_surface_typed_value_split_basis",
    "set_surface_policy_remaining",
    "mapstore_i64_first_candidate",
    "mapstore_any_deferred",
    "typed_scalar_write_before_any_write",
    "prior_hako_adopted_write_surface_metadata_coverage",
    "basis_only",
    "rerun_or_fixture_required_before_hako_pilot",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "set_hako_pilot_selected",
    "mapstore_i64_hako_pilot_selected",
    "mapstore_any_hako_pilot_selected",
    "set_split_unnecessary",
    "write_direct_closeout_materialized",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
    "runtime_mutation_authority",
    "publication_execution",
    "new_route_authority",
    "new_backend_route",
    "new_abi",
    "behavior_change",
    "runtime_fallback",
    "native_seed_materialization",
    "hako_generation",
    "new_python_semantic_projector",
    "route_count_as_proof",
    "apparent_simplicity_as_proof",
    "manual_subsurface_selection",
    "accepted_read_contract_similarity_as_proof",
    "owner_name_as_proof",
    "source_path_as_authority",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2131-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-SURFACE-TYPED-VALUE-SPLIT-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-set-surface-typed-value-split-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_surface_typed_value_split_basis_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-surface-typed-value-split-basis")
print("set_surface_typed_value_split_basis=1")
print("mapstore_i64_first_candidate=1")
print("mapstore_any_deferred=1")
print("basis_only=1")
print("mapstore_i64_hako_pilot_selected=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

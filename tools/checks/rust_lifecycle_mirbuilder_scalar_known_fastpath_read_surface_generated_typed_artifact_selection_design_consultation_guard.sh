#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-read-surface-generated-typed-artifact-selection-design-consultation-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_read_surface_generated_typed_artifact_selection_design_consultation.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3373-MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-GENERATED-TYPED-ARTIFACT-SELECTION-DESIGN-CONSULTATION-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
RERUN_003="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-003-v0.json"
SCALAR_CONTRACT="$ROOT/src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
COLLECTION_READ_ROUTES="$ROOT/src/mir/generic_method_route_plan/collection_read_routes.rs"
STRING_ROUTES="$ROOT/src/mir/generic_method_route_plan/string_routes.rs"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-read-surface-selection-consultation"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" \
  "$RERUN_003" "$SCALAR_CONTRACT" "$COLLECTION_READ_ROUTES" "$STRING_ROUTES"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$RERUN_003" "$SCALAR_CONTRACT" "$COLLECTION_READ_ROUTES" "$STRING_ROUTES" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
rerun_003 = json.load(open(sys.argv[5], encoding="utf-8"))
scalar_contract = Path(sys.argv[6]).read_text(encoding="utf-8")
collection_read_routes = Path(sys.argv[7]).read_text(encoding="utf-8")
string_routes = Path(sys.argv[8]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-GENERATED-TYPED-ARTIFACT-SELECTION-DESIGN-CONSULTATION-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-BASIS-MAPLOAD-SCALAR-I64-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathReadSurfaceGeneratedTypedArtifactSelectionDesignConsultationV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((rerun_003.get("decision") or {}).get("kind") == "KeepStoppedDesignConsultationRequired", "3372 did not stop for consultation")
need((rerun_003.get("decision") or {}).get("reason_token") == "NoMechanicalReadSurfaceGeneratedTypedArtifactPriority", "3372 reason drift")

axis = fixture.get("proof_axis") or {}
need(axis.get("name") == "ReadSurfaceGeneratedArtifactMinimalityAxis", "axis drift")
for key in [
    "artifact_shape_complexity",
    "live_decision_insertion_locality",
    "policy_homogeneity",
    "semantic_authority_non_broadening",
]:
    need(axis.get(key) is True, f"missing allowed axis bit: {key}")
for key in [
    "route_count_as_proof",
    "owner_name_as_proof",
    "source_path_as_authority",
    "route_membership_alone_as_proof",
    "manual_surface_selection",
]:
    need(axis.get(key) is False, f"forbidden proof drift: {key}")

assessments = fixture.get("candidate_assessment") or []
by_surface = {row.get("surface_id"): row for row in assessments}
need(set(by_surface) == {"MapLoadScalarI64Routes", "StringScalarI64Routes", "CollectionScalarI64Routes"}, "candidate set drift")
mapload = by_surface["MapLoadScalarI64Routes"]
need(mapload.get("selected_first") is True, "MapLoad not selected")
need(mapload.get("route_kind_family") == ["MapLoadScalarI64"], "MapLoad route drift")
need(mapload.get("core_op_family") == ["MapGet"], "MapLoad op drift")
need(mapload.get("return_shape") == "ScalarI64OrMissingZero", "MapLoad return drift")
need(mapload.get("value_demand") == "ScalarI64", "MapLoad demand drift")
need(mapload.get("publication_policy") == "NoPublication", "MapLoad publication drift")
need(mapload.get("effect_class") == "read", "MapLoad effect drift")
need(mapload.get("proof_family") == "ScalarI64MapGetStoreFact", "MapLoad proof family drift")
need(len(mapload.get("allowed_existing_proofs") or []) == 3, "MapLoad proof count drift")
need(by_surface["StringScalarI64Routes"].get("selected_first") is False, "String unexpectedly selected")
need(by_surface["CollectionScalarI64Routes"].get("selected_first") is False, "Collection unexpectedly selected")

need("MapLoadScalarI64Routes" in scalar_contract, "contract missing MapLoad surface")
need("GenericMethodRouteKind::MapLoadScalarI64" in collection_read_routes, "collection read owner missing MapLoad route")
need("prove_scalar_i64_map_get_store_fact" in collection_read_routes, "collection read owner missing scalar proof")
need("GenericMethodRouteKind::StringIndexOf" in string_routes, "string owner evidence missing")

summary = fixture.get("summary") or {}
for key in [
    "read_surface_generated_typed_artifact_selection_consultation",
    "read_surface_generated_artifact_minimality_axis",
    "mapload_scalar_i64_routes_selected_first",
    "mapload_generated_artifact_basis_selected",
    "basis_only",
    "implementation_deferred_to_next_card",
]:
    need(summary.get(key) == 1, f"summary missing claim: {key}")
for key in [
    "generated_typed_hako_artifact_created",
    "mapload_fastpath_shadow_consumed",
    "fastpath_connected_closeout",
    "hako_runtime_route_authority",
    "rust_fastpath_rewired",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectMapLoadScalarI64RoutesFirst", "decision kind drift")
need(decision.get("reason_token") == "ReadSurfaceGeneratedArtifactMinimalityAxis", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "read_surface_generated_typed_artifact_selection_consultation",
    "read_surface_generated_artifact_minimality_axis",
    "mapload_scalar_i64_routes_selected_first",
    "mapload_generated_artifact_basis_selected",
    "basis_only",
    "implementation_deferred_to_next_card",
]:
    need(claims.get(key) == 1, f"claim missing: {key}")
for key in [
    "generated_typed_hako_artifact_created",
    "mapload_fastpath_shadow_consumed",
    "read_surface_connection_complete",
    "fastpath_connected_closeout",
    "hako_runtime_route_authority",
    "rust_fastpath_rewired",
    "route_selection_authority_switch",
    "backend_lowering_authority",
    "runtime_mutation_authority",
    "publication_execution",
    "build_rs_hako_compiler_invocation",
    "live_hako_authority",
    "caller_orientation_runtime_path",
    "new_backend_route",
    "new_abi",
    "runtime_fallback",
    "source_selfhost_claim",
    "manual_surface_selection",
    "route_count_as_proof",
    "owner_name_as_proof",
    "source_path_as_authority",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("3373-MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-SURFACE-GENERATED-TYPED-ARTIFACT-SELECTION-DESIGN-CONSULTATION-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-read-surface-generated-typed-artifact-selection-design-consultation-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_read_surface_generated_typed_artifact_selection_design_consultation_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-read-surface-selection-consultation")
print("read_surface_generated_artifact_minimality_axis=1")
print("mapload_scalar_i64_routes_selected_first=1")
print("generated_typed_hako_artifact_created=0")
print("mapload_fastpath_shadow_consumed=0")
print("fastpath_connected_closeout=0")
print("hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

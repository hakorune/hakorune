#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-any-basis"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-any-basis-v0.json"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3367-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-SET-MAPSTORE-ANY-BASIS-001.md"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_hako_generated_typed_artifact_shadow_consume_set_mapstore_any_basis.py"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INVENTORY="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-v0.json"
MAPSTORE_ANY_HAKO="$ROOT/lang/src/compiler/lib/write_set_mapstore_any_policy_classifier.hako"
MAPSTORE_I64_ARTIFACT="$ROOT/src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_hako_policy.rs"
SHADOW_RS="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
WRITE_ROUTES="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TOOL" "$MANIFEST" "$TASK_ORDER" "$INVENTORY" "$MAPSTORE_ANY_HAKO" "$MAPSTORE_I64_ARTIFACT" "$SHADOW_RS" "$WRITE_ROUTES"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$MANIFEST" "$TASK_ORDER" "$INVENTORY" "$MAPSTORE_ANY_HAKO" "$MAPSTORE_I64_ARTIFACT" "$SHADOW_RS" "$WRITE_ROUTES" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
inventory = json.loads(Path(sys.argv[5]).read_text(encoding="utf-8"))
mapstore_any_hako = Path(sys.argv[6]).read_text(encoding="utf-8")
mapstore_i64_artifact = Path(sys.argv[7]).read_text(encoding="utf-8")
shadow_rs = Path(sys.argv[8]).read_text(encoding="utf-8")
write_routes = Path(sys.argv[9]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-SET-MAPSTORE-ANY-BASIS-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-SET-MAPSTORE-ANY-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathHakoGeneratedTypedArtifactShadowConsumeSetMapstoreAnyBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(next_card in card, "card missing next")

input_state = fixture.get("input_state") or {}
need(input_state.get("inventory_decision") == "SelectMapStoreAnyGeneratedTypedArtifactShadowConsumeBasis", "inventory decision drift")
need(input_state.get("inventory_selected_next_card") == token, "inventory next drift")

basis = fixture.get("basis") or {}
need(basis.get("surface") == "WriteScalarI64Routes/SetSurfacePolicy/MapStoreAny", "basis surface drift")
need(basis.get("route_kind") == "MapStoreAny", "basis route drift")
need(basis.get("prior_connected_surface") == "WriteScalarI64Routes/SetSurfacePolicy/MapStoreI64", "prior surface drift")
need(basis.get("proof_axis") == "PriorGeneratedTypedArtifactSameSetSurfacePolicyMinimalDeltaV1", "proof axis drift")
need(basis.get("next_mechanism") == "CheckedInGeneratedTypedHakoArtifactShadowConsume", "next mechanism drift")
need(basis.get("runtime_authority") == "RustRetained", "runtime authority drift")
need(basis.get("generated_artifact_allowed_next") is True, "artifact next not allowed")
need(basis.get("fastpath_connection_allowed_next") is True, "fastpath next not allowed")
need(basis.get("runtime_hako_source_text_parsing_allowed") is False, "runtime source text parsing allowed")
need(basis.get("build_rs_hako_compiler_invocation_allowed") is False, "build.rs invocation allowed")

shape = fixture.get("artifact_shape") or {}
expected_shape = {
    "row_id": "map_store_any_set_surface",
    "surface": "SetSurfacePolicy/MapStoreAny",
    "route_kind": "MapStoreAny",
    "core_op": "MapSet",
    "lowering_tier": "ColdFallback",
    "result_class": "NoneResult",
    "return_shape": "None",
    "value_demand": "WriteAny",
    "value_boundary": "Any",
    "publication_policy": "NonePublication",
    "effect_class": "mutate",
    "mutation_class": "MutatesReceiverOrContainer",
    "any_boundary_policy": "DeclaredMetadataOnly",
    "role": "classifier_policy_mirror_only",
}
need(shape == expected_shape, "artifact shape drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectMapStoreAnyGeneratedTypedArtifactShadowConsumeImplementation", "decision drift")
need(decision.get("selected_next_card") == next_card, "selected next drift")

claims = fixture.get("claims") or {}
for key in [
    "mapstore_any_generated_typed_artifact_shadow_consume_basis",
    "checked_in_generated_typed_artifact_allowed_next",
    "fastpath_shadow_consume_allowed_next",
    "same_set_surface_policy_minimal_delta",
    "basis_only",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "generated_typed_hako_artifact_shadow_consumed",
    "checked_in_generated_typed_artifact",
    "fastpath_connected_closeout",
    "runtime_hako_source_text_parsing",
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
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need((inventory.get("decision") or {}).get("selected_next_card") == token, "input inventory does not select this card")
need("map_store_any_set_surface|SetSurfacePolicy/MapStoreAny|MapStoreAny|MapSet|ColdFallback|NoneResult|None|WriteAny|Any|NonePublication|mutate|MutatesReceiverOrContainer|DeclaredMetadataOnly|classifier_policy_mirror_only" in mapstore_any_hako, "MapStoreAny hako row missing")
need("WRITE_SET_MAPSTORE_I64_HAKO_POLICY" in mapstore_i64_artifact, "prior typed artifact missing")
need("mapstore_i64_shadow_consumed_decision" in shadow_rs, "prior shadow consumer missing")
need("GenericMethodRouteKind::MapStoreAny" in write_routes, "MapStoreAny fastpath route missing")
need("GenericMethodRouteKind::MapStoreI64" in write_routes, "MapStoreI64 fastpath route missing")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
row = rows_by_token.get(token) or {}
need(row.get("card", "").endswith("3367-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-SET-MAPSTORE-ANY-BASIS-001.md"), "manifest card drift")
need(row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-any-basis-v0.json"), "manifest fixture drift")
need(row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_generated_typed_artifact_shadow_consume_set_mapstore_any_basis_guard.sh"), "manifest guard drift")

for needle in [token, next_card, "basis_only = 1", "hako_runtime_route_authority = 0", "source_selfhost_claim = 0"]:
    need(needle in card, f"card missing: {needle}")
for needle in [token, f"selected_next_card={next_card}"]:
    need(needle in task_order, f"task order missing: {needle}")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-any-basis-v0")
print("mapstore_any_generated_typed_artifact_shadow_consume_basis=1")
print("checked_in_generated_typed_artifact_allowed_next=1")
print("fastpath_shadow_consume_allowed_next=1")
print("generated_typed_hako_artifact_shadow_consumed=0")
print("hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

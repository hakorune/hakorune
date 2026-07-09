#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-basis-mapload-scalar-i64-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_hako_generated_typed_artifact_basis_mapload_scalar_i64.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3374-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-BASIS-MAPLOAD-SCALAR-I64-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
SELECTION="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-read-surface-generated-typed-artifact-selection-design-consultation-v0.json"
SCALAR_CONTRACT="$ROOT/src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
COLLECTION_READ_ROUTES="$ROOT/src/mir/generic_method_route_plan/collection_read_routes.rs"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-basis-mapload-scalar-i64"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" \
  "$SELECTION" "$SCALAR_CONTRACT" "$COLLECTION_READ_ROUTES"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$SELECTION" "$SCALAR_CONTRACT" "$COLLECTION_READ_ROUTES" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
selection = json.load(open(sys.argv[5], encoding="utf-8"))
scalar_contract = Path(sys.argv[6]).read_text(encoding="utf-8")
collection_read_routes = Path(sys.argv[7]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-BASIS-MAPLOAD-SCALAR-I64-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-MAPLOAD-SCALAR-I64-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathHakoGeneratedTypedArtifactBasisMaploadScalarI64V1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((selection.get("decision") or {}).get("kind") == "SelectMapLoadScalarI64RoutesFirst", "selection decision drift")
need((selection.get("decision") or {}).get("selected_next_card") == token, "selection next drift")

basis = fixture.get("basis") or {}
expected = {
    "surface": "MapLoadScalarI64Routes",
    "route_kind": "MapLoadScalarI64",
    "core_op": "MapGet",
    "lowering_tier": "WarmDirectAbi",
    "return_shape": "ScalarI64OrMissingZero",
    "value_demand": "ScalarI64",
    "publication_policy": "NoPublication",
    "effect_class": "read",
    "proof_family": "ScalarI64MapGetStoreFact",
    "next_mechanism": "CheckedInGeneratedTypedHakoArtifactShadowConsume",
    "runtime_authority": "RustRetained",
}
for key, value in expected.items():
    need(basis.get(key) == value, f"basis drift: {key}")
need(len(basis.get("allowed_existing_proofs") or []) == 3, "proof count drift")
for proof in [
    "MapSetScalarI64SameKeyNoEscape",
    "MapSetScalarI64DominatesNoEscape",
    "MapSetScalarI64CoveredDynamicI64KeyNoEscape",
]:
    need(proof in (basis.get("allowed_existing_proofs") or []), f"missing proof: {proof}")
for key in [
    "generated_artifact_allowed_next",
    "fastpath_connection_allowed_next",
]:
    need(basis.get(key) is True, f"basis allowed flag drift: {key}")
for key in [
    "runtime_hako_source_text_parsing_allowed",
    "build_rs_hako_compiler_invocation_allowed",
]:
    need(basis.get(key) is False, f"basis forbidden flag drift: {key}")

need("ScalarKnownContractId::MapLoadScalarI64" in scalar_contract, "contract missing MapLoad contract id")
need("ScalarKnownSurfaceId::MapLoadScalarI64Routes" in scalar_contract, "contract missing MapLoad surface id")
need("GenericMethodRouteKind::MapLoadScalarI64" in scalar_contract, "contract missing MapLoad route")
need("CoreMethodOp::MapGet" in scalar_contract, "contract missing MapGet op")
need("GenericMethodReturnShape::ScalarI64OrMissingZero" in scalar_contract, "contract missing return shape")

for needle in [
    "prove_scalar_i64_map_get_store_fact",
    "GenericMethodRouteKind::MapLoadScalarI64",
    "CoreMethodOp::MapGet",
    "CoreMethodLoweringTier::WarmDirectAbi",
    "GenericMethodReturnShape::ScalarI64OrMissingZero",
    "GenericMethodValueDemand::ScalarI64",
    "GenericMethodPublicationPolicy::NoPublication",
]:
    need(needle in collection_read_routes, f"collection_read_routes missing {needle}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectMapLoadScalarI64GeneratedTypedArtifactShadowConsumeImplementation", "decision kind drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "mapload_scalar_i64_generated_typed_artifact_basis",
    "checked_in_generated_typed_artifact_allowed_next",
    "fastpath_shadow_consume_allowed_next",
    "basis_only",
]:
    need(claims.get(key) == 1, f"claim missing: {key}")
for key in [
    "generated_typed_hako_artifact_created",
    "generated_typed_hako_artifact_shadow_consumed",
    "mapload_fastpath_shadow_consumed",
    "read_surface_connection_complete",
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

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("3374-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-BASIS-MAPLOAD-SCALAR-I64-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-basis-mapload-scalar-i64-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_generated_typed_artifact_basis_mapload_scalar_i64_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-basis-mapload-scalar-i64")
print("mapload_scalar_i64_generated_typed_artifact_basis=1")
print("checked_in_generated_typed_artifact_allowed_next=1")
print("fastpath_shadow_consume_allowed_next=1")
print("generated_typed_hako_artifact_created=0")
print("mapload_fastpath_shadow_consumed=0")
print("hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

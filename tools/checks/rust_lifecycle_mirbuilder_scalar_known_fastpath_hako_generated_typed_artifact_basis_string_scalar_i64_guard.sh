#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-basis-string-scalar-i64-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_hako_generated_typed_artifact_basis_string_scalar_i64.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3377-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-BASIS-STRING-SCALAR-I64-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
RERUN_004="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-004-v0.json"
SCALAR_CONTRACT="$ROOT/src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
STRING_ROUTES="$ROOT/src/mir/generic_method_route_plan/string_routes.rs"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-basis-string-scalar-i64"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" \
  "$RERUN_004" "$SCALAR_CONTRACT" "$STRING_ROUTES"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$RERUN_004" "$SCALAR_CONTRACT" "$STRING_ROUTES" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
rerun = json.load(open(sys.argv[5], encoding="utf-8"))
scalar_contract = Path(sys.argv[6]).read_text(encoding="utf-8")
string_routes = Path(sys.argv[7]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-BASIS-STRING-SCALAR-I64-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-STRING-SCALAR-I64-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathHakoGeneratedTypedArtifactBasisStringScalarI64V1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need((rerun.get("decision") or {}).get("kind") == "SelectStringScalarI64GeneratedTypedArtifactBasis", "rerun decision drift")
need((rerun.get("decision") or {}).get("selected_next_card") == token, "rerun next drift")

basis = fixture.get("basis") or {}
expected = {
    "surface": "StringScalarI64Routes",
    "lowering_tier": "WarmDirectAbi",
    "return_shape": "ScalarI64",
    "value_demand": "ScalarI64",
    "publication_policy": "NoPublication",
    "effect_class": "read",
    "next_mechanism": "CheckedInGeneratedTypedHakoArtifactShadowConsume",
    "runtime_authority": "RustRetained",
}
for key, value in expected.items():
    need(basis.get(key) == value, f"basis drift: {key}")
need(basis.get("route_kind_family") == ["StringIndexOf", "StringLastIndexOf", "StringContains"], "route family drift")
need(basis.get("core_ops") == ["StringIndexOf", "StringLastIndexOf", "StringContains"], "core op drift")
need(basis.get("proof_or_policy_sources") == [
    "IndexOfSurfacePolicy",
    "LastIndexOfSurfacePolicy",
    "ContainsSurfacePolicy",
], "proof family drift")
for key in ["generated_artifact_allowed_next", "fastpath_connection_allowed_next"]:
    need(basis.get(key) is True, f"basis allowed flag drift: {key}")
for key in ["runtime_hako_source_text_parsing_allowed", "build_rs_hako_compiler_invocation_allowed"]:
    need(basis.get(key) is False, f"basis forbidden flag drift: {key}")

for needle in [
    "ScalarKnownContractId::StringSearchScalarI64",
    "ScalarKnownSurfaceId::StringScalarI64Routes",
    "GenericMethodRouteKind::StringIndexOf",
    "GenericMethodRouteKind::StringLastIndexOf",
    "GenericMethodRouteKind::StringContains",
    "CoreMethodOp::StringIndexOf",
    "CoreMethodOp::StringLastIndexOf",
    "CoreMethodOp::StringContains",
    "GenericMethodReturnShape::ScalarI64",
    "GenericMethodPublicationPolicy::NoPublication",
]:
    need(needle in scalar_contract, f"contract missing {needle}")

for needle in [
    "GenericMethodRouteKind::StringIndexOf",
    "GenericMethodRouteKind::StringLastIndexOf",
    "GenericMethodRouteKind::StringContains",
    "GenericMethodRouteProof::IndexOfSurfacePolicy",
    "GenericMethodRouteProof::LastIndexOfSurfacePolicy",
    "GenericMethodRouteProof::ContainsSurfacePolicy",
    "CoreMethodLoweringTier::WarmDirectAbi",
    "GenericMethodReturnShape::ScalarI64",
    "GenericMethodValueDemand::ScalarI64",
    "GenericMethodPublicationPolicy::NoPublication",
]:
    need(needle in string_routes, f"string_routes missing {needle}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectStringScalarI64GeneratedTypedArtifactShadowConsumeImplementation", "decision kind drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "string_scalar_i64_generated_typed_artifact_basis",
    "checked_in_generated_typed_artifact_allowed_next",
    "fastpath_shadow_consume_allowed_next",
    "basis_only",
]:
    need(claims.get(key) == 1, f"claim missing: {key}")
for key in [
    "generated_typed_hako_artifact_created",
    "generated_typed_hako_artifact_shadow_consumed",
    "string_fastpath_shadow_consumed",
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
need(manifest_row.get("card", "").endswith("3377-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-BASIS-STRING-SCALAR-I64-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-basis-string-scalar-i64-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_generated_typed_artifact_basis_string_scalar_i64_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-basis-string-scalar-i64")
print("string_scalar_i64_generated_typed_artifact_basis=1")
print("checked_in_generated_typed_artifact_allowed_next=1")
print("fastpath_shadow_consume_allowed_next=1")
print("generated_typed_hako_artifact_created=0")
print("string_fastpath_shadow_consumed=0")
print("hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

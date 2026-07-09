#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-mapload-hako-authority-pilot-basis"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-mapload-hako-authority-pilot-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_mapload_hako_authority_pilot_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3388-MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-BASIS-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
MAPLOAD_ARTIFACT="$ROOT/src/mir/generic_method_route_plan/generated/mapload_scalar_i64_hako_policy.rs"
SHADOW_SOURCE="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
COLLECTION_READ_ROUTES="$ROOT/src/mir/generic_method_route_plan/collection_read_routes.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" \
  "$MAPLOAD_ARTIFACT" "$SHADOW_SOURCE" "$COLLECTION_READ_ROUTES"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$MAPLOAD_ARTIFACT" "$SHADOW_SOURCE" "$COLLECTION_READ_ROUTES" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[4]).read_text(encoding="utf-8"))
artifact = Path(sys.argv[5]).read_text(encoding="utf-8")
shadow_source = Path(sys.argv[6]).read_text(encoding="utf-8")
collection_read_routes = Path(sys.argv[7]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-BASIS-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathMaploadHakoAuthorityPilotBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("consultation_selected_next_card") == token, "consultation next drift")
need(inputs.get("mapload_hako_route_authority_pilot_basis_selected") == 1, "consultation selection drift")

basis = fixture.get("basis") or {}
need(basis.get("basis_only") is True, "basis-only drift")
need(basis.get("surface") == "MapLoadScalarI64Routes", "surface drift")
need(basis.get("route_kind") == "MapLoadScalarI64", "route kind drift")
need(basis.get("hako_authority_source") == "MAPLOAD_SCALAR_I64_HAKO_POLICY", "authority source drift")
need(basis.get("implementation_deferred") is True, "implementation deferral drift")
need(basis.get("selected_next_card") == next_card, "basis next drift")
fields = basis.get("hako_authority_result_fields") or []
for field in [
    "route_kind",
    "core_op",
    "lowering_tier",
    "return_shape",
    "value_demand",
    "publication_policy",
    "effect_class",
    "proof_family",
    "allowed_proofs",
    "role",
]:
    need(field in fields, f"authority field missing: {field}")
oracle = basis.get("rust_oracle_compat_checker_contract") or {}
need(oracle.get("retained") is True, "rust oracle retained drift")
need(oracle.get("rust_computes_existing_mapload_decision") is True, "rust computes drift")
need(oracle.get("rust_compares_against_hako_authority_result") is True, "rust compares drift")
need(oracle.get("mismatch_policy") == "FailFast", "mismatch policy drift")

shape = fixture.get("mapload_shape") or {}
for key, value in {
    "core_op": "MapGet",
    "lowering_tier": "WarmDirectAbi",
    "return_shape": "ScalarI64OrMissingZero",
    "value_demand": "ScalarI64",
    "publication_policy": "NoPublication",
    "effect_class": "read",
    "proof_family": "ScalarI64MapGetStoreFact",
}.items():
    need(shape.get(key) == value, f"shape drift: {key}")
need(shape.get("allowed_proof_count") == 3, "allowed proof count drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectMapLoadRouteDecisionAuthorityPilotImplementation", "decision kind drift")
need(decision.get("selected_next_card") == next_card, "decision next drift")

for needle in [
    "MAPLOAD_SCALAR_I64_HAKO_POLICY",
    "route_kind: GenericMethodRouteKind::MapLoadScalarI64",
    "core_op: CoreMethodOp::MapGet",
    "return_shape: GenericMethodReturnShape::ScalarI64OrMissingZero",
    "publication_policy: GenericMethodPublicationPolicy::NoPublication",
    'proof_family: "ScalarI64MapGetStoreFact"',
]:
    need(needle in artifact, f"artifact missing: {needle}")
need("mapload_scalar_i64_shadow_consumed_decision" in shadow_source, "shadow consumer missing MapLoad")
need("assert_hako_mapload_scalar_i64_policy_matches_rust" in shadow_source, "MapLoad mismatch helper missing")
need("mapload_scalar_i64_shadow_consumed_decision" in collection_read_routes, "live MapLoad call missing")

summary = fixture.get("summary") or {}
for key in [
    "mapload_hako_authority_pilot_basis",
    "mapload_authority_scope_defined",
    "hako_artifact_result_authority_source_defined",
    "rust_oracle_compat_checker_contract_defined",
    "mismatch_fail_fast_contract_defined",
    "basis_only",
]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
for key in [
    "mapload_hako_route_decision_authority_pilot",
    "scalar_known_hako_runtime_route_authority",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "mapload_hako_authority_pilot_basis",
    "mapload_authority_scope_defined",
    "hako_artifact_result_authority_source_defined",
    "rust_oracle_compat_checker_contract_defined",
    "mismatch_fail_fast_contract_defined",
    "basis_only",
]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
    "mapload_hako_route_decision_authority_pilot",
    "scalar_known_hako_runtime_route_authority",
    "scalar_known_transport_axis_authority_switch",
    "rust_fastpath_rewired",
    "route_selection_authority_switch",
    "backend_lowering_authority",
    "runtime_mutation_authority",
    "publication_execution",
    "caller_orientation_runtime_path",
    "build_rs_hako_compiler_invocation",
    "live_hako_authority",
    "source_selfhost_claim",
    "new_backend_route",
    "new_abi",
    "runtime_fallback",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("3388-MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-mapload-hako-authority-pilot-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_mapload_hako_authority_pilot_basis_guard.sh"), "manifest guard drift")
need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next missing")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-mapload-hako-authority-pilot-basis")
print("mapload_hako_authority_pilot_basis=1")
print("mapload_authority_scope_defined=1")
print("hako_artifact_result_authority_source_defined=1")
print("rust_oracle_compat_checker_contract_defined=1")
print("mismatch_fail_fast_contract_defined=1")
print("basis_only=1")
print("mapload_hako_route_decision_authority_pilot=0")
print("scalar_known_hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

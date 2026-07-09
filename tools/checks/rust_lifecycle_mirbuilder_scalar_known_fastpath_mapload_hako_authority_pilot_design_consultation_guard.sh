#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-mapload-hako-authority-pilot-design-consultation"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-mapload-hako-authority-pilot-design-consultation-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_mapload_hako_authority_pilot_design_consultation.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3387-MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-DESIGN-CONSULTATION-001.md"
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


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-DESIGN-CONSULTATION-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-BASIS-001"

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderScalarKnownFastpathMaploadHakoAuthorityPilotDesignConsultationV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("hardening_selected_next_card") == token, "hardening selected next drift")
need(inputs.get("all_scalar_known_shadow_mismatch_gate_current") == 1, "hardening mismatch gate drift")
need(inputs.get("rust_authority_retained") == 1, "hardening rust authority drift")
need(inputs.get("hako_runtime_route_authority") == 0, "hardening hako authority drift")

consultation = fixture.get("consultation_decision") or {}
need(consultation.get("decision") == "SelectMapLoadHakoAuthorityPilotBasis", "consultation decision drift")
need(consultation.get("basis_only") is True, "basis-only drift")
need(consultation.get("selected_surface") == "MapLoadScalarI64Routes", "surface drift")
need(consultation.get("selected_route_kind") == "MapLoadScalarI64", "route kind drift")
need(consultation.get("rust_role") == "OracleCompatCheckerRetained", "rust role drift")
need(consultation.get("mismatch_policy") == "FailFastRequired", "mismatch policy drift")
need(consultation.get("authority_switch_implementation_deferred") is True, "implementation deferral drift")
need(consultation.get("selected_next_card") == next_card, "consultation next drift")

scope = fixture.get("mapload_scope") or {}
for key, value in {
    "core_op": "MapGet",
    "lowering_tier": "WarmDirectAbi",
    "return_shape": "ScalarI64OrMissingZero",
    "value_demand": "ScalarI64",
    "publication_policy": "NoPublication",
    "effect_class": "read",
    "proof_family": "ScalarI64MapGetStoreFact",
}.items():
    need(scope.get(key) == value, f"mapload scope drift: {key}")
need(scope.get("allowed_proof_count") == 3, "allowed proof count drift")
for key in ["string_surface_deferred", "collection_surface_deferred", "write_surface_deferred"]:
    need(scope.get(key) is True, f"deferred surface drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectMapLoadHakoAuthorityPilotBasis", "decision kind drift")
need(decision.get("reason_token") == "MapLoadIsSmallestReadNoPublicationAuthorityPilotSurface", "reason drift")
need(decision.get("selected_next_card") == next_card, "decision next drift")

for needle in [
    "MAPLOAD_SCALAR_I64_HAKO_POLICY",
    "route_kind: GenericMethodRouteKind::MapLoadScalarI64",
    "core_op: CoreMethodOp::MapGet",
    "lowering_tier: CoreMethodLoweringTier::WarmDirectAbi",
    "return_shape: GenericMethodReturnShape::ScalarI64OrMissingZero",
    "value_demand: GenericMethodValueDemand::ScalarI64",
    "publication_policy: GenericMethodPublicationPolicy::NoPublication",
    'effect_class: "read"',
    'proof_family: "ScalarI64MapGetStoreFact"',
]:
    need(needle in artifact, f"artifact missing: {needle}")
need("mapload_scalar_i64_shadow_consumed_decision" in shadow_source, "shadow consumer missing MapLoad")
need("assert_hako_mapload_scalar_i64_policy_matches_rust" in shadow_source, "shadow mismatch helper missing")
need("mapload_scalar_i64_shadow_consumed_decision" in collection_read_routes, "live route missing MapLoad shadow call")

summary = fixture.get("summary") or {}
for key in [
    "mapload_hako_route_authority_pilot_basis",
    "selected_surface_mapload_scalar_i64_routes",
    "hako_generated_typed_artifact_authority_candidate",
    "rust_oracle_compat_checker_retained",
    "mismatch_fail_fast_required",
    "basis_only",
    "authority_switch_implementation_deferred",
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
    "mapload_hako_route_authority_pilot_basis",
    "selected_surface_mapload_scalar_i64_routes",
    "selected_route_kind_mapload_scalar_i64",
    "hako_generated_typed_artifact_authority_candidate",
    "rust_oracle_compat_checker_retained",
    "mismatch_fail_fast_required",
    "basis_only",
    "authority_switch_implementation_deferred",
]:
    need(claims.get(key) == 1, f"claim positive drift: {key}")
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
    "hako_generation",
    "new_route_authority",
    "behavior_change",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "native_seed_materialization",
    "new_python_semantic_projector",
    "manual_surface_selection",
    "row_count_as_proof",
    "route_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("3387-MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-DESIGN-CONSULTATION-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-mapload-hako-authority-pilot-design-consultation-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_mapload_hako_authority_pilot_design_consultation_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order missing next")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-mapload-hako-authority-pilot-design-consultation")
print("decision=SelectMapLoadHakoAuthorityPilotBasis")
print("selected_surface=MapLoadScalarI64Routes")
print("selected_route_kind=MapLoadScalarI64")
print("mapload_hako_route_authority_pilot_basis=1")
print("hako_generated_typed_artifact_authority_candidate=1")
print("rust_oracle_compat_checker_retained=1")
print("mismatch_fail_fast_required=1")
print("basis_only=1")
print("mapload_hako_route_decision_authority_pilot=0")
print("scalar_known_hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-mapload-hako-route-decision-authority-pilot"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-mapload-hako-route-decision-authority-pilot-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_mapload_hako_route_decision_authority_pilot.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3389-MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
MAPLOAD_ARTIFACT="$ROOT/src/mir/generic_method_route_plan/generated/mapload_scalar_i64_hako_policy.rs"
SHADOW_SOURCE="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
COLLECTION_READ_ROUTES="$ROOT/src/mir/generic_method_route_plan/collection_read_routes.rs"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" \
  "$MAPLOAD_ARTIFACT" "$SHADOW_SOURCE" "$COLLECTION_READ_ROUTES"

python3 "$TOOL" --check
cargo test -q scalar_known_hako_shadow

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


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-RERUN-001"
authority_fn = "mapload_scalar_i64_hako_route_authority_pilot_decision"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathMaploadHakoRouteDecisionAuthorityPilotV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("basis_selected_next_card") == token, "basis next drift")
need(inputs.get("basis_defined") == 1, "basis missing")

implementation = fixture.get("implementation") or {}
need(implementation.get("authority_function") == authority_fn, "authority function drift")
need(implementation.get("legacy_shadow_wrapper_retained") == "mapload_scalar_i64_shadow_consumed_decision", "legacy wrapper drift")
need(implementation.get("live_route_calls_authority_function") is True, "live route call drift")
need(implementation.get("hako_decision_constructed_from") == "MAPLOAD_SCALAR_I64_HAKO_POLICY", "authority source drift")
need(implementation.get("rust_oracle_compat_checker") is True, "oracle drift")
need(implementation.get("mismatch_policy") == "FailFast", "mismatch drift")
need(implementation.get("runtime_source_text_parsing") is False, "source text parser drift")
need(implementation.get("authority_scope") == "MapLoadOnly", "scope drift")

for needle in [
    "MAPLOAD_SCALAR_I64_HAKO_POLICY",
    "route_kind: GenericMethodRouteKind::MapLoadScalarI64",
    "core_op: CoreMethodOp::MapGet",
    "return_shape: GenericMethodReturnShape::ScalarI64OrMissingZero",
    "publication_policy: GenericMethodPublicationPolicy::NoPublication",
]:
    need(needle in artifact, f"artifact missing: {needle}")

for needle in [
    f"pub(super) fn {authority_fn}",
    "let hako_decision = GenericMethodRouteDecision::new",
    "policy.route_kind",
    "policy.core_op",
    "policy.lowering_tier",
    "policy.return_shape",
    "policy.value_demand",
    "policy.publication_policy",
    "let rust_oracle = GenericMethodRouteDecision::new",
    "MapLoad .hako authority pilot diverged from Rust oracle",
    "mapload_scalar_i64_shadow_consumed_decision",
]:
    need(needle in shadow_source, f"shadow source missing: {needle}")
need("include_str!" not in shadow_source, "runtime source text parser present")
need("split('|')" not in shadow_source, "runtime split parser present")
need(authority_fn in collection_read_routes, "live collection route does not call authority pilot")

summary = fixture.get("summary") or {}
for key in [
    "mapload_hako_route_decision_authority_pilot",
    "mapload_hako_authority_result_consumed",
    "mapload_rust_oracle_compat_checker",
    "mapload_mismatch_fail_fast",
    "mapload_live_route_calls_authority_pilot",
]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
for key in ["scalar_known_hako_runtime_route_authority", "source_selfhost_claim"]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "mapload_hako_route_decision_authority_pilot",
    "mapload_hako_authority_result_consumed",
    "mapload_rust_oracle_compat_checker",
    "mapload_mismatch_fail_fast",
    "mapload_live_route_calls_authority_pilot",
]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
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
need(manifest_row.get("card", "").endswith("3389-MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-mapload-hako-route-decision-authority-pilot-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_mapload_hako_route_decision_authority_pilot_guard.sh"), "manifest guard drift")
need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next missing")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-mapload-hako-route-decision-authority-pilot")
print("mapload_hako_route_decision_authority_pilot=1")
print("mapload_hako_authority_result_consumed=1")
print("mapload_rust_oracle_compat_checker=1")
print("mapload_mismatch_fail_fast=1")
print("scalar_known_hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

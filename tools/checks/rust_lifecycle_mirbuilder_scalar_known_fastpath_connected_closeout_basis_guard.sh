#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-connected-closeout-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_connected_closeout_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3365-MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-BASIS-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
WRITE_ROUTES="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"
SHADOW_SOURCE="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
GENERATED_POLICY="$ROOT/src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_hako_policy.rs"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-connected-closeout-basis"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" "$WRITE_ROUTES" "$SHADOW_SOURCE" "$GENERATED_POLICY"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$WRITE_ROUTES" "$SHADOW_SOURCE" "$GENERATED_POLICY" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
write_routes = Path(sys.argv[5]).read_text(encoding="utf-8")
shadow_source = Path(sys.argv[6]).read_text(encoding="utf-8")
generated_policy = Path(sys.argv[7]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-BASIS-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathConnectedCloseoutBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("transport_selected_next_card") == token, "transport next drift")
need(inputs.get("bridge_plan_selected_path") == "C_SHADOW_TYPED_ARTIFACT_FIRST_THEN_HAKO_CALLER_ORIENTATION", "bridge path drift")
for key in ["transport_closeout_hash", "typed_shadow_consume_hash", "bridge_plan_hash"]:
    need(inputs.get(key), f"missing input hash: {key}")

basis = fixture.get("basis") or {}
need(basis.get("basis_only") is True, "basis-only drift")
need(basis.get("required_connection_kind") == "CheckedInGeneratedTypedHakoArtifactShadowConsumedAtRustFastpathDecisionPoint", "connection kind drift")
need(basis.get("rust_authority_retained") is True, "rust authority drift")
need(basis.get("hako_runtime_route_authority") is False, "hako authority drift")
need(basis.get("runtime_source_text_parsing_allowed") is False, "source parsing drift")
need(basis.get("rerun_required_before_connected_closeout") is True, "rerun rule drift")

connected = basis.get("connected_surface_rows") or []
need(len(connected) == 1, "connected row count drift")
row = connected[0]
need(row.get("surface_id") == "WriteScalarI64Routes", "connected surface drift")
need(row.get("subsurface_id") == "SetSurfacePolicy/MapStoreI64", "connected subsurface drift")
need(row.get("route_kind") == "MapStoreI64", "connected route drift")
need(row.get("connected") is True, "connected row drift")
unconnected = basis.get("known_unconnected_surface_rows") or []
need(len(unconnected) == 5, "unconnected row count drift")

need("scalar_known_hako_shadow::mapstore_i64_shadow_consumed_decision()" in write_routes, "write route does not consume shadow decision")
need("WRITE_SET_MAPSTORE_I64_HAKO_POLICY" in shadow_source, "shadow source missing generated policy")
need("include_str!" not in shadow_source, "runtime source text parsing still present")
need("split('|')" not in shadow_source, "runtime split parser still present")
need("pub(crate) const WRITE_SET_MAPSTORE_I64_HAKO_POLICY" in generated_policy, "generated policy const missing")

summary = fixture.get("summary") or {}
for key in [
    "fastpath_connected_closeout_basis",
    "scalar_known_transport_axis_closeout",
    "generated_typed_hako_artifact_shadow_consumed",
]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
need(summary.get("connected_surface_row_count") == 1, "summary connected count drift")
need(summary.get("known_unconnected_surface_row_count") == 5, "summary unconnected count drift")
for key in [
    "fastpath_connected_closeout",
    "hako_runtime_route_authority",
    "rust_fastpath_rewired",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectFastpathConnectedCloseoutInventoryRerun", "decision kind drift")
need(decision.get("reason_token") == "FastpathConnectedCloseoutBasisDefinedMapStoreI64OnlyConnected", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "fastpath_connected_closeout_basis",
    "basis_only",
    "scalar_known_transport_axis_closeout",
    "generated_typed_hako_artifact_shadow_consumed",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "fastpath_connected_closeout",
    "hako_runtime_route_authority",
    "rust_fastpath_rewired",
    "source_selfhost_claim",
    "hako_generation",
    "new_route_authority",
    "behavior_change",
    "runtime_mutation_authority",
    "publication_execution",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "native_seed_materialization",
    "new_python_semantic_projector",
    "manual_axis_selection",
    "manual_carrier_selection",
    "manual_subsurface_selection",
    "row_count_as_proof",
    "route_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("3365-MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-connected-closeout-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_connected_closeout_basis_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-connected-closeout-basis")
print("fastpath_connected_closeout_basis=1")
print("connected_surface_row_count=1")
print("known_unconnected_surface_row_count=5")
print("fastpath_connected_closeout=0")
print("hako_runtime_route_authority=0")
print("rust_fastpath_rewired=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-transport-closeout-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_transport_closeout_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2101-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-RERUN-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" "$MANIFEST" "$ROOT" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[5], encoding="utf-8"))
root = Path(sys.argv[6])


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownTransportCloseoutRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_target_axis") == "ScalarKnownTransportAxis", "target axis drift")
need(inputs.get("current_target_requirement") == "ScalarKnownCloseoutAuthority", "target requirement drift")
need(inputs.get("closeout_basis", "").endswith("mirbuilder-carrier-type-scalar-known-transport-closeout-basis-v0.json"), "basis input drift")

accepted = fixture.get("accepted_scoped_closeouts") or []
need(len(accepted) == 1, "accepted scoped closeout count drift")
need(accepted[0].get("contract_id") == "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract", "contract drift")
need(accepted[0].get("route_kind") == "MapLoadScalarI64", "route drift")

uncovered = fixture.get("uncovered_scalar_known_surfaces") or []
need(len(uncovered) == 3, "uncovered surface count drift")
expected_sources = {
    "src/mir/generic_method_route_plan/string_routes.rs": "GenericMethodReturnShape::ScalarI64",
    "src/mir/generic_method_route_plan/collection_read_routes.rs": "GenericMethodReturnShape::ScalarI64",
    "src/mir/generic_method_route_plan/write_routes.rs": "GenericMethodReturnShape::ScalarI64",
}
for rel_path, token_text in expected_sources.items():
    text = (root / rel_path).read_text(encoding="utf-8")
    need(token_text in text, f"missing scalar return token in {rel_path}")
    need("GenericMethodPublicationPolicy::NoPublication" in text, f"missing no-publication token in {rel_path}")

summary = fixture.get("summary") or {}
need(summary.get("accepted_scoped_closeout_count") == 1, "summary scoped closeout drift")
need(summary.get("uncovered_scalar_known_surface_count") == 3, "summary uncovered drift")
need(summary.get("scalar_known_transport_axis_closeout") == 0, "axis closeout drift")
need(summary.get("scoped_map_load_scalar_i64_closeout") == 1, "scoped closeout drift")
need(summary.get("source_selfhost_claim") == 0, "source selfhost summary drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepScopedCloseout", "decision kind drift")
need(decision.get("reason_token") == "ScalarKnownTransportAxisHasUncoveredScalarSurfaces", "reason drift")
need(decision.get("selected_carrier_type_axis") is None, "axis must not be selected")
need(decision.get("selected_component_requirement") == "ScalarKnownCloseoutAuthority", "component drift")
need(decision.get("selected_next_card") == design_stop, "next drift")
need(decision.get("consultation_required") is True, "consultation drift")

claims = fixture.get("claims") or {}
need(claims.get("scoped_map_load_scalar_i64_closeout") == 1, "missing scoped closeout claim")
need(claims.get("accepted_scoped_closeout_count") == 1, "accepted scoped count claim drift")
for key in [
    "scalar_known_transport_axis_closeout",
    "concrete_carrier_type_axis_selection",
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "manual_axis_selection",
    "manual_carrier_selection",
    "row_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2101-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-transport-closeout-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_transport_closeout_rerun_guard.sh"), "manifest guard drift")

need(state.get("latest_card") == token, "CURRENT_STATE latest drift")
need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")
need(token in task_order, "task order missing token")
need(f"selected_next_card={design_stop}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-transport-closeout-rerun")
print("scoped_map_load_scalar_i64_closeout=1")
print("scalar_known_transport_axis_closeout=0")
print("selected_next_card=" + design_stop)
print("consultation_required=1")
print("source_selfhost_claim=0")
print("summary=ok")
PY

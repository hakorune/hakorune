#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-subsurface-priority-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_subsurface_priority_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2113-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SUBSURFACE-PRIORITY-BASIS-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

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


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SUBSURFACE-PRIORITY-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SUBSURFACE-PRIORITY-RERUN-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteSubsurfacePriorityBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("write_result_policy_rerun_decision") == "SelectWriteSubsurfacePriorityBasis", "previous decision drift")
need(inputs.get("write_result_policy_rerun_selected_next_card") == token, "previous next drift")
need(inputs.get("write_subsurface_candidate_count") == 3, "previous candidate drift")
need(inputs.get("whole_direct_contract_candidate_count") == 0, "previous whole count drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("basis_selects_write_subsurface") is False, "basis selection drift")
need(rule.get("rerun_may_select_subsurface_only_if_exactly_one_proof_tuple_complete") is True, "rerun rule drift")
need(rule.get("if_zero_subsurface_proof_tuples_keep_stopped") is True, "zero stop drift")
need(rule.get("if_multiple_subsurface_proof_tuples_keep_stopped") is True, "multiple stop drift")
need(set(rule.get("forbidden_priority_sources") or []) == {
    "route_count",
    "owner_name",
    "source_path",
    "route_membership_alone",
    "lexical_order",
    "coverage_percentage",
    "apparent_simplicity",
    "accepted_read_contract_similarity",
    "manual_subsurface_selection",
}, "forbidden source drift")

rows = {row.get("subsurface_id"): row for row in fixture.get("candidate_subsurfaces") or []}
need(set(rows) == {"PushSurfacePolicy", "DeleteSurfacePolicy", "SetSurfacePolicy"}, "candidate drift")
need(rows["PushSurfacePolicy"].get("routes") == ["ArrayAppendAny"], "push route drift")
need(rows["DeleteSurfacePolicy"].get("routes") == ["MapDeleteAny"], "delete route drift")
need(rows["SetSurfacePolicy"].get("routes") == ["MapStoreI64", "MapStoreAny"], "set route drift")
for row in rows.values():
    need(row.get("scope_eligible") is True, "scope drift")
    need(row.get("route_count_as_proof") is False, "route count proof drift")
    need(row.get("proof_tuple_complete") is False, "proof tuple drift")
    need(row.get("selection_eligible") is False, "basis eligible drift")
    for key in [
        "stable_result_publication_contract",
        "mutation_semantics_policy_ready",
        "direct_contract_shape_ready",
        "typed_value_boundary_ready",
    ]:
        need((row.get(key) or {}).get("status") == "NotEvaluatedAtBasis", f"{key} drift")

allowed = fixture.get("allowed_proof_axes") or {}
need(set(allowed) == {
    "stable_result_publication_contract",
    "mutation_semantics_policy_ready",
    "direct_contract_shape_ready",
    "typed_value_boundary_ready",
}, "allowed proof axes drift")

summary = fixture.get("summary") or {}
need(summary.get("write_subsurface_priority_basis") == 1, "summary basis drift")
need(summary.get("candidate_write_subsurface_count") == 3, "summary candidate drift")
need(summary.get("basis_selection_eligible_subsurface_count") == 0, "summary eligible drift")
need(summary.get("basis_selects_write_subsurface") == 0, "summary selection drift")
for key in [
    "write_direct_closeout_materialized",
    "write_result_policy_ready",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWriteSubsurfacePriorityRerun", "decision kind drift")
need(decision.get("reason_token") == "WriteSubsurfacePriorityBasisDefined", "reason drift")
need(decision.get("selected_subsurface") is None, "selected subsurface drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("write_subsurface_priority_basis") == 1, "missing basis claim")
for key in [
    "basis_selects_write_subsurface",
    "write_direct_closeout_materialized",
    "write_result_policy_ready",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "component_specific_direct_contract_materialized",
    "hako_adoption",
    "source_selfhost_claim",
    "new_route_authority",
    "behavior_change",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "new_python_semantic_projector",
    "manual_subsurface_selection",
    "manual_axis_selection",
    "manual_carrier_selection",
    "route_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
    "apparent_simplicity_as_proof",
    "accepted_read_contract_similarity_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2113-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SUBSURFACE-PRIORITY-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-subsurface-priority-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_subsurface_priority_basis_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-subsurface-priority-basis")
print("write_subsurface_priority_basis=1")
print("candidate_write_subsurface_count=3")
print("basis_selects_write_subsurface=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-connected-closeout-all-surfaces-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_connected_closeout_all_surfaces_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3383-MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-ALL-SURFACES-BASIS-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
SHADOW_SOURCE="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-connected-closeout-all-surfaces-basis"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" "$SHADOW_SOURCE"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$SHADOW_SOURCE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
shadow_source = Path(sys.argv[5]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-ALL-SURFACES-BASIS-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-RERUN-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathConnectedCloseoutAllSurfacesBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("inventory_selected_next_card") == token, "inventory next drift")
need(inputs.get("bridge_plan_selected_path") == "C_SHADOW_TYPED_ARTIFACT_FIRST_THEN_HAKO_CALLER_ORIENTATION", "bridge path drift")
for key in ["inventory_rerun_006_hash", "bridge_plan_hash"]:
    need(inputs.get(key), f"missing input hash: {key}")

basis = fixture.get("basis") or {}
need(basis.get("basis_only") is True, "basis-only drift")
need(basis.get("required_connection_kind") == "CheckedInGeneratedTypedHakoArtifactShadowConsumedAtRustFastpathDecisionPoint", "connection kind drift")
need(basis.get("required_connected_surface_row_count") == 6, "required connected drift")
need(basis.get("required_known_unconnected_surface_row_count") == 0, "required unconnected drift")
need(len(basis.get("connected_surface_rows") or []) == 6, "basis connected rows drift")
need(basis.get("known_unconnected_surface_rows") == [], "basis unconnected rows drift")
need(basis.get("rust_authority_retained") is True, "rust authority drift")
need(basis.get("hako_runtime_route_authority") is False, "hako authority drift")
need(basis.get("runtime_source_text_parsing_allowed") is False, "source parsing drift")
need(basis.get("closeout_rerun_required") is True, "rerun required drift")

rule = basis.get("closeout_acceptance_rule") or {}
for key in [
    "all_known_surface_rows_shadow_consumed",
    "write_surface_connection_complete",
    "read_surface_connection_complete",
    "generated_typed_artifact_check_guards_required",
]:
    need(rule.get(key) is True, f"required rule drift: {key}")
for key in ["runtime_authority_switch_allowed", "row_count_alone_as_proof", "route_count_as_proof"]:
    need(rule.get(key) is False, f"forbidden rule drift: {key}")

for needle in [
    "mapstore_i64_shadow_consumed_decision",
    "mapstore_any_hako_route_authority_pilot_decision",
    "write_push_shadow_consumed_decision",
    "mapload_scalar_i64_shadow_consumed_decision",
    "string_scalar_i64_shadow_consumed_decision",
    "collection_scalar_i64_shadow_consumed_decision",
]:
    need(needle in shadow_source, f"shadow consumer missing {needle}")
need("include_str!" not in shadow_source, "runtime source text parsing present")
need("split('|')" not in shadow_source, "runtime split parser present")

summary = fixture.get("summary") or {}
for key in [
    "fastpath_connected_closeout_all_surfaces_basis",
    "basis_only",
    "write_surface_connection_complete",
    "read_surface_connection_complete",
    "all_known_scalar_known_surfaces_shadow_consumed",
]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
need(summary.get("connected_surface_row_count") == 6, "summary connected drift")
need(summary.get("known_unconnected_surface_row_count") == 0, "summary unconnected drift")
for key in ["fastpath_connected_closeout", "hako_runtime_route_authority", "rust_fastpath_rewired", "source_selfhost_claim"]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectFastpathConnectedCloseoutRerun", "decision kind drift")
need(decision.get("reason_token") == "AllKnownScalarKnownFastpathConnectedCloseoutBasisDefined", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "fastpath_connected_closeout_all_surfaces_basis",
    "basis_only",
    "write_surface_connection_complete",
    "read_surface_connection_complete",
    "all_known_scalar_known_surfaces_shadow_consumed",
]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
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
    "source_selfhost_claim",
    "hako_generation",
    "new_route_authority",
    "behavior_change",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "native_seed_materialization",
    "new_python_semantic_projector",
    "manual_axis_selection",
    "manual_carrier_selection",
    "manual_subsurface_selection",
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
need(manifest_row.get("card", "").endswith("3383-MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-ALL-SURFACES-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-connected-closeout-all-surfaces-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_connected_closeout_all_surfaces_basis_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-connected-closeout-all-surfaces-basis")
print("fastpath_connected_closeout_all_surfaces_basis=1")
print("connected_surface_row_count=6")
print("known_unconnected_surface_row_count=0")
print("fastpath_connected_closeout=0")
print("hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

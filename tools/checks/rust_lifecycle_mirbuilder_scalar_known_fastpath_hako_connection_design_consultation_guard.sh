#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-hako-connection-design-consultation-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_hako_connection_design_consultation.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3342-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-CONNECTION-DESIGN-CONSULTATION-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-connection-design-consultation"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST"

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


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-CONNECTION-DESIGN-CONSULTATION-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-SET-MAPSTORE-I64-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathHakoConnectionDesignConsultationV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("inventory_decision") == "DesignConsultationRequired", "inventory decision drift")
need(inputs.get("mapstore_i64_hako_adopted") == 1, "MapStoreI64 adoption drift")
need(inputs.get("mapstore_i64_scoped_closeout_materialized") == 1, "MapStoreI64 closeout drift")

result = fixture.get("consultation_result") or {}
need(result.get("choice") == "B-shadow-consumption-first", "choice drift")
need(result.get("connection_mechanism") == "CompiledOrGeneratedHakoPolicyArtifactShadowConsumedAtRustFastpath", "mechanism drift")
need(result.get("first_surface") == "SetSurfacePolicy/MapStoreI64", "surface drift")
need(result.get("mismatch_policy") == "FailGuardDiagnostic", "mismatch policy drift")
authority = result.get("authority_policy") or {}
need(authority.get("rust_fastpath_authority_retained") is True, "rust authority drift")
need(authority.get("hako_fastpath_shadow_consumed") is True, "shadow policy drift")
for key in [
    "hako_runtime_route_authority",
    "hako_backend_lowering_authority",
    "route_selection_authority_switch",
]:
    need(authority.get(key) is False, f"forbidden authority drift: {key}")

rule = fixture.get("selection_rule") or {}
need(rule.get("immediate_authority_switch_allowed") is False, "authority switch rule drift")
need(rule.get("shadow_consumption_required_before_authority_switch") is True, "shadow rule drift")
need(rule.get("first_surface_must_avoid_any_write_boundary") is True, "Any boundary rule drift")

summary = fixture.get("summary") or {}
for key in [
    "fastpath_hako_connection_design_consultation",
    "selected_connection_mechanism_shadow_consumption",
    "selected_surface_set_mapstore_i64",
    "hako_adopted_as_executable_mirror",
    "rust_fastpath_authority_retained",
]:
    need(summary.get(key) == 1, f"missing summary claim: {key}")
for key in [
    "fastpath_connected_closeout",
    "hako_fastpath_runtime_authority",
    "route_selection_authority_switch",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectFastpathShadowConsumeHandoff", "decision kind drift")
need(decision.get("reason_token") == "MapStoreI64HakoAdoptedScopedCloseoutAvoidsAnyBoundary", "reason drift")
need(decision.get("selected_surface") == "SetSurfacePolicy/MapStoreI64", "decision surface drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("design_consultation_consumed") == 1, "consultation claim drift")
need(claims.get("shadow_consumption_first_connection_selected") == 1, "shadow selection claim drift")
for key in [
    "hako_fastpath_shadow_consumed",
    "rust_fastpath_rewired",
    "hako_runtime_route_authority",
    "hako_backend_lowering_authority",
    "route_selection_authority",
    "new_route_authority",
    "runtime_mutation_authority",
    "publication_execution",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("3342-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-CONNECTION-DESIGN-CONSULTATION-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-hako-connection-design-consultation-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_connection_design_consultation_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-connection-design-consultation")
print("selected_connection_mechanism_shadow_consumption=1")
print("selected_surface=SetSurfacePolicy/MapStoreI64")
print("rust_fastpath_authority_retained=1")
print("hako_fastpath_shadow_consumed=0")
print("hako_runtime_route_authority=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

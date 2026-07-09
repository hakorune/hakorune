#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-mapload-hako-authority-pilot-rerun"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-mapload-hako-authority-pilot-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_mapload_hako_authority_pilot_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3390-MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-RERUN-001.md"
NEXT_CARD="$ROOT/docs/development/current/main/phases/phase-296x/3391-MIRBUILDER-SCALAR-KNOWN-FASTPATH-NEXT-HAKO-AUTHORITY-SURFACE-DESIGN-STOP-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
SHADOW_SOURCE="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
COLLECTION_READ_ROUTES="$ROOT/src/mir/generic_method_route_plan/collection_read_routes.rs"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$NEXT_CARD" "$TASK_ORDER" \
  "$MANIFEST" "$SHADOW_SOURCE" "$COLLECTION_READ_ROUTES"

python3 "$TOOL" --check
cargo test -q scalar_known_hako_shadow

python3 - "$FIXTURE" "$CARD" "$NEXT_CARD" "$TASK_ORDER" "$MANIFEST" "$SHADOW_SOURCE" "$COLLECTION_READ_ROUTES" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
next_card_text = Path(sys.argv[3]).read_text(encoding="utf-8")
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[5]).read_text(encoding="utf-8"))
shadow_source = Path(sys.argv[6]).read_text(encoding="utf-8")
collection_read_routes = Path(sys.argv[7]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-RERUN-001"
next_token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-NEXT-HAKO-AUTHORITY-SURFACE-DESIGN-STOP-001"
authority_fn = "mapload_scalar_i64_hako_route_authority_pilot_decision"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathMaploadHakoAuthorityPilotRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(next_token in next_card_text, "next design-stop card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("pilot_selected_next_card") == token, "pilot next drift")
need(inputs.get("mapload_hako_route_decision_authority_pilot") == 1, "pilot claim drift")

rerun = fixture.get("rerun") or {}
for key in [
    "mapload_hako_route_decision_authority_pilot",
    "mapload_rust_oracle_compat_checker",
    "mapload_mismatch_fail_fast",
    "live_route_calls_authority_pilot",
    "next_authority_step_requires_design_consultation",
]:
    need(rerun.get(key) is True, f"rerun positive drift: {key}")
need(rerun.get("scalar_known_hako_runtime_route_authority") is False, "global authority drift")
need(authority_fn in shadow_source, "authority helper missing")
need(authority_fn in collection_read_routes, "live route missing authority helper")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStoppedForNextHakoAuthoritySurfaceDesign", "decision kind drift")
need(decision.get("selected_next_card") == next_token, "decision next drift")

summary = fixture.get("summary") or {}
for key in [
    "mapload_hako_authority_pilot_rerun",
    "mapload_hako_route_decision_authority_pilot",
    "mapload_rust_oracle_compat_checker",
    "mapload_mismatch_fail_fast",
    "next_authority_step_design_consultation_required",
]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
for key in ["scalar_known_hako_runtime_route_authority", "source_selfhost_claim"]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "mapload_hako_authority_pilot_rerun",
    "mapload_hako_route_decision_authority_pilot",
    "mapload_rust_oracle_compat_checker",
    "mapload_mismatch_fail_fast",
    "next_authority_step_design_consultation_required",
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
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("3390-MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-mapload-hako-authority-pilot-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_mapload_hako_authority_pilot_rerun_guard.sh"), "manifest guard drift")
need(token in task_order, "task order missing token")
need(f"selected_next_card={next_token}" in task_order, "task order next missing")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-mapload-hako-authority-pilot-rerun")
print("mapload_hako_authority_pilot_rerun=1")
print("mapload_hako_route_decision_authority_pilot=1")
print("next_authority_step_design_consultation_required=1")
print("scalar_known_hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_token)
print("summary=ok")
PY

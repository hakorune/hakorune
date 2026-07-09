#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-all-surface-mismatch-gate-hardening"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-all-surface-mismatch-gate-hardening-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_all_surface_mismatch_gate_hardening.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3386-MIRBUILDER-SCALAR-KNOWN-FASTPATH-ALL-SURFACE-MISMATCH-GATE-HARDENING-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
SHADOW_SOURCE="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
WRITE_ROUTES="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"
STRING_ROUTES="$ROOT/src/mir/generic_method_route_plan/string_routes.rs"
COLLECTION_READ_ROUTES="$ROOT/src/mir/generic_method_route_plan/collection_read_routes.rs"

ARTIFACT_GENERATORS=(
  "src/mir/generic_method_route_plan/generated/write_push_hako_policy.rs|tools/rust_lifecycle/generate_write_push_hako_policy.py"
  "src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_hako_policy.rs|tools/rust_lifecycle/generate_write_set_mapstore_i64_hako_policy.py"
  "src/mir/generic_method_route_plan/generated/write_set_mapstore_any_hako_policy.rs|tools/rust_lifecycle/generate_write_set_mapstore_any_hako_policy.py"
  "src/mir/generic_method_route_plan/generated/mapload_scalar_i64_hako_policy.rs|tools/rust_lifecycle/generate_mapload_scalar_i64_hako_policy.py"
  "src/mir/generic_method_route_plan/generated/string_search_scalar_i64_hako_policy.rs|tools/rust_lifecycle/generate_string_search_scalar_i64_hako_policy.py"
  "src/mir/generic_method_route_plan/generated/collection_len_scalar_i64_hako_policy.rs|tools/rust_lifecycle/generate_collection_len_scalar_i64_hako_policy.py"
)

guard_require_command "$TAG" python3
guard_require_command "$TAG" diff
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" \
  "$SHADOW_SOURCE" "$WRITE_ROUTES" "$STRING_ROUTES" "$COLLECTION_READ_ROUTES"

for entry in "${ARTIFACT_GENERATORS[@]}"; do
  IFS='|' read -r artifact generator <<<"$entry"
  guard_require_files "$TAG" "$ROOT/$artifact" "$ROOT/$generator"
done

python3 "$TOOL" --check

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT
for entry in "${ARTIFACT_GENERATORS[@]}"; do
  IFS='|' read -r artifact generator <<<"$entry"
  out="$TMP_DIR/$(basename "$artifact")"
  python3 "$ROOT/$generator" > "$out"
  diff -u "$ROOT/$artifact" "$out"
done

cargo test -q scalar_known_hako_shadow

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$SHADOW_SOURCE" "$WRITE_ROUTES" "$STRING_ROUTES" "$COLLECTION_READ_ROUTES" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[4]).read_text(encoding="utf-8"))
shadow_source = Path(sys.argv[5]).read_text(encoding="utf-8")
write_routes = Path(sys.argv[6]).read_text(encoding="utf-8")
string_routes = Path(sys.argv[7]).read_text(encoding="utf-8")
collection_read_routes = Path(sys.argv[8]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-ALL-SURFACE-MISMATCH-GATE-HARDENING-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-DESIGN-CONSULTATION-001"

need(fixture.get("schema_version") == 0, "bad schema_version")
need(fixture.get("kind") == "MirBuilderScalarKnownFastpathAllSurfaceMismatchGateHardeningV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("closeout_selected_next_card") == "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-CALLER-ORIENTATION-AUTHORITY-DESIGN-STOP-001", "closeout input drift")
need(inputs.get("closeout_fastpath_connected") == 1, "closeout must be connected")

surfaces = fixture.get("surfaces") or []
need(len(surfaces) == 6, "surface row count drift")

route_sources = {
    "src/mir/generic_method_route_plan/write_routes.rs": write_routes,
    "src/mir/generic_method_route_plan/string_routes.rs": string_routes,
    "src/mir/generic_method_route_plan/collection_read_routes.rs": collection_read_routes,
}
for row in surfaces:
    live_call = row.get("live_call")
    helper = row.get("helper")
    route_file = row.get("live_route_file")
    artifact = Path(row.get("generated_artifact") or "")
    generator = row.get("generator") or ""
    hako_source = row.get("hako_source") or ""
    need(live_call in shadow_source, f"shadow source missing live call: {live_call}")
    need(helper in shadow_source, f"shadow source missing mismatch helper: {helper}")
    need(live_call in route_sources.get(route_file, ""), f"route source missing live call: {live_call}")
    need(str(artifact).endswith(".rs"), "bad artifact path")
    need(generator.endswith(".py"), "bad generator path")
    need(hako_source.endswith(".hako"), "bad hako source path")

for forbidden in ["include_str!", "split('|')"]:
    need(forbidden not in shadow_source, f"runtime text parser token present: {forbidden}")

need(shadow_source.count("#[should_panic") >= 13, "not enough mismatch should_panic tests")
for needle in [
    "mapstore_i64_shadow_rejects_role_mismatch",
    "write_push_shadow_rejects_publication_mismatch",
    "mapstore_any_shadow_rejects_any_boundary_policy_mismatch",
    "mapload_scalar_i64_shadow_rejects_role_mismatch",
    "string_scalar_i64_shadow_rejects_role_mismatch",
    "collection_scalar_i64_shadow_rejects_role_mismatch",
]:
    need(needle in shadow_source, f"missing mismatch test: {needle}")

hardening = fixture.get("hardening") or {}
need(hardening.get("all_scalar_known_shadow_mismatch_gate_current") is True, "mismatch gate drift")
need(hardening.get("generated_typed_artifact_drift_check_current") is True, "drift check drift")
need(hardening.get("shadow_consumer_mismatch_tests_current") is True, "mismatch tests drift")
need(hardening.get("runtime_hako_source_text_parsing") is False, "runtime text parsing drift")
need(hardening.get("rust_authority_retained") is True, "rust authority drift")
need(hardening.get("hako_runtime_route_authority") is False, "hako authority drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectMapLoadAuthorityPilotDesignConsultation", "decision kind drift")
need(decision.get("reason_token") == "AllSurfaceMismatchGateCurrentRustAuthorityRetained", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

summary = fixture.get("summary") or {}
for key in [
    "all_surface_mismatch_gate_hardening",
    "all_scalar_known_shadow_mismatch_gate_current",
    "generated_typed_artifact_drift_check_current",
    "shadow_consumer_mismatch_tests_current",
    "rust_authority_retained",
]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
need(summary.get("connected_surface_row_count") == 6, "summary count drift")
for key in ["runtime_hako_source_text_parsing", "hako_runtime_route_authority", "source_selfhost_claim"]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "all_surface_mismatch_gate_hardening",
    "all_scalar_known_shadow_mismatch_gate_current",
    "generated_typed_artifact_drift_check_current",
    "shadow_consumer_mismatch_tests_current",
    "rust_authority_retained",
]:
    need(claims.get(key) == 1, f"claim positive drift: {key}")
for key in [
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
need(manifest_row.get("card", "").endswith("3386-MIRBUILDER-SCALAR-KNOWN-FASTPATH-ALL-SURFACE-MISMATCH-GATE-HARDENING-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-all-surface-mismatch-gate-hardening-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_all_surface_mismatch_gate_hardening_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order missing next")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-all-surface-mismatch-gate-hardening")
print("all_scalar_known_shadow_mismatch_gate_current=1")
print("generated_typed_artifact_drift_check_current=1")
print("shadow_consumer_mismatch_tests_current=1")
print("runtime_hako_source_text_parsing=0")
print("hako_runtime_route_authority=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

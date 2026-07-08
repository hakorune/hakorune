#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3353-MIRBUILDER-SCALAR-KNOWN-WRITE-DELETE-SURFACE-MIRROR-RETIRE-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
WRITE_ROUTES="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"
ROUTE_PLAN="$ROOT/src/mir/generic_method_route_plan.rs"
MODEL="$ROOT/src/mir/generic_method_route_plan/model.rs"
SCALAR_SHADOW="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-scalar-known-write-delete-surface-mirror-retire"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$STATE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$WRITE_ROUTES" "$ROUTE_PLAN" "$MODEL" "$SCALAR_SHADOW"

python3 - "$ROOT" "$STATE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$WRITE_ROUTES" "$ROUTE_PLAN" "$MODEL" "$SCALAR_SHADOW" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

root, state_path, card_path, task_order_path, manifest_path, write_routes_path, route_plan_path, model_path, scalar_shadow_path = map(Path, sys.argv[1:])
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
card = card_path.read_text(encoding="utf-8")
task_order = task_order_path.read_text(encoding="utf-8")
manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
write_routes = write_routes_path.read_text(encoding="utf-8")
route_plan = route_plan_path.read_text(encoding="utf-8")
model = model_path.read_text(encoding="utf-8")
scalar_shadow = scalar_shadow_path.read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-WRITE-DELETE-SURFACE-MIRROR-RETIRE-001"
next_card = "MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-CURRENT-GATE-001"

need(state.get("latest_card") == token, "CURRENT_STATE latest drift")
need(state.get("latest_card_path", "").endswith(card_path.name), "CURRENT_STATE latest path drift")
need(state.get("current_blocker_token") == next_card, "CURRENT_STATE blocker drift")
need(token in card, "card missing token")
need(next_card in card, "card missing selected next")
need(token in task_order, "task-order missing token")
need(f"selected_next_card={next_card}" in task_order, "task-order selected next drift")

deleted_paths = [
    "lang/src/compiler/lib/write_delete_surface_policy_classifier.hako",
    "tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_delete_surface_hako_adoption_decision.py",
    "tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_delete_surface_rust_oracle_parity_fixture.py",
    "tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_delete_surface_direct_closeout_rerun.py",
    "tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_delete_surface_typed_direct_closeout_contract_basis.py",
    "tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_delete_surface_hako_parity_pilot.py",
    "tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_remaining_subsurface_post_delete_adoption_rerun.py",
    "tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_remaining_subsurface_post_delete_closeout_rerun.py",
    "tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_delete_surface_direct_closeout_rerun_guard.sh",
    "tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_delete_surface_hako_adoption_decision_guard.sh",
    "tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_delete_surface_hako_parity_pilot_guard.sh",
    "tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_delete_surface_parity_gate.sh",
    "tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_delete_surface_rust_oracle_parity_fixture_guard.sh",
    "tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_delete_surface_typed_direct_closeout_contract_basis_guard.sh",
    "tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_remaining_subsurface_post_delete_adoption_rerun_guard.sh",
    "tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_remaining_subsurface_post_delete_closeout_rerun_guard.sh",
    "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-delete-surface-direct-closeout-rerun-v0.json",
    "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-delete-surface-hako-adoption-decision-v0.json",
    "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-delete-surface-hako-parity-pilot-v0.json",
    "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-delete-surface-rust-oracle-v0.json",
    "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-delete-surface-typed-direct-closeout-contract-basis-v0.json",
    "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-delete-adoption-rerun-v0.json",
    "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-delete-closeout-rerun-v0.json",
    "docs/development/current/main/phases/phase-296x/2123-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-RUST-ORACLE-PARITY-FIXTURE-001.md",
    "docs/development/current/main/phases/phase-296x/2124-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-HAKO-PARITY-PILOT-001.md",
    "docs/development/current/main/phases/phase-296x/2125-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-PARITY-GATE-001.md",
    "docs/development/current/main/phases/phase-296x/2126-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-HAKO-ADOPTION-DECISION-001.md",
    "docs/development/current/main/phases/phase-296x/2127-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-POST-DELETE-ADOPTION-RERUN-001.md",
    "docs/development/current/main/phases/phase-296x/2128-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001.md",
    "docs/development/current/main/phases/phase-296x/2129-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-DIRECT-CLOSEOUT-RERUN-001.md",
    "docs/development/current/main/phases/phase-296x/2130-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-POST-DELETE-CLOSEOUT-RERUN-001.md",
]
for rel in deleted_paths:
    need(not (root / rel).exists(), f"retired artifact still exists: {rel}")

retired_tokens = {
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-RUST-ORACLE-PARITY-FIXTURE-001",
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-HAKO-PARITY-PILOT-001",
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-PARITY-GATE-001",
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-HAKO-ADOPTION-DECISION-001",
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-POST-DELETE-ADOPTION-RERUN-001",
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001",
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-DIRECT-CLOSEOUT-RERUN-001",
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-POST-DELETE-CLOSEOUT-RERUN-001",
}
manifest_tokens = {row.get("token") for row in manifest.get("rows") or []}
need(not (retired_tokens & manifest_tokens), "manifest still lists retired DeleteSurface rows")

for needle in [
    "pub(super) fn match_generic_delete_route",
    "GenericMethodRouteKind::MapDeleteAny",
    "GenericMethodRouteProof::DeleteSurfacePolicy",
    "CoreMethodOp::MapDelete",
    "GenericMethodReturnShape::ScalarI64",
]:
    need(needle in write_routes, f"live Rust delete route lost: {needle}")
need('"delete" => match_generic_delete_route' in route_plan, "route plan no longer calls delete matcher")
need("MapDeleteAny" in model and "DeleteSurfacePolicy" in model, "route model lost delete vocabulary")
need("write_delete_surface_policy_classifier" not in scalar_shadow, "scalar shadow still consumes retired Delete .hako mirror")

for claim in [
    "delete_surface_hako_mirror_retired = 1",
    "delete_surface_lifecycle_artifacts_deleted = 1",
    "delete_surface_manifest_rows_removed = 1",
    "rust_map_delete_route_preserved = 1",
    "map_delete_any_runtime_semantics_preserved = 1",
]:
    need(claim in card, f"card claim missing: {claim}")
for non_claim in [
    "rust_map_delete_route_deleted = 0",
    "runtime_behavior_change = 0",
    "write_scalar_i64_routes_closeout = 0",
    "scalar_known_transport_axis_closeout = 0",
    "hako_runtime_route_authority = 0",
    "hako_backend_lowering_authority = 0",
    "source_selfhost_claim = 0",
]:
    need(non_claim in card, f"card non-claim missing: {non_claim}")

print("delete_surface_hako_mirror_retired=1")
print("delete_surface_lifecycle_artifacts_deleted=1")
print("delete_surface_manifest_rows_removed=1")
print("rust_map_delete_route_preserved=1")
print("map_delete_any_runtime_semantics_preserved=1")
print("source_selfhost_claim=0")
PY

cargo check -q --lib
cargo test -q --lib map_value_delete_remove_use_unified_receiver_arg_shape_and_receipt_string_return

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-scalar-known-write-delete-surface-mirror-retire
token=MIRBUILDER-SCALAR-KNOWN-WRITE-DELETE-SURFACE-MIRROR-RETIRE-001
delete_surface_hako_mirror_retired=1
delete_surface_lifecycle_artifacts_deleted=1
delete_surface_manifest_rows_removed=1
rust_map_delete_route_preserved=1
map_delete_any_runtime_semantics_preserved=1
runtime_behavior_change=0
write_scalar_i64_routes_closeout=0
scalar_known_transport_axis_closeout=0
hako_runtime_route_authority=0
hako_backend_lowering_authority=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-SIDECAR-BRIDGE-CURRENT-GATE-001
summary=ok
REPORT

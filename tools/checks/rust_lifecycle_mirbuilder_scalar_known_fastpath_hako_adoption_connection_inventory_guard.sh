#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-hako-adoption-connection-inventory-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_hako_adoption_connection_inventory.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3341-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-ADOPTION-CONNECTION-INVENTORY-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-adoption-connection-inventory"

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


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-ADOPTION-CONNECTION-INVENTORY-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-CONNECTION-DESIGN-CONSULTATION-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathHakoAdoptionConnectionInventoryV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

contract = fixture.get("contract_inventory") or {}
need(contract.get("module_declared_in_route_plan") is True, "contract module declaration drift")
need(contract.get("contract_table_defined") is True, "contract table drift")
need(contract.get("external_rust_reference_count", 0) > 0, "contract shadow connection missing")
need(contract.get("fastpath_connected") is True, "contract fastpath shadow connection drift")

hako = fixture.get("hako_adoption_inventory") or {}
need(hako.get("compiler_runtime_connection_found") is False, "hako runtime connection drift")

summary = fixture.get("summary") or {}
for key in [
    "scalar_known_fastpath_hako_adoption_connection_inventory",
    "rust_fastpath_owner_still_write_routes",
    "contract_module_declared",
    "hako_policy_mirror_guard_only",
    "closeout_chain_pause_required",
]:
    need(summary.get(key) == 1, f"missing summary claim: {key}")
for key in [
    "contract_fastpath_connected",
    "hako_fastpath_runtime_connection",
    "hako_adopted_as_runtime_authority",
    "source_selfhost_claim",
]:
    expected = 1 if key == "contract_fastpath_connected" else 0
    need(summary.get(key) == expected, f"forbidden summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "DesignConsultationRequired", "decision kind drift")
need(decision.get("reason_token") == "HakoAdoptionMirrorNotConnectedToRustFastpath", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("inventory_only") == 1, "inventory claim drift")
need(claims.get("connection_design_consultation_required") == 1, "consultation claim drift")
for key in [
    "rust_fastpath_rewired",
    "hako_runtime_route_authority",
    "hako_backend_lowering_authority",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("3341-MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-ADOPTION-CONNECTION-INVENTORY-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-scalar-known-fastpath-hako-adoption-connection-inventory-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_adoption_connection_inventory_guard.sh"), "manifest guard drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-hako-adoption-connection-inventory")
print("contract_external_rust_reference_count=" + str(summary.get("contract_external_rust_reference_count")))
print("contract_fastpath_connected=1")
print("hako_policy_mirror_guard_only=1")
print("hako_fastpath_runtime_connection=0")
print("closeout_chain_pause_required=1")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

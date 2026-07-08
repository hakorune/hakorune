#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-hako-adoption-decision-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_set_mapstore_i64_hako_adoption_decision.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2135-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-HAKO-ADOPTION-DECISION-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
PARITY_GATE="$ROOT/tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_i64_parity_gate.sh"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-hako-adoption-decision"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" "$PARITY_GATE"

python3 "$TOOL" --check
bash "$PARITY_GATE" >/dev/null

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


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-HAKO-ADOPTION-DECISION-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-POST-ADOPTION-RERUN-001"
pilot_token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-HAKO-PARITY-PILOT-001"
parity_token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-PARITY-GATE-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteSetMapStoreI64HakoAdoptionDecisionV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("hako_parity_pilot_token") == pilot_token, "pilot token drift")
need(inputs.get("hako_parity_pilot_selected_next_card") == parity_token, "pilot next drift")

evidence = fixture.get("evidence") or {}
need(evidence.get("hako_source") == "lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako", "source drift")
need(evidence.get("parity_rows") == 1, "parity row drift")
need(evidence.get("parity_status") == "green", "parity status drift")

decision = fixture.get("adoption_decision") or {}
need(decision.get("decision") == "Adopt", "decision drift")
need(decision.get("reason_token") == "WriteSetMapStoreI64RustOracleParityGateGreen", "reason drift")
need(decision.get("adopted_owner") == "write_set_mapstore_i64_policy_classifier", "owner drift")
need(decision.get("adopted_surface") == "SetSurfacePolicy/MapStoreI64", "surface drift")
need(decision.get("hako_adopted") is True, "adoption drift")
need(decision.get("rust_bootstrap_retained") is True, "bootstrap retained drift")
need(decision.get("rust_oracle_retained") is True, "oracle retained drift")
need(decision.get("mapstore_any_deferred") is True, "MapStoreAny defer drift")
need(decision.get("selected_next_card") == next_card, "next drift")

summary = fixture.get("summary") or {}
for key in [
    "decision_adopt",
    "write_set_mapstore_i64_hako_adopted",
    "hako_adopted_decision",
    "parity_gate_green",
    "rust_bootstrap_retained",
    "rust_oracle_retained",
    "mapstore_any_deferred",
]:
    need(summary.get(key) == 1, f"missing summary claim: {key}")
for key in [
    "any_write_boundary_opened",
    "write_direct_closeout_materialized",
    "write_result_policy_ready",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "source_selfhost_claim",
    "runtime_mutation_authority",
    "publication_execution",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "decision_adopt",
    "write_set_mapstore_i64_hako_adopted",
    "hako_adopted_decision",
    "parity_gate_green",
    "rust_bootstrap_retained",
    "rust_oracle_retained",
    "mapstore_any_deferred",
]:
    need(claims.get(key) == 1, f"missing positive claim: {key}")
for key in [
    "any_write_boundary_opened",
    "write_subsurface_selected",
    "write_direct_closeout_materialized",
    "write_result_policy_ready",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "component_specific_direct_contract_materialized",
    "source_selfhost_claim",
    "new_route_authority",
    "behavior_change",
    "runtime_mutation_authority",
    "publication_execution",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "native_seed_materialization",
    "hako_generation",
    "rust_deletion",
    "manual_subsurface_selection",
    "route_count_as_proof",
    "apparent_simplicity_as_proof",
    "accepted_read_contract_similarity_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2135-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-HAKO-ADOPTION-DECISION-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-hako-adoption-decision-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_i64_hako_adoption_decision_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-hako-adoption-decision")
print("decision=Adopt")
print("write_set_mapstore_i64_hako_adopted=1")
print("hako_adopted_decision=1")
print("parity_gate_green=1")
print("mapstore_any_deferred=1")
print("any_write_boundary_opened=0")
print("write_direct_closeout_materialized=0")
print("scalar_known_transport_axis_closeout=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

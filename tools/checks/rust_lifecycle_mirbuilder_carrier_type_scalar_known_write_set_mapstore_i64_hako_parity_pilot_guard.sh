#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-hako-parity-pilot-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_scalar_known_write_set_mapstore_i64_hako_parity_pilot.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2133-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-HAKO-PARITY-PILOT-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
HAKO_IMPL="$ROOT/lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako"
HAKO_BIN="$ROOT/tools/bin/hako"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-hako-parity-pilot"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST" "$HAKO_IMPL" "$HAKO_BIN"

python3 "$TOOL" --check
bash "$HAKO_BIN" --backend mir --verify "$HAKO_IMPL" >/dev/null

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" "$HAKO_IMPL" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[4], encoding="utf-8"))
hako_source = Path(sys.argv[5]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-HAKO-PARITY-PILOT-001"
next_card = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-PARITY-GATE-001"
rust_oracle = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-RUST-ORACLE-PARITY-FIXTURE-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeScalarKnownWriteSetMapStoreI64HakoParityPilotV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("rust_oracle_token") == rust_oracle, "rust oracle token drift")
need(inputs.get("rust_oracle_selected_next_card") == token, "rust oracle next drift")

impl = fixture.get("hako_implementation") or {}
need(impl.get("source") == "lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako", "hako source drift")
need(impl.get("box") == "WriteSetMapStoreI64PolicyClassifierBox", "box drift")
need(impl.get("method") == "classify", "method drift")
need(impl.get("role") == "classifier_policy_mirror_only", "role drift")
need(impl.get("publication_execution") is False, "publication execution drift")
need(impl.get("runtime_mutation_authority") is False, "runtime mutation drift")
need(impl.get("any_write_boundary_opened") is False, "any write drift")

for expected in [
    "static box WriteSetMapStoreI64PolicyClassifierBox",
    "classify(route_kind)",
    'route_kind == "MapStoreI64"',
    "map_store_i64_set_surface|SetSurfacePolicy|MapStoreI64|MapSet|ColdFallback|NoneResult|None|WriteAny|ScalarI64|NonePublication|mutate|MutatesReceiverOrContainer|classifier_policy_mirror_only",
    "unsupported_write_set_mapstore_i64_policy",
]:
    need(expected in hako_source, f"hako implementation missing: {expected}")

pilot = fixture.get("parity_pilot_fixture") or {}
need(pilot.get("fixture_id") == "WriteSetMapStoreI64HakoParityPilotV0", "fixture id drift")
need(pilot.get("row_count") == 1, "row count drift")
rows = pilot.get("rows") or []
need(len(rows) == 1, "rows drift")
need(rows[0].get("input_route_kind") == "MapStoreI64", "input route drift")
need(rows[0].get("expected", "").endswith("classifier_policy_mirror_only"), "expected role drift")

summary = fixture.get("summary") or {}
for key in [
    "write_set_mapstore_i64_hako_parity_pilot",
    "hako_implementation_landed",
    "hako_source_verifies",
    "mapstore_i64_scope",
    "set_surface_policy_scope",
    "none_result_metadata_declared",
    "none_publication_metadata_reused",
    "classifier_policy_mirror_only",
    "mapstore_any_deferred",
    "parity_gate_required",
]:
    need(summary.get(key) == 1, f"missing summary claim: {key}")
for key in [
    "any_write_boundary_opened",
    "publication_execution",
    "runtime_mutation_authority",
    "write_direct_closeout_materialized",
    "write_scalar_i64_routes_closeout",
    "scalar_known_transport_axis_closeout",
    "hako_adoption",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWriteSetMapStoreI64ParityGate", "decision kind drift")
need(decision.get("reason_token") == "WriteSetMapStoreI64HakoParityPilotLanded", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2133-MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-HAKO-PARITY-PILOT-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-hako-parity-pilot-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_i64_hako_parity_pilot_guard.sh"), "manifest guard drift")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-hako-parity-pilot")
print("hako_implementation_landed=1")
print("hako_source_verifies=1")
print("write_set_mapstore_i64_hako_parity_pilot=1")
print("parity_gate_required=1")
print("any_write_boundary_opened=0")
print("hako_adoption=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_card)
print("summary=ok")
PY

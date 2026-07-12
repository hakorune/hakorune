#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-mapstore-i64-caller-orientation-authority-pilot"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-mapstore-i64-caller-orientation-authority-pilot-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_mapstore_i64_caller_orientation_authority_pilot.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3454-MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-I64-CALLER-ORIENTATION-AUTHORITY-PILOT-001.md"
CALLER="$ROOT/src/mir/generic_method_route_plan/caller_orientation.rs"
SHADOW="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
POLICY="$ROOT/src/mir/generic_method_route_plan/generated/write_set_mapstore_route_policy.rs"
CONTRACT="$ROOT/src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_caller_orientation_contract.rs"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$CALLER" "$SHADOW" "$POLICY" "$CONTRACT"

python3 "$TOOL" --check
cargo test -q caller_orientation

python3 - "$FIXTURE" "$CARD" "$CALLER" "$SHADOW" "$POLICY" "$CONTRACT" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
caller = Path(sys.argv[3]).read_text(encoding="utf-8")
shadow = Path(sys.argv[4]).read_text(encoding="utf-8")
policy = Path(sys.argv[5]).read_text(encoding="utf-8")
contract = Path(sys.argv[6]).read_text(encoding="utf-8")


def need(condition, message):
    if not condition:
        raise SystemExit(message)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-I64-CALLER-ORIENTATION-AUTHORITY-PILOT-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-MAPSTORE-I64-CALLER-ORIENTATION-PILOT-DESIGN-STOP-001"
need(fixture.get("token") == token, "fixture token drift")
need(next_card in card, "3455 next card missing")
scope = fixture.get("scope") or {}
for key, value in {
    "surface": "SetSurfacePolicy",
    "route_kind": "MapStoreI64",
    "policy_row_id": "map_store_i64_set_surface",
    "authority_scope": "policy_row_id_contract_only",
    "consumer_input": "PolicyRowIdOnly",
    "consumer_return": "Unit",
    "key_domain": "I64",
    "stored_value_domain": "Any",
    "mutation_boundary": "DeclaredMetadataOnly",
}.items():
    need(scope.get(key) == value, f"scope drift: {key}")
claims = fixture.get("claims") or {}
for key in [
    "mapstore_i64_caller_orientation_authority_pilot",
    "mapstore_i64_caller_orientation_authority_scope_policy_row_id_contract_only",
    "mapstore_i64_caller_orientation_consumer_unit_only",
    "mapstore_i64_key_domain_i64",
    "mapstore_i64_stored_value_domain_any",
    "mapstore_i64_mismatch_fail_fast",
    "rust_route_match_authority_retained",
    "rust_compatibility_veto_retained",
]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
    "caller_selected_route_authority",
    "caller_runtime_dispatch_authority",
    "caller_orientation_runtime_path",
    "runtime_mutation_authority",
    "backend_lowering_authority",
    "publication_execution",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"non-claim drift: {key}")
need("assert_mapstore_i64_policy_row(policy.policy_row_id)" in shadow, "shadow caller contract check missing")
need("assert_mapstore_policy_row(policy)" in shadow, "shared policy validator missing")
need("policy.policy_row_id, policy_row_id" in caller, "caller policy identity check missing")
need('key_domain: "I64"' in policy and 'stored_value_domain: "Any"' in policy, "typed domains missing")
need('policy_row_id: "map_store_i64_set_surface"' in contract, "caller row missing")
need("-> GenericMethodRouteDecision" not in caller[caller.index("pub(super) fn assert_mapstore_i64_policy_row"):caller.index("pub(super) fn assert_push_arrayappendany_policy_row")], "caller authority returned a route decision")
print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-mapstore-i64-caller-orientation-authority-pilot")
print("mapstore_i64_caller_orientation_authority_pilot=1")
print("consumer_input=PolicyRowIdOnly")
print("consumer_return=Unit")
print("source_selfhost_claim=0")
print("summary=ok")
PY

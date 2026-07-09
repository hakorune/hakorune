#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-mapload-caller-orientation-authority-pilot"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-mapload-caller-orientation-authority-pilot-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_mapload_caller_orientation_authority_pilot.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3441-MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-AUTHORITY-PILOT-001.md"
NEXT_CARD="$ROOT/docs/development/current/main/phases/phase-296x/3442-MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-AUTHORITY-PILOT-RERUN-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
MODULE="$ROOT/src/mir/generic_method_route_plan/caller_orientation.rs"
SHADOW="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
POLICY="$ROOT/src/mir/generic_method_route_plan/generated/mapload_scalar_i64_hako_policy.rs"
CONTRACT="$ROOT/src/mir/generic_method_route_plan/generated/mapload_scalar_i64_caller_orientation_contract.rs"
POLICY_SOURCE="$ROOT/lang/src/compiler/lib/map_load_scalar_i64_policy_classifier.hako"
CONTRACT_SOURCE="$ROOT/lang/src/compiler/lib/map_load_scalar_i64_caller_orientation_contract.hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$NEXT_CARD" "$TASK_ORDER" "$MANIFEST" "$MODULE" "$SHADOW" "$POLICY" "$CONTRACT" "$POLICY_SOURCE" "$CONTRACT_SOURCE"

python3 "$TOOL" --check
python3 - "$FIXTURE" "$CARD" "$NEXT_CARD" "$TASK_ORDER" "$MANIFEST" "$MODULE" "$SHADOW" "$POLICY" "$CONTRACT" "$POLICY_SOURCE" "$CONTRACT_SOURCE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
next_card_path = Path(sys.argv[3])
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[5]).read_text(encoding="utf-8"))
module = Path(sys.argv[6]).read_text(encoding="utf-8")
shadow = Path(sys.argv[7]).read_text(encoding="utf-8")
policy = Path(sys.argv[8]).read_text(encoding="utf-8")
contract = Path(sys.argv[9]).read_text(encoding="utf-8")
policy_source = Path(sys.argv[10]).read_text(encoding="utf-8")
contract_source = Path(sys.argv[11]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-AUTHORITY-PILOT-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-AUTHORITY-PILOT-RERUN-001"
row_id = "map_load_scalar_i64_routes"
need(fixture.get("token") == token, "token drift")
need(token in card, "card token missing")
need(next_card in task_order and next_card in card, "next task pointer missing")
need(next_card_path.exists(), "next card file missing")
need(token in {row.get("token") for row in manifest.get("rows", [])}, "manifest token missing")
scope = fixture.get("scope") or {}
need(scope.get("exhaustive_row_ids") == [row_id], "single row scope drift")
need(scope.get("consumer_input") == "PolicyRowIdOnly", "consumer input drift")
need(scope.get("consumer_return") == "Unit", "consumer return drift")
need(module.count("pub(super) fn assert_mapload_authority_pilot(policy_row_id: &str)") == 1, "authority function drift")
need("assert_mapload_authority_pilot(policy.policy_row_id);" in shadow, "authority consumer call missing")
need("MAPLOAD_SCALAR_I64_CALLER_ORIENTATION_CONTRACT" in module, "caller contract not consumed")
need('policy_row_id: "map_load_scalar_i64_routes"' in contract, "caller contract row missing")
need(policy.count('policy_row_id: "map_load_scalar_i64_routes"') == 1, "policy row identity drift")
need(contract_source.count(f'return "{row_id}|') == 1, "caller contract source extra/missing row")
need(policy_source.count(row_id) == 1, "policy source extra/missing row")
for required in [
    'surface: "MapLoadScalarI64Routes"',
    'route_kind: GenericMethodRouteKind::MapLoadScalarI64',
    'core_op: CoreMethodOp::MapGet',
    'lowering_tier: CoreMethodLoweringTier::WarmDirectAbi',
    'value_demand: GenericMethodValueDemand::ScalarI64',
    'publication_policy: GenericMethodPublicationPolicy::NoPublication',
    'effect_class: "read"',
    'proof_family: "ScalarI64MapGetStoreFact"',
    'role: "classifier_policy_mirror_only"',
]:
    need(required in policy, f"generated policy metadata missing: {required}")
start = module.index("pub(super) fn assert_mapload_authority_pilot")
end = module.index("pub(super) fn assert_string_policy_row", start)
body = module[start:end]
need("-> GenericMethodRouteDecision" not in body, "authority pilot returns route decision")
for forbidden in ["ValueId", "runtime_dispatch", "backend_lowering_authority", "runtime_mutation", "publication_execution", "fallback"]:
    need(forbidden not in body, f"authority boundary leak: {forbidden}")
claims = fixture.get("claims") or {}
for key in [
    "mapload_caller_orientation_authority_pilot", "mapload_caller_orientation_authority_scope_policy_row_id_contract_only",
    "mapload_caller_orientation_consumer_unit_only", "mapload_hako_route_decision_authority_retained",
    "mapload_rust_oracle_compat_checker_retained", "mapload_mismatch_fail_fast",
    "read_caller_orientation_assertion_closeout_retained", "non_delete_write_caller_orientation_assertion_closeout_retained",
    "single_surface_mapload_scope", "no_new_route_authority",
]:
    need(claims.get(key) == 1, f"claim drift: {key}")
for key, value in claims.items():
    if key not in {
        "mapload_caller_orientation_authority_pilot", "mapload_caller_orientation_authority_scope_policy_row_id_contract_only",
        "mapload_caller_orientation_consumer_unit_only", "mapload_hako_route_decision_authority_retained",
        "mapload_rust_oracle_compat_checker_retained", "mapload_mismatch_fail_fast",
        "read_caller_orientation_assertion_closeout_retained", "non_delete_write_caller_orientation_assertion_closeout_retained",
        "single_surface_mapload_scope", "no_new_route_authority",
    }:
        need(value == 0, f"non-claim drift: {key}")
print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-mapload-caller-orientation-authority-pilot")
print("mapload_caller_orientation_authority_pilot=1")
print("single_surface_mapload_scope=1")
print("no_new_route_authority=1")
print("caller_orientation_runtime_path=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY

cargo test -q caller_orientation

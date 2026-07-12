#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-mapstore-i64-fact-plan-boundary-inventory"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-mapstore-i64-fact-plan-boundary-inventory-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_mapstore_i64_fact_plan_boundary_inventory.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3457-MIRBUILDER-MAPSTORE-I64-FACT-PLAN-BOUNDARY-INVENTORY-001.md"
FACTS="$ROOT/src/mir/generic_method_route_facts.rs"
EXACT_FACTS="$ROOT/src/mir/exact_numeric_value_facts.rs"
WITNESS="$ROOT/src/mir/generic_method_route_plan/mapstore_i64_key_witness.rs"
ROUTES="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"
TESTS="$ROOT/src/mir/generic_method_route_plan/tests/map_set_routes/map_get_scalar.rs"
REFRESH="$ROOT/src/mir/semantic_refresh.rs"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$FACTS" "$EXACT_FACTS" "$WITNESS" "$ROUTES" "$TESTS" "$REFRESH"

python3 "$TOOL" --check
cargo test -q generic_method_route_facts
cargo test -q mapstore_i64_key_witness

python3 - "$FIXTURE" "$CARD" "$FACTS" "$EXACT_FACTS" "$WITNESS" "$ROUTES" "$TESTS" "$REFRESH" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
facts = Path(sys.argv[3]).read_text(encoding="utf-8")
exact_facts = Path(sys.argv[4]).read_text(encoding="utf-8")
witness = Path(sys.argv[5]).read_text(encoding="utf-8")
routes = Path(sys.argv[6]).read_text(encoding="utf-8")
tests = Path(sys.argv[7]).read_text(encoding="utf-8")
refresh = Path(sys.argv[8]).read_text(encoding="utf-8")


def need(condition, message):
    if not condition:
        raise SystemExit(message)


need(fixture.get("token") == "MIRBUILDER-MAPSTORE-I64-FACT-PLAN-BOUNDARY-INVENTORY-001", "token drift")
candidates = {row["candidate_id"]: row for row in fixture.get("candidates", [])}
const = candidates["fact.mapstore.key_domain.i64_const"]
dynamic = candidates["fact.mapstore.key_domain.i64_value"]
witness_row = candidates["projection.mapstore.key_domain.exact_i64_witness"]
need(const["eligibility"] == "implemented_narrow_fact_owner", "I64Const eligibility drift")
need(dynamic["eligibility"] == "pending", "I64Value must remain pending")
need(witness_row["eligibility"] == "implemented_exact_i64_projection", "witness eligibility drift")
need(candidates["plan.mapstore.set.route_decision"]["eligibility"] == "blocked", "Plan opened")
need(candidates["boundary.mapstore.set.mutation"]["eligibility"] == "blocked", "Boundary opened")
claims = fixture.get("claims") or {}
need(claims.get("mapstore_i64_const_fact_owner_implemented") == 1, "Fact owner claim missing")
need(claims.get("existing_exact_numeric_fact_owner_reused") == 1, "exact numeric owner reuse missing")
need(claims.get("mapstore_i64_source_backed_key_witness_candidate") == 1, "witness claim missing")
need(claims.get("mapstore_i64_first_hard_scope") == "exact_i64_only", "witness scope drift")
need(claims.get("current_i64value_disposition") == "derived_projection", "I64Value disposition drift")
need(claims.get("mirtype_integer_hard_authority") == 0, "MirType hard authority opened")
need(claims.get("new_dynamic_integer_owner") == 0, "second numeric owner opened")
for key in [
    "hard_authority_activation", "route_behavior_change", "runtime_mutation_authority",
    "backend_lowering_authority", "publication_execution", "mapstore_any_opened",
    "array_append_any_opened", "delete_opened", "scalar_known_wide_opened", "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"non-claim drift: {key}")
need("MapStoreI64ConstKeyFact" in facts, "Fact owner type missing")
need("mapstore_i64_const_key_fact" in facts, "Fact owner function missing")
need("ConstValue::Integer" in facts, "Fact source evidence missing")
need("mapstore_i64_const_key_fact(function, def_map, key)" in facts, "classify_key_route bypasses Fact owner")
need("classify_key_route(function, def_map" in routes, "route consumer missing")
need("I64Const" in tests, "I64Const fixture witness missing")
need("ExactNumericValueFact" in exact_facts, "exact numeric authority missing")
need("MapStoreI64KeyWitness" in witness, "MapStore witness type missing")
need("verify_mapstore_i64_key_route" in witness, "MapStore witness verifier missing")
need('declared_type_name != "i64"' in witness, "exact i64 scope guard missing")
need("refresh_module_mapstore_i64_key_witnesses" in witness, "refresh projection missing")
exact_refresh = "refresh_module_exact_numeric_value_facts(module);"
witness_refresh = "refresh_module_mapstore_i64_key_witnesses(module);"
need(exact_refresh in refresh and witness_refresh in refresh, "refresh order call missing")
need(refresh.index(exact_refresh) < refresh.index(witness_refresh), "witness refresh precedes exact facts")
need("3457" in card and "I64Const" in card, "card scope missing")
print("output_contract=rust-lifecycle-mirbuilder-mapstore-i64-fact-plan-boundary-inventory")
print("mapstore_i64_const_fact_owner_implemented=1")
print("existing_exact_numeric_fact_owner_reused=1")
print("mapstore_i64_source_backed_key_witness=1")
print("current_i64value_disposition=derived_projection")
print("mapstore_dynamic_i64_fact_candidate=pending")
print("plan_authority_selection=0")
print("boundary_authority_selection=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY

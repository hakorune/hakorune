#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-mapstore-i64-fact-plan-boundary-inventory"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-mapstore-i64-fact-plan-boundary-inventory-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_mapstore_i64_fact_plan_boundary_inventory.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3457-MIRBUILDER-MAPSTORE-I64-FACT-PLAN-BOUNDARY-INVENTORY-001.md"
FACTS="$ROOT/src/mir/generic_method_route_facts.rs"
ROUTES="$ROOT/src/mir/generic_method_route_plan/write_routes.rs"
TESTS="$ROOT/src/mir/generic_method_route_plan/tests/map_set_routes/map_get_scalar.rs"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$FACTS" "$ROUTES" "$TESTS"

python3 "$TOOL" --check
cargo test -q generic_method_route_facts

python3 - "$FIXTURE" "$CARD" "$FACTS" "$ROUTES" "$TESTS" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
facts = Path(sys.argv[3]).read_text(encoding="utf-8")
routes = Path(sys.argv[4]).read_text(encoding="utf-8")
tests = Path(sys.argv[5]).read_text(encoding="utf-8")


def need(condition, message):
    if not condition:
        raise SystemExit(message)


need(fixture.get("token") == "MIRBUILDER-MAPSTORE-I64-FACT-PLAN-BOUNDARY-INVENTORY-001", "token drift")
candidates = {row["candidate_id"]: row for row in fixture.get("candidates", [])}
const = candidates["fact.mapstore.key_domain.i64_const"]
dynamic = candidates["fact.mapstore.key_domain.i64_value"]
need(const["eligibility"] == "implemented_narrow_fact_owner", "I64Const eligibility drift")
need(dynamic["eligibility"] == "pending", "I64Value must remain pending")
need(candidates["plan.mapstore.set.route_decision"]["eligibility"] == "blocked", "Plan opened")
need(candidates["boundary.mapstore.set.mutation"]["eligibility"] == "blocked", "Boundary opened")
claims = fixture.get("claims") or {}
need(claims.get("mapstore_i64_const_fact_owner_implemented") == 1, "Fact owner claim missing")
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
need("3457" in card and "I64Const" in card, "card scope missing")
print("output_contract=rust-lifecycle-mirbuilder-mapstore-i64-fact-plan-boundary-inventory")
print("mapstore_i64_const_fact_owner_implemented=1")
print("mapstore_dynamic_i64_fact_candidate=pending")
print("plan_authority_selection=0")
print("boundary_authority_selection=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-write-set-mapstore-any-caller-orientation-live-assert-consumer"
source "$ROOT/tools/checks/lib/guard_common.sh"

CARD="$ROOT/docs/development/current/main/phases/phase-296x/3438-MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-ANY-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
MODULE="$ROOT/src/mir/generic_method_route_plan/caller_orientation.rs"
SHADOW="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
CONTRACT="$ROOT/src/mir/generic_method_route_plan/generated/write_set_mapstore_any_caller_orientation_contract.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CARD" "$TASK_ORDER" "$MANIFEST" "$MODULE" "$SHADOW" "$CONTRACT"

python3 - "$CARD" "$TASK_ORDER" "$MANIFEST" "$MODULE" "$SHADOW" "$CONTRACT" <<'PY'
import json
import sys
from pathlib import Path

card = Path(sys.argv[1]).read_text(encoding="utf-8")
task_order = Path(sys.argv[2]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
module = Path(sys.argv[4]).read_text(encoding="utf-8")
shadow = Path(sys.argv[5]).read_text(encoding="utf-8")
contract = Path(sys.argv[6]).read_text(encoding="utf-8")

def need(condition, message):
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-ANY-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-NON-DELETE-WRITE-CALLER-ORIENTATION-ASSERTION-CLOSEOUT-001"
need(token in card, "card token missing")
need(token in {row.get("token") for row in manifest.get("rows", [])}, "manifest token missing")
need(next_card in card and next_card in task_order, "next task pointer missing")
need("pub(super) fn assert_mapstore_any_policy_row(policy_row_id: &str)" in module, "assertion signature drift")
need("assert_mapstore_any_policy_row(policy.policy_row_id);" in shadow, "MapStoreAny live assertion call missing")
need("WRITE_SET_MAPSTORE_ANY_CALLER_ORIENTATION_CONTRACT" in module, "contract artifact not consumed")
need('policy_row_id: "map_store_any_set_surface"' in contract, "contract row identity missing")
start = module.index("pub(super) fn assert_mapstore_any_policy_row")
end = module.index("#[cfg(test)]", start)
body = module[start:end]
for forbidden in ["GenericMethodRouteDecision", "route_kind", "core_op", "value_demand", "effect_class", "mutation_class", "ValueId"]:
    need(forbidden not in body, f"caller assertion boundary widened: {forbidden}")
need("-> GenericMethodRouteDecision" not in body, "caller assertion returns route decision")
need("assert_mapstore_any_policy_row(\"map_store_any_set_surface\")" in module, "positive test missing")
need("assert_mapstore_any_policy_row(\"unknown_policy_row\")" in module, "negative identity test missing")
need("mapstore_any_assertion_rejects_metadata_drift" in module, "metadata fail-fast test missing")
need("mapstore_any_hako_route_authority_pilot_decision" in shadow, "existing MapStoreAny oracle missing")
for forbidden in ["runtime_dispatch", "runtime_mutation", "publication_execution", "fallback"]:
    need(forbidden not in body, f"forbidden authority marker leaked: {forbidden}")
print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-write-set-mapstore-any-caller-orientation-live-assert-consumer")
print("mapstore_any_caller_orientation_live_assert_consumer=1")
print("caller_orientation_unit_consumer=1")
print("caller_selected_route_authority=0")
print("any_write_boundary_authority=0")
print("write_mutation_authority=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY

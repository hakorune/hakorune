#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-read-caller-orientation-assertion-closeout"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-read-caller-orientation-assertion-closeout-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_read_caller_orientation_assertion_closeout.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3430-MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-CALLER-ORIENTATION-ASSERTION-CLOSEOUT-001.md"
NEXT_CARD="$ROOT/docs/development/current/main/phases/phase-296x/3431-MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-READ-CALLER-ORIENTATION-DESIGN-STOP-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
MODULE="$ROOT/src/mir/generic_method_route_plan/caller_orientation.rs"
SHADOW="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$NEXT_CARD" "$TASK_ORDER" "$MANIFEST" "$MODULE" "$SHADOW"

python3 "$TOOL" --check
python3 - "$ROOT" "$FIXTURE" "$CARD" "$NEXT_CARD" "$TASK_ORDER" "$MANIFEST" "$MODULE" "$SHADOW" <<'PY'
import json
import sys
from pathlib import Path

root = Path(sys.argv[1])
fixture = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
card = Path(sys.argv[3]).read_text(encoding="utf-8")
next_card_path = Path(sys.argv[4])
task_order = Path(sys.argv[5]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[6]).read_text(encoding="utf-8"))
module = Path(sys.argv[7]).read_text(encoding="utf-8")
shadow = Path(sys.argv[8]).read_text(encoding="utf-8")


def need(condition, message):
    if not condition:
        raise SystemExit(message)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-CALLER-ORIENTATION-ASSERTION-CLOSEOUT-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-READ-CALLER-ORIENTATION-DESIGN-STOP-001"
expected = [
    ("MapLoad", "map_load_scalar_i64_routes"),
    ("String", "string_indexof_scalar_i64_routes"),
    ("String", "string_lastindexof_scalar_i64_routes"),
    ("String", "string_contains_scalar_i64_routes"),
    ("Collection", "collection_map_entry_count_scalar_i64_routes"),
    ("Collection", "collection_array_slot_len_scalar_i64_routes"),
    ("Collection", "collection_string_len_scalar_i64_routes"),
    ("Collection", "collection_any_length_scalar_i64_routes"),
]
need(fixture.get("token") == token, "token drift")
need(token in card, "card token missing")
need(next_card in task_order, "next task pointer missing")
need(next_card_path.exists(), "next card file missing")
need(next_card in {row.get("token") for row in manifest.get("rows", [])}, "next card manifest missing")
rows = fixture.get("rows") or []
need([(row.get("surface"), row.get("policy_row_id")) for row in rows] == expected, "eight-row closeout drift")
need(all(row.get("consumer") == "assertion_only" for row in rows), "consumer kind drift")
for function, row_id in [
    ("assert_mapload_policy_row", "map_load_scalar_i64_routes"),
    ("assert_string_policy_row", "string_indexof_scalar_i64_routes"),
    ("assert_collection_policy_row", "collection_map_entry_count_scalar_i64_routes"),
]:
    need(f"fn {function}(policy_row_id: &str)" in module, f"assertion function missing: {function}")
    need(row_id in module, f"contract row missing: {row_id}")
need("assert_mapload_policy_row(policy.policy_row_id);" in shadow, "MapLoad call missing")
need("assert_string_policy_row(policy.policy_row_id);" in shadow, "String call missing")
need("assert_collection_policy_row(policy.policy_row_id);" in shadow, "Collection call missing")
for forbidden in [
    "route_selection_authority_switch", "caller_orientation_runtime_path",
    "backend_lowering_authority", "runtime_mutation_authority", "publication_execution",
    "write_caller_orientation_contract", "delete_hako_route_decision_authority_pilot",
    "scalar_known_wide_authority", "source_selfhost_claim",
]:
    need((fixture.get("claims") or {}).get(forbidden) == 0, f"claim drift: {forbidden}")
for claim in [
    "read_caller_orientation_assertion_closeout", "all_eight_read_rows_live_asserted",
    "mapload_live_assertion", "string_three_row_live_assertion",
    "collection_four_row_live_assertion", "assertion_only",
]:
    need((fixture.get("claims") or {}).get(claim) == 1, f"positive claim drift: {claim}")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-read-caller-orientation-assertion-closeout")
print("read_caller_orientation_assertion_closeout=1")
print("all_eight_read_rows_live_asserted=1")
print("runtime_dispatch=0")
print("route_selection_authority_switch=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY

cargo test -q caller_orientation

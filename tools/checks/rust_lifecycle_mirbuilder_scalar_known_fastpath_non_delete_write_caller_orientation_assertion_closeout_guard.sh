#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-non-delete-write-caller-orientation-assertion-closeout"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-non-delete-write-caller-orientation-assertion-closeout-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_non_delete_write_caller_orientation_assertion_closeout.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3439-MIRBUILDER-SCALAR-KNOWN-FASTPATH-NON-DELETE-WRITE-CALLER-ORIENTATION-ASSERTION-CLOSEOUT-001.md"
NEXT_CARD="$ROOT/docs/development/current/main/phases/phase-296x/3440-MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-NON-DELETE-WRITE-CALLER-ORIENTATION-DESIGN-CONSULTATION-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
MODULE="$ROOT/src/mir/generic_method_route_plan/caller_orientation.rs"
SHADOW="$ROOT/src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$NEXT_CARD" "$TASK_ORDER" "$MANIFEST" "$MODULE" "$SHADOW"

python3 "$TOOL" --check
python3 - "$FIXTURE" "$CARD" "$NEXT_CARD" "$TASK_ORDER" "$MANIFEST" "$MODULE" "$SHADOW" <<'PY'
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

def need(condition, message):
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-NON-DELETE-WRITE-CALLER-ORIENTATION-ASSERTION-CLOSEOUT-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-NON-DELETE-WRITE-CALLER-ORIENTATION-DESIGN-CONSULTATION-001"
expected = [
    ("MapStoreI64", "map_store_i64_set_surface"),
    ("ArrayAppendAny", "array_append_any_push_surface"),
    ("MapStoreAny", "map_store_any_set_surface"),
]
need(fixture.get("token") == token, "token drift")
need(token in card, "card token missing")
need(next_card in task_order, "next task pointer missing")
need(next_card_path.exists(), "next card file missing")
need(next_card in {row.get("token") for row in manifest.get("rows", [])}, "next card manifest missing")
rows = fixture.get("rows") or []
need([(row.get("surface"), row.get("policy_row_id")) for row in rows] == expected, "three-row closeout drift")
need(all(row.get("consumer") == "assertion_only" for row in rows), "consumer kind drift")
for function, row_id, call in [
    ("assert_mapstore_i64_policy_row", "map_store_i64_set_surface", "assert_mapstore_i64_policy_row(policy.policy_row_id);"),
    ("assert_push_arrayappendany_policy_row", "array_append_any_push_surface", "assert_push_arrayappendany_policy_row(policy.policy_row_id);"),
    ("assert_mapstore_any_policy_row", "map_store_any_set_surface", "assert_mapstore_any_policy_row(policy.policy_row_id);"),
]:
    need(f"fn {function}(policy_row_id: &str)" in module, f"assertion function missing: {function}")
    need(row_id in module, f"contract row missing: {row_id}")
    need(call in shadow, f"live call missing: {function}")
claims = fixture.get("claims") or {}
for key in [
    "non_delete_write_caller_orientation_assertion_closeout", "all_three_non_delete_write_rows_live_asserted",
    "mapstore_i64_live_assertion", "push_arrayappendany_live_assertion", "mapstore_any_live_assertion", "assertion_only",
]:
    need(claims.get(key) == 1, f"positive claim drift: {key}")
for key in [
    "caller_orientation_runtime_path", "caller_runtime_dispatch_authority", "route_selection_authority_switch",
    "hako_runtime_route_authority", "scalar_known_hako_runtime_route_authority", "backend_lowering_authority",
    "write_mutation_authority", "runtime_mutation_authority", "publication_execution",
    "delete_hako_route_decision_authority_pilot", "write_wide_authority", "scalar_known_wide_authority",
    "new_backend_route", "new_abi", "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"non-claim drift: {key}")
print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-non-delete-write-caller-orientation-assertion-closeout")
print("non_delete_write_caller_orientation_assertion_closeout=1")
print("all_three_non_delete_write_rows_live_asserted=1")
print("caller_orientation_runtime_path=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY

cargo test -q caller_orientation

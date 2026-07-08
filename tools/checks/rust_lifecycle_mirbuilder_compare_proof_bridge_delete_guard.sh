#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3347-MIRBUILDER-COMPARE-PROOF-BRIDGE-DELETE-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
BUILDER="$ROOT/src/mir/builder.rs"
COMPARISON="$ROOT/src/mir/builder/ops/comparison.rs"
INVENTORY="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-artifact-reachability-classification-inventory-v0.json"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-compare-proof-bridge-delete"

guard_require_command "$TAG" python3
guard_require_command "$TAG" cargo
guard_require_files "$TAG" "$CARD" "$STATE" "$TASK_ORDER" "$BUILDER" "$COMPARISON" "$INVENTORY"

python3 - "$ROOT" "$CARD" "$STATE" "$TASK_ORDER" "$BUILDER" "$COMPARISON" "$INVENTORY" <<'PY'
import json
import sys
from pathlib import Path
import tomllib

root, card_path, state_path, task_order_path, builder_path, comparison_path, inventory_path = map(Path, sys.argv[1:])
card = card_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
builder = builder_path.read_text(encoding="utf-8")
comparison = comparison_path.read_text(encoding="utf-8")
inventory = json.loads(inventory_path.read_text(encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-COMPARE-PROOF-BRIDGE-DELETE-001"
next_card = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-SET-MAPSTORE-I64-001"
bridges = [
    "compare_branch_emission_bridge",
    "compare_localssa_finalize_compare_bridge",
    "compare_mir_compare_emission_bridge",
    "compare_rhs_symbolref_contract",
    "compare_rhs_symbolref_lookup_bridge",
    "compare_rhs_valueid_resolution_bridge",
]

need(state.get("latest_card") == token, "CURRENT_STATE latest drift")
need(state.get("current_blocker_token") == next_card, "CURRENT_STATE blocker drift")
need(token in card, "card missing token")
need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

for module in bridges:
    path = root / "src/mir/builder" / f"{module}.rs"
    need(not path.exists(), f"deleted bridge file still exists: {path.relative_to(root)}")
    need(f"mod {module};" not in builder, f"builder.rs still declares {module}")
    need(module not in comparison, f"live comparison path references {module}")

for needle in [
    "ssa::local::finalize_compare(self, &mut lhs2, &mut rhs2)",
    "emission::compare::emit_to(self, dst, op, lhs2, rhs2)",
    "pub(in crate::mir::builder) fn build_comparison_op",
]:
    need(needle in comparison, f"live compare path missing token: {needle}")

summary = inventory.get("summary") or {}
need(summary.get("compare_proof_bridge_file_count") == 6, "inventory bridge count drift")
need(summary.get("compare_proof_bridge_deleted_file_count") == 6, "inventory deleted count drift")
need(summary.get("compare_proof_bridge_live_file_count") == 0, "inventory live bridge count drift")
need(summary.get("compare_proof_bridge_total_lines") == 0, "inventory line count drift")
need(summary.get("compare_proof_bridge_production_connected") == 0, "inventory production connection drift")

print("compare_proof_bridge_deleted=1")
print("compare_proof_bridge_deleted_file_count=6")
print("builder_mod_declarations_removed=1")
print("live_compare_path_preserved=1")
print("compare_lowering_behavior_changed=0")
print("source_selfhost_claim=0")
PY

cargo check -q --lib
cargo test -q --lib lower_value_ast_accepts_compare_value_and_emits_compare
cargo test -q --lib test_expr_lowerer_literal_less_var_generates_compare

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-compare-proof-bridge-delete
token=MIRBUILDER-COMPARE-PROOF-BRIDGE-DELETE-001
compare_proof_bridge_deleted=1
compare_proof_bridge_deleted_file_count=6
builder_mod_declarations_removed=1
live_compare_path_preserved=1
compare_lowering_behavior_changed=0
route_selection_authority=0
hako_runtime_route_authority=0
hako_backend_lowering_authority=0
runtime_fallback=0
new_backend_route=0
new_abi=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-SET-MAPSTORE-I64-001
summary=ok
REPORT

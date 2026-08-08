#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s2"
STATE="$ROOT/src/parser/declarations/box_def/state.rs"
BODY="$ROOT/src/parser/declarations/box_def/body.rs"
SOURCE_AUTHORITY="$ROOT/src/parser/source_authority.rs"
AST_INVENTORY="$ROOT/crates/hakorune_frontend_ast/src/box_method_inventory/mod.rs"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$STATE" "$BODY" "$SOURCE_AUTHORITY" "$AST_INVENTORY"

python3 - "$STATE" "$BODY" "$SOURCE_AUTHORITY" "$AST_INVENTORY" <<'PY'
import sys
from pathlib import Path

state_path, body_path, authority_path, ast_inventory_path = map(Path, sys.argv[1:])
state = state_path.read_text(encoding="utf-8")
body = body_path.read_text(encoding="utf-8")
authority = authority_path.read_text(encoding="utf-8")
ast_inventory = ast_inventory_path.read_text(encoding="utf-8")

for needle in (
    "source_tx: OpenBoxMethodSourceTransactionV1",
    "try_merge_selected_gate(other.source_tx",
    "pub(crate) fn methods(&self)",
):
    if needle not in state:
        raise SystemExit(f"missing R6-S2 transaction owner contract: {needle}")

for needle in (
    "&mut state.source_tx",
    "parse_box_member_gate_block(p, &state.source_tx)",
    "parse_box_member_gate_group(p, gate_site, &state.source_tx)",
):
    if needle not in body:
        raise SystemExit(f"missing R6-S2 ordinary producer cutover: {needle}")

for needle in (
    "PreparedBoxMethodInventoryAppendV1",
    "commit_prepared_append",
    "try_commit_generated_batch_with_placements",
):
    if needle not in ast_inventory and needle not in authority:
        raise SystemExit(f"missing R6-S2 typed append/rebase bridge: {needle}")

for path in (state_path, body_path, authority_path, ast_inventory_path):
    if len(path.read_text(encoding="utf-8").splitlines()) >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path}")

parser_root = state_path.parents[2]
for path in parser_root.rglob("*.rs"):
    text = path.read_text(encoding="utf-8")
    for forbidden in (
        "method_source_member_ordinals",
        "record_new_methods_since",
        "try_merge_selected_gate(selected, &[u32]",
    ):
        if forbidden in text:
            raise SystemExit(f"R6-S2 legacy parallel owner remains: {path}: {forbidden}")

print("source_transaction_owner=1")
print("ordinary_direct_property_gate_cutover=1")
print("typed_append_rebase_bridge=1")
print("method_sidecars=0")
print("ast_parallel_ordinal_merge=0")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"

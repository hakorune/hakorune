#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s3b-b1"
PARSER="$ROOT/src/parser/mod.rs"
PREDICATE="$ROOT/src/parser/build_cfg/predicate.rs"
AUTHORITY="$ROOT/src/parser/source_authority.rs"
BOX="$ROOT/src/parser/declarations/box_def/mod.rs"
TASK="$ROOT/docs/development/current/main/design/parser-postpass-source-handoff-ssot.md"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$PARSER" "$PREDICATE" "$AUTHORITY" "$BOX" "$TASK"

python3 - "$PARSER" "$PREDICATE" "$AUTHORITY" "$BOX" "$TASK" <<'PY'
import sys
from pathlib import Path

parser_path, predicate_path, authority_path, box_path, task_path = map(Path, sys.argv[1:])
parser = parser_path.read_text(encoding="utf-8")
predicate = predicate_path.read_text(encoding="utf-8")
authority = authority_path.read_text(encoding="utf-8")
box = box_path.read_text(encoding="utf-8")
task = task_path.read_text(encoding="utf-8")

for needle in (
    "next_source_build_gate_id",
    "active_source_declaration_path",
    "issue_source_build_gate_id",
    "SourceBoxDeclarationPathV1::root",
):
    if needle not in parser:
        raise SystemExit(f"missing parser B1 transport: {needle}")

for needle in (
    "SourceBuildGateBranchV1",
    "SourceBoxPathCursorV1",
    "SourceBoxDeclarationPathV1",
    "SourceBuildGateIdV1",
    "open_with_path",
):
    if needle not in authority:
        raise SystemExit(f"missing parser-private B1 type/issuer: {needle}")

for needle in (
    "parse_build_gate_item_block(",
    "SourceBuildGateBranchV1::Then",
    "SourceBuildGateBranchV1::Else",
    "cursor.next_child()",
    "active_source_declaration_path.replace",
):
    if needle not in predicate:
        raise SystemExit(f"missing gate path/cursor transport: {needle}")

if "active_source_statement_ordinal()" in box:
    raise SystemExit("Box source transaction must use typed path, not ordinal fallback")
if "BuildGateSelectionReceiptV1" in (parser + predicate + authority + box):
    raise SystemExit("B1 must not open B2 selection receipt")
if "prune_selected" in (parser + predicate + authority + box):
    raise SystemExit("B1 must not open B2 source-session prune/rebase")

for needle in (
    "R6-S3B-B1",
    "parser-issued gate id",
    "B2",
    "no post-prune ordinal reconstruction",
):
    if needle not in task:
        raise SystemExit(f"missing B1 SSOT boundary: {needle}")

for path in (parser_path, predicate_path, authority_path, box_path):
    if len(path.read_text(encoding="utf-8").splitlines()) >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path}")

print("parser_gate_id_issuer=1")
print("branch_child_path_cursor=1")
print("box_transaction_path_transport=1")
print("b2_prune_rebase_closed=1")
print("no_ast_ordinal_fallback=1")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"

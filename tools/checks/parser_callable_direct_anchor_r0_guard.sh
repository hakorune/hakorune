#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="parser-callable-direct-anchor-r0"
ANCHOR="$ROOT/src/parser/callable_source_anchor.rs"
TESTS="$ROOT/src/parser/callable_source_anchor_tests.rs"
PENDING="$ROOT/src/parser/declarations/box_def/members/pending_method.rs"
BODY="$ROOT/src/parser/declarations/box_def/body.rs"
STATIC="$ROOT/src/parser/declarations/static_def/mod.rs"
FUNCTIONS="$ROOT/src/parser/items/functions.rs"
STATIC_ITEMS="$ROOT/src/parser/items/static_items.rs"
SOURCE_PATH="$ROOT/src/parser/source_path.rs"
SOURCE_AUTHORITY="$ROOT/src/parser/source_authority.rs"
PARSER_MOD="$ROOT/src/parser/mod.rs"
README="$ROOT/src/parser/README.md"
TASK="$ROOT/docs/development/current/main/investigations/dynamic-carrier-ingress-lifecycle-d0-design-task-2026-08-10.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$ANCHOR" "$TESTS" "$PENDING" "$BODY" "$STATIC" \
  "$FUNCTIONS" "$STATIC_ITEMS" "$SOURCE_PATH" "$SOURCE_AUTHORITY" \
  "$PARSER_MOD" "$README" "$TASK" "$INDEX"

python3 - "$ANCHOR" "$TESTS" "$PENDING" "$BODY" "$STATIC" "$FUNCTIONS" \
  "$STATIC_ITEMS" "$SOURCE_PATH" "$SOURCE_AUTHORITY" "$PARSER_MOD" \
  "$README" "$TASK" "$INDEX" <<'PY'
import re
import sys
from pathlib import Path

paths = list(map(Path, sys.argv[1:]))
(
    anchor_path, tests_path, pending_path, body_path, static_path,
    functions_path, static_items_path, source_path, authority_path,
    parser_mod_path, readme_path, task_path, index_path,
) = paths
texts = {path: path.read_text(encoding="utf-8") for path in paths}
anchor = texts[anchor_path]
tests = texts[tests_path]
pending = texts[pending_path]
body = texts[body_path]
static = texts[static_path]
functions = texts[functions_path]
static_items = texts[static_items_path]
source = texts[source_path]
authority = texts[authority_path]
parser_mod = texts[parser_mod_path]
readme = texts[readme_path]
task = texts[task_path]
index = texts[index_path]

for needle in (
    "CallableDeclarationAnchorV1",
    "PreparedDirectCallableSourceV1",
    "ParserCallableSourceSessionV1",
    "DuplicateAnchor",
    "DuplicatePath",
    "ForeignParser",
):
    if needle not in anchor:
        raise SystemExit(f"missing direct callable anchor contract: {needle}")

for needle in (
    "SourceProgramDeclarationPathV1",
    "SourceProgramCallablePathV1",
    "SourceProgramMemberGateStepV1",
):
    if needle not in source:
        raise SystemExit(f"missing Program-wide callable path: {needle}")

if "DirectExplicitMethodSinkV1" not in authority:
    raise SystemExit("missing same-sink direct explicit commit capability")
if authority.count("impl DirectExplicitMethodSinkV1 for") + static.count(
    "impl DirectExplicitMethodSinkV1 for"
) != 2:
    raise SystemExit("direct explicit sink must have exactly ordinary and static implementations")
if "commit_direct(" not in pending or "CommittedDirectExplicitMethodV1" not in pending:
    raise SystemExit("missing path-bearing explicit commit receipt")
if re.search(r"derive\([^)]*Clone[^)]*\)\s*pub\([^)]*\) struct CommittedDirectExplicitMethodV1", pending):
    raise SystemExit("path-bearing explicit commit receipt must remain non-Clone")
if "SourceProgramCallablePathV1," in anchor.split(
    "fn issue_committed_explicit_box_method", 1
)[1].split("impl", 1)[0]:
    raise SystemExit("Box anchor issuer must consume the commit receipt, not a sibling raw path")

for text, needle, label in (
    (functions, "issue_direct_free_function", "free function parser"),
    (static_items, "issue_direct_free_static_function", "free static parser"),
    (body, "issue_committed_instance_box_method", "instance method commit"),
    (static, "issue_committed_static_box_method", "static method commit"),
    (parser_mod, "ParserCallableSourceSessionV1::open", "parser session"),
):
    if needle not in text:
        raise SystemExit(f"{label} missing direct anchor issuance: {needle}")

combined = "\n".join((anchor, tests, static, body))
for forbidden in ('MainMethod', 'box_name == "Main"', 'diagnostic_name == "main"'):
    if forbidden in combined:
        raise SystemExit(f"by-name Main classification is forbidden: {forbidden}")

for needle in (
    "mixed_direct_source_keeps_five_rows_across_four_direct_kinds",
    "generated_property_does_not_enter_the_direct_anchor_session",
    "generated_delegate_does_not_add_a_direct_anchor_row",
    "top_level_gate_children_keep_both_written_paths_before_selection",
    "member_gate_children_keep_both_written_paths_before_selection",
    "nested_member_gate_keeps_the_full_written_branch_path",
    "foreign_parser_path_rejects_before_publication",
    "duplicate_anchor_and_duplicate_path_are_distinct_rejects",
    "equal_diagnostics_and_coordinates_in_foreign_sessions_never_recreate_anchor",
):
    if needle not in tests:
        raise SystemExit(f"missing direct anchor focused test: {needle}")

for document, label in ((readme, "parser README"), (task, "active task")):
    for needle in (
        "PARSER-CALLABLE-DIRECT-ANCHOR-R0",
        "DirectExplicitMethodSinkV1",
        "ordinary static Box method",
    ):
        if needle not in document:
            raise SystemExit(f"{label} missing landed boundary: {needle}")
if "Status: **closed**" not in task.split("#### `PARSER-CALLABLE-DIRECT-ANCHOR-R0`", 1)[1].split("#### `PARSER-CALLABLE-GATE-PROJECTION-R0`", 1)[0]:
    raise SystemExit("direct anchor row must be closed before pointer advancement")
if "parser_callable_direct_anchor_r0_guard.sh" not in index:
    raise SystemExit("check index must list direct callable anchor guard")

for path in (
    anchor_path, tests_path, pending_path, body_path, static_path,
    functions_path, static_items_path, source_path, authority_path, parser_mod_path,
):
    lines = len(texts[path].splitlines())
    if lines >= 760:
        raise SystemExit(f"parser source reached the 760-line split trigger: {path}: {lines}")

print("opaque_direct_anchors=1")
print("same_sink_explicit_commit=1")
print("written_gate_paths=1")
print("generated_rows_excluded=1")
print("by_name_main_classification=0")
print("source_files_below_760=1")
print("summary=ok")
PY

echo "[$TAG] ok"

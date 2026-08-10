#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="parser-callable-gate-projection-r0"
PROJECTION="$ROOT/src/parser/callable_gate_projection.rs"
TESTS="$ROOT/src/parser/callable_gate_projection_tests.rs"
OPEN="$ROOT/src/parser/postpass_open.rs"
ANCHOR="$ROOT/src/parser/callable_source_anchor.rs"
MODEL="$ROOT/src/parser/source_seal/model.rs"
GATE="$ROOT/src/parser/source_seal/gate_projection.rs"
FINALIZE="$ROOT/src/parser/source_seal/finalize.rs"
ENVELOPE="$ROOT/src/parser/postpass_envelope.rs"
AUTHORITY="$ROOT/src/parser/source_authority.rs"
SELECTED_GATE="$ROOT/src/parser/source_authority/selected_gate.rs"
PARSER_MOD="$ROOT/src/parser/mod.rs"
README="$ROOT/src/parser/README.md"
TASK="$ROOT/docs/development/current/main/investigations/dynamic-carrier-ingress-lifecycle-d0-design-task-2026-08-10.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
INDEX="$ROOT/docs/tools/check-scripts-index.md"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$PROJECTION" "$TESTS" "$OPEN" "$ANCHOR" \
  "$MODEL" "$GATE" "$FINALIZE" "$ENVELOPE" "$AUTHORITY" \
  "$SELECTED_GATE" "$PARSER_MOD" "$README" "$TASK" "$STATE" "$INDEX"

python3 - "$PROJECTION" "$TESTS" "$OPEN" "$ANCHOR" "$MODEL" "$GATE" \
  "$FINALIZE" "$ENVELOPE" "$AUTHORITY" "$SELECTED_GATE" "$PARSER_MOD" \
  "$README" "$TASK" "$STATE" "$INDEX" <<'PY'
import re
import sys
from pathlib import Path

paths = list(map(Path, sys.argv[1:]))
(
    projection_path, tests_path, open_path, anchor_path, model_path, gate_path,
    finalize_path, envelope_path, authority_path, selected_gate_path,
    parser_mod_path, readme_path, task_path, state_path, index_path,
) = paths
texts = {path: path.read_text(encoding="utf-8") for path in paths}
projection = texts[projection_path]
tests = texts[tests_path]
open_postpass = texts[open_path]
anchor = texts[anchor_path]
model = texts[model_path]
gate = texts[gate_path]
finalize = texts[finalize_path]
envelope = texts[envelope_path]
authority = texts[authority_path]
selected_gate = texts[selected_gate_path]
parser_mod = texts[parser_mod_path]
readme = texts[readme_path]
task = texts[task_path]
state = texts[state_path]
index = texts[index_path]

for needle in (
    "MemberGateSelectionReceiptV1",
    "issue_from_selected_path",
    "prune_direct_callable_rows",
    "selected_path == parent_path",
):
    if needle not in projection:
        raise SystemExit(f"missing exact member-gate projection contract: {needle}")

for forbidden in (
    "eval_build_predicate",
    "ASTNode",
    "diagnostic_name() ==",
    "fallback",
    "retry",
):
    if forbidden in projection:
        raise SystemExit(f"callable projection must not recreate selection authority: {forbidden}")

for needle in (
    "MemberGateSelectionReceiptV1::issue_from_selected_path",
    "selected.member_gate_selection_receipts",
    "self.member_gate_selection_receipts.extend(receipt)",
):
    if needle not in selected_gate:
        raise SystemExit(f"selected transaction must issue and move member receipts: {needle}")

if "callable_source_session: Option<" not in parser_mod:
    raise SystemExit("callable source session must be one-shot parser state")
for needle in ("callable_source_session", ".take()", "already moved into postpass"):
    if needle not in open_postpass:
        raise SystemExit(f"postpass open must consume the callable session once: {needle}")

for struct_name in ("CallableDeclarationAnchorV1", "PreparedDirectCallableSourceV1"):
    pattern = rf"derive\([^)]*Clone[^)]*\)\s*pub\(super\) struct {struct_name}"
    if re.search(pattern, anchor):
        raise SystemExit(f"{struct_name} must remain non-Clone")

for needle in (
    "direct_callable_rows: Vec<PreparedDirectCallableSourceV1>",
    "member_gate_selection_receipts: Box<[MemberGateSelectionReceiptV1]>",
):
    if needle not in model:
        raise SystemExit(f"source-aware postpass is missing callable projection state: {needle}")
if "prepare_prune(\n        self,\n        receipts: Vec<BuildGateSelectionReceiptV1>" not in gate:
    raise SystemExit("source prune must consume the session and top-level receipt set")
if ".prepare_prune(projection.receipts)" not in gate:
    raise SystemExit("top-level selection receipts must move into the atomic prune")

for text, label in ((finalize, "finalizer"), (envelope, "postpass envelope")):
    if "direct_callable_rows" not in text:
        raise SystemExit(f"{label} must retain selected callable rows")
if "from_compatibility" not in envelope or "direct_callable_rows" not in envelope:
    raise SystemExit("compatibility completion must retain selected callable rows privately")

for needle in (
    "selected_top_level_and_member_rows_are_pruned_in_one_transaction",
    "nested_member_selection_keeps_the_full_selected_path_only",
    "top_level_else_and_nested_top_level_leaf_are_selected_exactly",
    "inactive_outer_member_branch_does_not_demand_its_nested_receipt",
    "opening_postpass_twice_rejects_the_moved_callable_session",
    "compatibility_finish_retains_selected_callable_rows_privately",
):
    if needle not in tests:
        raise SystemExit(f"missing callable projection regression: {needle}")

for document, label in ((readme, "parser README"), (task, "active task")):
    if "PARSER-CALLABLE-GATE-PROJECTION-R0" not in document:
        raise SystemExit(f"{label} is missing the gate-projection receipt")
section = task.split("#### `PARSER-CALLABLE-GATE-PROJECTION-R0`", 1)[1].split(
    "#### `PARSER-CALLABLE-GENERATED-ANCHOR-R0`", 1
)[0]
if "Status: **closed**" not in section:
    raise SystemExit("gate-projection row must be closed before pointer advancement")
if 'current_execution_row = "PARSER-CALLABLE-GENERATED-ANCHOR-R0"' not in state:
    raise SystemExit("current pointer must advance to generated callable anchors")
if "parser_callable_gate_projection_r0_guard.sh" not in index:
    raise SystemExit("check index must list the callable gate projection guard")

for path in (
    projection_path, tests_path, open_path, anchor_path, model_path, gate_path,
    finalize_path, envelope_path, authority_path, selected_gate_path, parser_mod_path,
):
    lines = len(texts[path].splitlines())
    if lines >= 760:
        raise SystemExit(f"parser source reached the 760-line split trigger: {path}: {lines}")

print("one_shot_callable_session_move=1")
print("top_level_selection_receipt_reused=1")
print("member_selection_receipt_issued_at_merge=1")
print("predicate_re_evaluation=0")
print("ordinary_and_compatibility_retention=1")
print("source_files_below_760=1")
print("summary=ok")
PY

echo "[$TAG] ok"

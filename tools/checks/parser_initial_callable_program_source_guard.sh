#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="parser-initial-callable-program-source"
DIR="$ROOT/src/parser/initial_callable_program_source"
FINALIZE="$ROOT/src/parser/source_seal/finalize.rs"
ENVELOPE="$ROOT/src/parser/postpass_envelope.rs"
SLOTS="$ROOT/src/parser/build_cfg/program_item_slots.rs"
README="$ROOT/src/parser/README.md"
TASK="$ROOT/docs/development/current/main/investigations/dynamic-carrier-ingress-lifecycle-d0-design-task-2026-08-10.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
INDEX="$ROOT/docs/tools/check-scripts-index.md"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" \
  "$DIR/mod.rs" "$DIR/model.rs" "$DIR/issue.rs" "$DIR/syntax_loan.rs" "$DIR/tests.rs" \
  "$FINALIZE" "$ENVELOPE" "$SLOTS" "$README" "$TASK" "$STATE" "$INDEX"

python3 - "$DIR" "$FINALIZE" "$ENVELOPE" "$SLOTS" "$README" "$TASK" "$STATE" "$INDEX" <<'PY'
import re
import sys
from pathlib import Path

directory, finalize_path, envelope_path, slots_path, readme_path, task_path, state_path, index_path = map(Path, sys.argv[1:])
sources = {path: path.read_text(encoding="utf-8") for path in directory.glob("*.rs")}
model = sources[directory / "model.rs"]
issue = sources[directory / "issue.rs"]
loan = sources[directory / "syntax_loan.rs"]
tests = sources[directory / "tests.rs"]
finalize = finalize_path.read_text(encoding="utf-8")
envelope = envelope_path.read_text(encoding="utf-8")
slots = slots_path.read_text(encoding="utf-8")
readme = readme_path.read_text(encoding="utf-8")
task = task_path.read_text(encoding="utf-8")
state = state_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

for needle in (
    "struct VerifiedInitialCallableProgramSourceV1",
    "InitialCallableFinalSlotV1",
    "sources: Box<[PreparedCallableSourceV1]>",
    "slots: Box<[InitialCallableFinalSlotV1]>",
    "with_callable_syntax",
):
    if needle not in model:
        raise SystemExit(f"missing atomic initial callable product contract: {needle}")

if re.search(r"derive\([^)]*Clone[^)]*\)\s*pub\(in crate::parser\) struct VerifiedInitialCallableProgramSourceV1", model):
    raise SystemExit("verified initial callable Program source must remain non-Clone")
for forbidden in ("into_parts", "from_ast", "pub fn new"):
    if forbidden in model:
        raise SystemExit(f"verified initial callable product exposes a splitting/arbitrary API: {forbidden}")
if "for<'syntax> FnOnce(InitialCallableProgramSyntaxLoanV1<'syntax>)" not in model:
    raise SystemExit("callable syntax must be lent through a higher-ranked callback")

for needle in (
    "exact_final_slot",
    "exact_selected_path_for_method_site",
    "generated_inventory_placement",
    "GeneratedCallableOriginV1::Property",
    "GeneratedCallableOriginV1::Delegate",
    "expected_callable_slots",
    "same_slot_coverage",
    "DuplicateAnchor",
    "DuplicateFinalCallableSlot",
    "UnsupportedMethodProvenance",
):
    if needle not in issue:
        raise SystemExit(f"co-seal is missing exact coverage/identity handling: {needle}")
for forbidden in ("diagnostic_name()", "eval_build_predicate", "fallback", "retry", "crate::mir"):
    if forbidden in issue + loan + model:
        raise SystemExit(f"initial callable source recreated or widened authority: {forbidden}")

for needle in (
    "issue_initial_callable_program_source_v1",
    "InitialCallableProgramSource",
    "from_initial_compatibility",
):
    if needle not in finalize + envelope:
        raise SystemExit(f"sole parser finalizer is missing the atomic co-seal: {needle}")
if "projected_program_item_slots" not in finalize or "brand_matches" not in slots:
    raise SystemExit("source-aware final Program placement is not retained through finalization")

for name in (
    "co_seal_covers_mixed_direct_program_without_name_repair",
    "co_seal_uses_selected_top_level_and_member_gate_receipts",
    "co_seal_covers_generated_property_and_delegate_origins",
    "syntax_loan_is_repeatable_but_never_splits_the_program",
    "issuer_rejects_missing_rows_and_arbitrary_ast",
    "issuer_rejects_foreign_slots_and_compatibility_only_methods",
):
    if name not in tests:
        raise SystemExit(f"missing initial callable source regression: {name}")

for document, label in ((readme, "parser README"), (task, "active card")):
    if "PARSER-INITIAL-CALLABLE-SOURCE-COSEAL-I0" not in document:
        raise SystemExit(f"{label} is missing the co-seal receipt")
section = task.split("#### `PARSER-INITIAL-CALLABLE-SOURCE-COSEAL-I0`", 1)[1].split("### 3F.", 1)[0]
if "Status: **closed**" not in section:
    raise SystemExit("initial callable source row must be closed")
if 'current_execution_row = "PARSER-INITIAL-CALLABLE-SOURCE-COSEAL-I0"' in state:
    raise SystemExit("current pointer must advance beyond the closed initial co-seal row")
if "parser_initial_callable_program_source_guard.sh" not in index:
    raise SystemExit("check index must list the reusable initial callable source guard")

for path in list(sources) + [finalize_path, envelope_path, slots_path]:
    lines = len(path.read_text(encoding="utf-8").splitlines())
    if lines >= 760:
        raise SystemExit(f"parser source reached the 760-line split trigger: {path}: {lines}")

print("opaque_anchor_identity=1")
print("exact_full_callable_coverage=1")
print("callback_scoped_syntax_loan=1")
print("compatibility_origin_semantic_admission=0")
print("resolver_builder_home_recipe_authority=0")
print("source_files_below_760=1")
print("summary=ok")
PY

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s3b-d-i0"
DESIGN="$ROOT/docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-d-d0-design-task-2026-08-09.md"
TASK="$ROOT/docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-d-i0-implementation-task-2026-08-09.md"
SSOT="$ROOT/docs/development/current/main/design/parser-postpass-source-handoff-ssot.md"
README="$ROOT/src/parser/README.md"
REFERENCE="$ROOT/docs/reference/language/callable-contracts.md"
TASKMAP="$ROOT/docs/development/current/main/investigations/callable-contract-and-instance-call-implementation-task-map-2026-08-08.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
INDEX="$ROOT/docs/tools/check-scripts-index.md"
SEAL_MOD="$ROOT/src/parser/source_seal/mod.rs"
SEAL_MODEL="$ROOT/src/parser/source_seal/model.rs"
SEAL_GATE="$ROOT/src/parser/source_seal/gate_projection.rs"
SEAL_FINALIZE="$ROOT/src/parser/source_seal/finalize.rs"
FINALIZER="$ROOT/src/parser/source_seal_finalizer.rs"
RELATION="$ROOT/src/parser/delegate_source_relation.rs"
TESTS="$ROOT/src/parser/source_seal_delegate_tests.rs"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$DESIGN" "$TASK" "$SSOT" "$README" "$REFERENCE" "$TASKMAP" "$STATE" "$INDEX" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" "$FINALIZER" "$RELATION" "$TESTS"

python3 - "$DESIGN" "$TASK" "$SSOT" "$README" "$REFERENCE" "$TASKMAP" "$STATE" "$INDEX" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" "$FINALIZER" "$RELATION" "$TESTS" <<'PY'
import sys
from pathlib import Path

design, task, ssot, readme, reference, taskmap, state, index = [
    Path(p).read_text(encoding="utf-8") for p in sys.argv[1:9]
]
seal_paths = list(map(Path, sys.argv[9:13]))
source_seal = "\n".join(path.read_text(encoding="utf-8") for path in seal_paths)
finalizer, relation, tests = [Path(p).read_text(encoding="utf-8") for p in sys.argv[13:16]]

if "Status: accepted design boundary; D-I0 implementation closed" not in design:
    raise SystemExit("D0 design must record the closed D-I0 boundary")
if "Status: active bounded implementation" not in task and "Status: closed implementation receipt" not in task:
    raise SystemExit("D-I0 task is neither active nor closed")
for document, label in ((ssot, "SSOT"), (readme, "README"), (reference, "reference"), (taskmap, "task map")):
    for needle in ("ParserBoxSourceSealV1", "D-I0", "final-seal"):
        if needle not in document:
            raise SystemExit(f"{label} missing D-I0 receipt: {needle}")
for needle in (
    "finalizer-owned relation coverage plan",
    "GeneratedDelegateSourceRelationV1",
    "generated_inventory_placement",
    "same-brand",
    "sole non-Clone `ParserBoxSourceSealV1`",
    "no AST/name/ordinal",
    "no partial",
    "no fallback",
):
    if needle not in task:
        raise SystemExit(f"D-I0 task missing contract: {needle}")
for needle in (
    "GeneratedDelegateCoverageErrorV1",
    "validate_generated_delegate_coverage",
    "generated_delegate_source_relations: self.generated_delegate_source_relations",
    "pub(in crate::parser) fn generated_delegate_source_relations",
):
    if needle not in source_seal + finalizer:
        raise SystemExit(f"D-I0 final-seal implementation missing: {needle}")
for needle in (
    "relation key",
    "placement",
    "orphan",
    "duplicate",
    "generated_delegate_source_relations()",
):
    if needle not in tests:
        raise SystemExit(f"D-I0 focused tests missing: {needle}")
for forbidden in ("AST-only", "name-based", "inventory ordinal reconstruction", "CallSlot", "ValueId"):
    if forbidden in finalizer:
        raise SystemExit(f"finalizer contains forbidden later authority: {forbidden}")
if not any(
    token in state
    for token in (
        'current_execution_row = "FRONTEND-PARSED-BOX-SOURCE-SEAL-R6-S3B-D-I0"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-CUTOVER-D0"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-S0"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-I0-A"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-I0-B"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-I0-C"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-I0-C-S0"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-I0-C-PROJECTION-D0"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-I0-C-PROJECTION-I0"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-D0"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-RETIRE-S0"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-NOELSE-RECEIPT-I0"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-CLOSEOUT-D0"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-GUARD-CLEANUP-S0"',
    )
):
    raise SystemExit("CURRENT_STATE is neither on D-I0 nor its explicit postpass successor")
if "frontend_parsed_box_source_seal_r6_s3b_d_i0_guard.sh" not in index:
    raise SystemExit("check index must list the D-I0 guard")
for path in (*seal_paths, Path("src/parser/source_seal_finalizer.rs"), Path("src/parser/delegate_source_relation.rs")):
    resolved = path if path.is_absolute() else Path.cwd() / path
    lines = resolved.read_text(encoding="utf-8").splitlines()
    if len(lines) >= 800:
        raise SystemExit(f"{path} exceeds the 800-line boundary")

print("d_i0_receipt=1")
print("final_seal_relation_coverage=1")
print("no_later_authority=1")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s3b-b3"
SEAL="$ROOT/src/parser/source_seal.rs"
FINALIZER="$ROOT/src/parser/source_seal_finalizer.rs"
FINALIZER_TESTS="$ROOT/src/parser/source_seal_finalizer_tests.rs"
DELEGATE_TESTS="$ROOT/src/parser/source_seal_delegate_tests.rs"
TASK="$ROOT/docs/development/current/main/design/parser-postpass-source-handoff-ssot.md"
REFERENCE="$ROOT/docs/reference/language/callable-contracts.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$SEAL" "$FINALIZER" "$FINALIZER_TESTS" "$DELEGATE_TESTS" "$TASK" "$REFERENCE" "$INDEX"

python3 - "$SEAL" "$FINALIZER" "$FINALIZER_TESTS" "$DELEGATE_TESTS" "$TASK" "$REFERENCE" "$INDEX" <<'PY'
import sys
from pathlib import Path

seal_path, finalizer_path, finalizer_tests_path, delegate_tests_path, task_path, reference_path, index_path = map(Path, sys.argv[1:])
seal = seal_path.read_text(encoding="utf-8")
finalizer = finalizer_path.read_text(encoding="utf-8")
finalizer_tests = finalizer_tests_path.read_text(encoding="utf-8")
delegate_tests = delegate_tests_path.read_text(encoding="utf-8")
task = task_path.read_text(encoding="utf-8")
reference = reference_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

for needle in (
    "final_box_paths",
    "SourceBoxDeclarationPathV1",
):
    if needle not in seal:
        raise SystemExit(f"missing parser-private final Box path transport: {needle}")

for needle in (
    "FinalizerCoveragePlanV1",
    "prepared_to_final",
    "FinalAstBoxPathCoverageMismatch",
    "DuplicateFinalAstBoxPath",
    "PreparedBoxPathMissing",
    "ForeignFinalAstBoxPath",
    "final_box_paths",
):
    if needle not in seal:
        raise SystemExit(f"missing B3 finalizer boundary: {needle}")

for needle in (
    "r6_s3b_d_i0_final_seal_retains_complete_delegate_relation_rows",
    "r6_s3b_c_i0_zero_delegate_program_is_an_exact_noop",
):
    if needle not in delegate_tests:
        raise SystemExit(f"missing delegate isolation test: {needle}")

if "coverage.prepared_to_final[prepared_index]" not in seal:
    raise SystemExit("finalizer must use path coverage, not final AST positional order")
if "zip(final_inventories" in seal:
    raise SystemExit("finalizer must not use positional final-inventory zip")
if "inventory: self.inventory" not in seal:
    raise SystemExit("generated delegate suffix must remain outside the source seal")

for needle in (
    "R6-S3B-B3-I0",
    "FinalizerCoveragePlanV1",
):
    if needle not in task or needle not in reference:
        raise SystemExit(f"missing B3 landed documentation: {needle}")

for document, label in ((task, "task"), (reference, "reference")):
    if "resolver-visible" not in document or "source seal" not in document:
        raise SystemExit(f"{label} must record delegate isolation from the source seal")

if "no positional zip" not in task:
    raise SystemExit("B3 task must retain the no-positional-zip contract")
if "positional final" not in reference:
    raise SystemExit("B3 reference must record the path mapping over positional order")

if "frontend_parsed_box_source_seal_r6_s3b_b3_guard.sh" not in index:
    raise SystemExit("check index must list the B3 guard")

for path in (seal_path, finalizer_path, finalizer_tests_path, delegate_tests_path):
    if len(path.read_text(encoding="utf-8").splitlines()) >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path}")

print("parser_final_box_paths=1")
print("private_finalizer_coverage_plan=1")
print("no_positional_final_inventory_zip=1")
print("delegate_suffix_outside_source_seal=1")
print("landed_docs=1")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"

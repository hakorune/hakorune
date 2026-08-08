#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s3b-b3"
WALKER="$ROOT/src/parser/source_gate_prune.rs"
SEAL="$ROOT/src/parser/source_seal.rs"
TASK="$ROOT/docs/development/current/main/design/parser-postpass-source-handoff-ssot.md"
REFERENCE="$ROOT/docs/reference/language/callable-contracts.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$WALKER" "$SEAL" "$TASK" "$REFERENCE" "$INDEX"

python3 - "$WALKER" "$SEAL" "$TASK" "$REFERENCE" "$INDEX" <<'PY'
import sys
from pathlib import Path

walker_path, seal_path, task_path, reference_path, index_path = map(Path, sys.argv[1:])
walker = walker_path.read_text(encoding="utf-8")
seal = seal_path.read_text(encoding="utf-8")
task = task_path.read_text(encoding="utf-8")
reference = reference_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

for needle in (
    "GatePruneOutputV1",
    "final_box_paths",
    "SourceBoxDeclarationPathV1",
):
    if needle not in walker:
        raise SystemExit(f"missing parser-private final Box path transport: {needle}")

for needle in (
    "FinalizerCoveragePlanV1",
    "prepared_to_final",
    "FinalAstBoxPathCoverageMismatch",
    "DuplicateFinalAstBoxPath",
    "PreparedBoxPathMissing",
    "ForeignFinalAstBoxPath",
    "final_box_paths",
    "r6_s3b_b3_keeps_delegate_suffix_outside_source_seal",
):
    if needle not in seal:
        raise SystemExit(f"missing B3 finalizer boundary: {needle}")

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

for path in (walker_path, seal_path):
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

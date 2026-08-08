#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="parser-public-ast-postpass-final-noelse-i0"
DESIGN="$ROOT/docs/development/current/main/investigations/parser-public-ast-postpass-final-no-else-receipt-d0-design-task-2026-08-09.md"
IMPLEMENTATION="$ROOT/docs/development/current/main/investigations/parser-public-ast-postpass-final-no-else-receipt-i0-implementation-task-2026-08-09.md"
SSOT="$ROOT/docs/development/current/main/design/parser-postpass-source-handoff-ssot.md"
REFERENCE="$ROOT/docs/reference/language/build-conditional-gate.md"
README="$ROOT/src/parser/README.md"
TASKMAP="$ROOT/docs/development/current/main/investigations/callable-contract-and-instance-call-implementation-task-map-2026-08-08.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
SELECTION="$ROOT/src/parser/build_gate_selection.rs"
RECEIPT="$ROOT/src/parser/source_gate_receipt.rs"
PROJECTION="$ROOT/src/parser/build_cfg/projection.rs"
SEAL="$ROOT/src/parser/source_seal.rs"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$DESIGN" "$IMPLEMENTATION" "$SSOT" "$REFERENCE" "$README" "$TASKMAP" "$STATE" "$SELECTION" "$RECEIPT" "$PROJECTION" "$SEAL"

python3 - "$DESIGN" "$IMPLEMENTATION" "$SSOT" "$REFERENCE" "$README" "$TASKMAP" "$STATE" "$SELECTION" "$RECEIPT" "$PROJECTION" "$SEAL" <<'PY'
import sys
from pathlib import Path

paths = list(map(Path, sys.argv[1:]))
design, implementation, ssot, reference, readme, taskmap, state, selection, receipt, projection, seal = [
    path.read_text(encoding="utf-8") for path in paths
]

if "FINAL-NOELSE-RECEIPT-D0" not in design:
    raise SystemExit("NoElse D0 design is missing")
if "Status: closed implementation receipt" not in implementation:
    raise SystemExit("NoElse implementation receipt must be closed")
for text, label in ((ssot, "SSOT"), (reference, "reference"), (readme, "README"), (taskmap, "task map")):
    for needle in ("BuildGateSelectionOutcomeV1", "SourceBuildGateBranchV1", "NoElse"):
        if needle not in text:
            raise SystemExit(f"{label} missing NoElse contract: {needle}")
active = all(
    needle in state
    for needle in (
        'work_mode = "fast"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-NOELSE-RECEIPT-I0"',
    )
)
closed = (
    'work_mode = "design_stop"' in state
    and any(
        needle in state
        for needle in (
            'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-NOELSE-RECEIPT-D0"',
            'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-CLOSEOUT-D0"',
            'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-GUARD-CLEANUP-S0"',
        )
    )
)
cleanup_successor = all(
    needle in state
    for needle in (
        'work_mode = "fast"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-GUARD-CLEANUP-S0"',
    )
)
if not (active or closed or cleanup_successor):
    raise SystemExit("CURRENT_STATE missing NoElse active/closeout pointer")
if "BuildGateSelectionOutcomeV1" not in selection:
    raise SystemExit("semantic outcome owner is missing")
if "selected_branch: BuildGateSelectionOutcomeV1" not in receipt:
    raise SystemExit("receipt must own semantic selection outcome")
if "source BuildGate record cannot represent a no-else selection" in projection:
    raise SystemExit("projection still rejects NoElse")
if "selection_matches_path" not in seal:
    raise SystemExit("source seal must separate semantic outcome from path branch")
if "SourceBuildGateBranchV1::NoElse" in selection + receipt + projection + seal:
    raise SystemExit("NoElse must not enter path segment authority")
for path in (paths[-4], paths[-3], paths[-2], paths[-1]):
    if len(path.read_text(encoding="utf-8").splitlines()) >= 760:
        raise SystemExit(f"{path} reached the 760-line split trigger")
print("semantic_outcome_shared=1")
print("receipt_noelse=1")
print("path_then_else_only=1")
print("records_receipts_totality=1")
print("summary=ok")
PY

echo "[$TAG] ok"

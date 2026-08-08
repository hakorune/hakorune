#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="parser-public-ast-postpass-cutover-d0"
TASK="$ROOT/docs/development/current/main/investigations/parser-public-ast-postpass-cutover-d0-design-task-2026-08-09.md"
TASKMAP="$ROOT/docs/development/current/main/investigations/callable-contract-and-instance-call-implementation-task-map-2026-08-08.md"
SSOT="$ROOT/docs/development/current/main/design/parser-postpass-source-handoff-ssot.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
README="$ROOT/src/parser/README.md"
REFERENCE="$ROOT/docs/reference/language/callable-contracts.md"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TASK" "$TASKMAP" "$SSOT" "$STATE" "$README" "$REFERENCE"

python3 - "$TASK" "$TASKMAP" "$SSOT" "$STATE" "$README" "$REFERENCE" <<'PY'
import sys
from pathlib import Path

task, taskmap, ssot, state, readme, reference = [
    Path(p).read_text(encoding="utf-8") for p in sys.argv[1:]
]

if "Status: accepted design; implementation not opened" not in task:
    raise SystemExit("cutover D0 must remain design-only")
for document, label in (
    (taskmap, "task map"),
    (ssot, "SSOT"),
    (readme, "README"),
    (reference, "reference"),
):
    for needle in ("PARSER-PUBLIC-AST-POSTPASS-CUTOVER-D0/I0", "ordinary-Box"):
        if needle not in document:
            raise SystemExit(f"{label} missing cutover boundary: {needle}")
for needle in (
    "interface",
    "static",
    "record",
    "mixed",
    "fuel",
    "metadata",
    "explain-report",
    "no catch-and-fallback",
    "no fake seal",
):
    if needle not in task:
        raise SystemExit(f"cutover task missing contract: {needle}")
for needle in (
    "CompletedParserPostpassV1",
    "SourceSealedOrdinary",
    "AstOnlyCompatibility",
    "PreparedBuildGateDecisionSetV1",
    "NyashParser::parse",
    "PARSER-PUBLIC-AST-POSTPASS-S0",
    "PARSER-PUBLIC-AST-POSTPASS-I0-A",
    "PARSER-PUBLIC-AST-POSTPASS-I0-B",
    "PARSER-PUBLIC-AST-POSTPASS-I0-C",
    "PARSER-PUBLIC-AST-POSTPASS-FINAL",
    "NoSafeSlice",
    "no retry",
):
    if needle not in task:
        raise SystemExit(f"cutover task missing total-envelope contract: {needle}")
if not any(
    token in state
    for token in (
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-CUTOVER-D0"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-S0"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-I0-A"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-I0-B"',
    )
):
    raise SystemExit("CURRENT_STATE must point to the cutover design/S0/I0-A/I0-B boundary")
if not any(
    token in state
    for token in ('work_mode = "design_stop"', 'work_mode = "fast"')
):
    raise SystemExit("CURRENT_STATE must route the cutover design/S0 boundary")
print("cutover_d0_design=1")
print("no_broad_cutover_implementation=1")
print("cohort_parity_boundary=1")
print("summary=ok")
PY

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s3b-c-i0-d0"
DESIGN="$ROOT/docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-i0-d0-design-task-2026-08-09.md"
IMPLEMENTATION="$ROOT/docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-i0-implementation-task-2026-08-09.md"
SSOT="$ROOT/docs/development/current/main/design/parser-postpass-source-handoff-ssot.md"
README="$ROOT/src/parser/README.md"
REFERENCE="$ROOT/docs/reference/language/callable-contracts.md"
TASKMAP="$ROOT/docs/development/current/main/investigations/callable-contract-and-instance-call-implementation-task-map-2026-08-08.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$DESIGN" "$IMPLEMENTATION" "$SSOT" "$README" "$REFERENCE" "$TASKMAP" "$STATE"

python3 - "$DESIGN" "$IMPLEMENTATION" "$SSOT" "$README" "$REFERENCE" "$TASKMAP" "$STATE" <<'PY'
import sys
from pathlib import Path

design, implementation, ssot, readme, reference, taskmap, state = [
    Path(p).read_text(encoding="utf-8") for p in sys.argv[1:]
]

for needle in (
    "Status: accepted design; implementation not opened",
    "PreparedDelegatePostpassBatchV1",
    "borrowed descriptive target",
    "method declaration/signature view",
    "all hosts/exposes",
    "staging inventory",
    "GeneratedDelegateSourceRelationV1",
    "A zero-delegate",
    "NoSafeSlice",
    "Rejected",
    "Unresolved",
    "Declined",
    "Candidate",
    "no partial per-host commit",
    "same-session",
    "R6-S3B-D",
):
    if needle not in design:
        raise SystemExit(f"C-I0 design receipt missing: {needle}")

if "Status: planned; implementation not opened" not in implementation:
    raise SystemExit("C-I0 implementation must remain unopened")
for needle in (
    "GeneratedDelegateSourceRelationV1",
    "staged-vs-actual placement receipt equality",
    "ParserBoxSourceSealV1",
    "no fallback",
):
    if needle not in implementation:
        raise SystemExit(f"C-I0 implementation card missing stop/gate: {needle}")

for document, label in ((ssot, "SSOT"), (reference, "reference"), (readme, "README"), (taskmap, "task map")):
    for needle in ("R6-S3B-C-I0-D0", "PreparedDelegatePostpassBatchV1", "atomic"):
        if needle not in document:
            raise SystemExit(f"{label} missing C-I0 design receipt: {needle}")

for needle in (
    'work_mode = "design_stop"',
    'current_execution_row = "FRONTEND-PARSED-BOX-SOURCE-SEAL-R6-S3B-C-I0-D0"',
    'next_execution_card = "frontend-parsed-box-source-aware-delegate-r6-s3b-c-i0"',
    'current_blocker_token = "R6-S3B-C-I0-D0:',
):
    if needle not in state:
        raise SystemExit(f"CURRENT_STATE missing clean C-I0 stop: {needle}")

print("accepted_design=1")
print("all_host_expose_preflight=1")
print("staged_atomic_batch=1")
print("relation_persistence=1")
print("typed_failure_matrix=1")
print("implementation_closed=1")
print("summary=ok")
PY

echo "[$TAG] ok"

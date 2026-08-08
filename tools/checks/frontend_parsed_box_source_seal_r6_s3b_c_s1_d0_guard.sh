#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s3b-c-s1-d0"
DESIGN="$ROOT/docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-s1-d0-design-task-2026-08-09.md"
IMPLEMENTATION="$ROOT/docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-s1-implementation-task-2026-08-09.md"
SSOT="$ROOT/docs/development/current/main/design/parser-postpass-source-handoff-ssot.md"
REFERENCE="$ROOT/docs/reference/language/callable-contracts.md"
TASKMAP="$ROOT/docs/development/current/main/investigations/callable-contract-and-instance-call-implementation-task-map-2026-08-08.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
INDEX="$ROOT/docs/tools/check-scripts-index.md"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$DESIGN" "$IMPLEMENTATION" "$SSOT" "$REFERENCE" "$TASKMAP" "$STATE" "$INDEX"

python3 - "$DESIGN" "$IMPLEMENTATION" "$SSOT" "$REFERENCE" "$TASKMAP" "$STATE" "$INDEX" <<'PY'
import sys
from pathlib import Path

design, implementation, ssot, reference, taskmap, state, index = [
    Path(p).read_text(encoding="utf-8") for p in sys.argv[1:]
]

for needle in (
    "Status: accepted design; implementation not opened",
    "DelegateTargetIndexV1<'product>",
    "Vec<TargetBoxEntryV1>",
    "TargetMethodRef<'product>",
    "same-brand",
    "Candidate",
    "Declined",
    "Unresolved",
    "Rejected",
    "NoSafeSlice",
    "no final ParserBoxSourceSealV1 extension",
):
    if needle not in design:
        raise SystemExit(f"design missing C-S1 boundary: {needle}")

for document, label in ((ssot, "SSOT"), (reference, "reference")):
    for needle in ("R6-S3B-C-S1-D0", "DelegateTargetIndexV1", "TargetMethodRef", "resolver-visible"):
        if needle not in document:
            raise SystemExit(f"{label} missing C-S1 design receipt: {needle}")

for needle in (
    "Status: planned; implementation not opened",
    "same-brand",
    "GeneratedDelegateSourceRelation",
    "no final ParserBoxSourceSealV1 extension",
):
    if needle not in implementation:
        raise SystemExit(f"implementation task missing stop line: {needle}")

for needle in (
    "R6-S3B-C-S1-D0 (accepted design stop; implementation closed)",
    "frontend-parsed-box-source-aware-delegate-r6-s3b-c-s1-implementation-task-2026-08-09.md",
):
    if needle not in taskmap:
        raise SystemExit(f"task map missing C-S1 task pointer: {needle}")

for needle in (
    'work_mode = "design_stop"',
    'current_execution_row = "FRONTEND-PARSED-BOX-SOURCE-SEAL-R6-S3B-C-S1-D0"',
    'current_blocker_token = "R6-S3B-C-S1-D0:',
):
    if needle not in state:
        raise SystemExit(f"current state missing design stop: {needle}")

if "frontend_parsed_box_source_seal_r6_s3b_c_s1_d0_guard.sh" not in index:
    raise SystemExit("check index must list the C-S1-D0 guard")

for path in (Path("src/parser/source_authority.rs"), Path("src/parser/source_seal.rs")):
    if "DelegateTargetIndexV1" in (Path.cwd() / path).read_text(encoding="utf-8"):
        raise SystemExit(f"C-S1 implementation must remain closed: {path}")

print("accepted_design=1")
print("private_borrowed_index=1")
print("disposition_matrix=1")
print("implementation_closed=1")
print("current_design_stop=1")
print("landed_docs=1")
print("summary=ok")
PY

echo "[$TAG] ok"

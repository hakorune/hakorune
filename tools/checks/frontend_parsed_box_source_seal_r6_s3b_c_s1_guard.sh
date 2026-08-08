#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s3b-c-s1"
MODULE="$ROOT/src/parser/delegate_target_index.rs"
AUTHORITY="$ROOT/src/parser/source_authority.rs"
README="$ROOT/src/parser/README.md"
SSOT="$ROOT/docs/development/current/main/design/parser-postpass-source-handoff-ssot.md"
REFERENCE="$ROOT/docs/reference/language/callable-contracts.md"
TASK="$ROOT/docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-s1-implementation-task-2026-08-09.md"
NEXT="$ROOT/docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-i0-d0-design-task-2026-08-09.md"
NEXT_IMPL="$ROOT/docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-i0-implementation-task-2026-08-09.md"
TASKMAP="$ROOT/docs/development/current/main/investigations/callable-contract-and-instance-call-implementation-task-map-2026-08-08.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
INDEX="$ROOT/docs/tools/check-scripts-index.md"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$MODULE" "$AUTHORITY" "$README" "$SSOT" "$REFERENCE" "$TASK" "$NEXT" "$NEXT_IMPL" "$TASKMAP" "$STATE" "$INDEX"

python3 - "$MODULE" "$AUTHORITY" "$README" "$SSOT" "$REFERENCE" "$TASK" "$NEXT" "$NEXT_IMPL" "$TASKMAP" "$STATE" "$INDEX" <<'PY'
import sys
from pathlib import Path

module, authority, readme, ssot, reference, task, next_design, next_impl, taskmap, state, index = [
    Path(p).read_text(encoding="utf-8") for p in sys.argv[1:]
]

for needle in (
    "DelegateTargetIndexV1",
    "TargetMethodRefV1",
    "SourceBoxDeclarationPathV1",
    "MethodSourceRelationV1::Explicit",
    "DelegateTargetResolutionV1::Unresolved",
    "DelegateTargetResolutionV1::Rejected",
    "DelegateTargetResolutionV1::Declined",
    "c_s1_positive_target_is_exact_and_reusable",
    "c_s1_missing_field_is_unresolved_without_partial_target",
    "c_s1_missing_method_is_rejected_not_fallback",
    "c_s1_duplicate_target_name_rejects_index",
):
    if needle not in module:
        raise SystemExit(f"C-S1 module/test receipt missing: {needle}")

for forbidden in (
    "ParserBoxSourceSealV1::",
    "GeneratedDelegateSourceRelation",
    "CallSlot",
    "ValueId",
    "provider::",
    "runtime::",
):
    if forbidden in module:
        raise SystemExit(f"C-S1 module opened forbidden later authority: {forbidden}")

for document, label in ((ssot, "SSOT"), (reference, "reference"), (task, "task")):
    for needle in ("R6-S3B-C-S1", "implementation receipt", "C-I0"):
        if needle not in document:
            raise SystemExit(f"{label} missing C-S1 closeout: {needle}")

if "Status: accepted design; implementation closed" not in next_design:
    raise SystemExit("C-I0 design receipt must be accepted and closed")
if "Status: closed implementation receipt" not in next_impl:
    raise SystemExit("C-I0 implementation receipt must be closed")
for needle in (
    "R6-S3B-C-S1 (closed)",
    "frontend-parsed-box-source-aware-delegate-r6-s3b-c-i0-d0-design-task-2026-08-09.md",
    "frontend-parsed-box-source-aware-delegate-r6-s3b-c-i0-implementation-task-2026-08-09.md",
):
    if needle not in taskmap:
        raise SystemExit(f"task map missing next C-I0 boundary: {needle}")

closeout_state = all(
    needle in state
    for needle in (
        'work_mode = "closeout"',
        'current_execution_row = "FRONTEND-PARSED-BOX-SOURCE-SEAL-R6-S3B-C-I0"',
        'current_blocker_token = "R6-S3B-C-I0-CLOSEOUT:',
    )
)
advanced_design_stop = all(
    needle in state
    for needle in (
        'work_mode = "design_stop"',
        'current_execution_row = "FRONTEND-PARSED-BOX-SOURCE-SEAL-R6-S3B-D-D0"',
        'current_blocker_token = "R6-S3B-D-D0:',
    )
)
if not (closeout_state or advanced_design_stop):
    raise SystemExit("current state missing C-I0 closeout or the accepted next D0 stop")

if "frontend_parsed_box_source_seal_r6_s3b_c_s1_guard.sh" not in index:
    raise SystemExit("check index must list the C-S1 implementation guard")
if "C-S1 delegate target index" not in readme:
    raise SystemExit("parser README must record the C-S1 owner boundary")

for path in (Path("src/parser/delegate_target_index.rs"), Path("src/parser/source_authority.rs"), Path("src/parser/source_seal.rs")):
    if len((Path.cwd() / path).read_text(encoding="utf-8").splitlines()) >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path}")

print("private_target_index=1")
print("exact_path_relation=1")
print("focused_disposition_tests=1")
print("no_later_authority=1")
print("c_i0_design_receipt=1")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"

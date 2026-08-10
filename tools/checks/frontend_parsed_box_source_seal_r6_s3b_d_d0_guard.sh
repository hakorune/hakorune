#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s3b-d-d0"
DESIGN="$ROOT/docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-d-d0-design-task-2026-08-09.md"
IMPLEMENTATION="$ROOT/docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-d-i0-implementation-task-2026-08-09.md"
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
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$DESIGN" "$IMPLEMENTATION" "$SSOT" "$README" "$REFERENCE" "$TASKMAP" "$STATE" "$INDEX" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE"

python3 - "$DESIGN" "$IMPLEMENTATION" "$SSOT" "$README" "$REFERENCE" "$TASKMAP" "$STATE" "$INDEX" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" <<'PY'
import sys
from pathlib import Path

design, implementation, ssot, readme, reference, taskmap, state, index = [
    Path(p).read_text(encoding="utf-8") for p in sys.argv[1:9]
]
seal_paths = list(map(Path, sys.argv[9:13]))
seal = "\n".join(path.read_text(encoding="utf-8") for path in seal_paths)

for needle in (
    "Status: accepted design boundary; D-I0 implementation opened",
    "sole resolver-visible",
    "GeneratedDelegateSourceRelationV1",
    "finalizer-owned relation/placement coverage",
    "relation key",
    "placement receipt",
    "same-brand",
    "NoSafeSlice",
    "Rejected",
    "Unresolved",
    "Declined",
    "Candidate",
    "no AST/name",
    "no partial",
    "no resolver",
):
    if needle not in design:
        raise SystemExit(f"D0 design receipt missing: {needle}")

if not any(
    status in implementation
    for status in (
        "Status: planned implementation; not opened",
        "Status: active bounded implementation",
        "Status: closed implementation receipt",
    )
):
    raise SystemExit("D-I0 implementation status is invalid")
for needle in ("ParserBoxSourceSealV1", "old S3A", "NoSafeSlice", "same-slice"):
    if needle not in implementation:
        raise SystemExit(f"D-I0 task missing boundary: {needle}")

for document, label in ((ssot, "SSOT"), (readme, "README"), (reference, "reference"), (taskmap, "task map")):
    for needle in ("R6-S3B-D-D0", "ParserBoxSourceSealV1", "D-I0"):
        if needle not in document:
            raise SystemExit(f"{label} missing D0 receipt: {needle}")

active_implementation = all(
    needle in state
    for needle in (
        'work_mode = "fast"',
        'current_execution_row = "FRONTEND-PARSED-BOX-SOURCE-SEAL-R6-S3B-D-I0"',
        'current_blocker_token = "R6-S3B-D-I0:',
    )
)
if not active_implementation:
    raise SystemExit("CURRENT_STATE must point at the active D-I0 implementation")

if "frontend_parsed_box_source_seal_r6_s3b_d_d0_guard.sh" not in index:
    raise SystemExit("check index must list the D0 guard")

for path in (*seal_paths, Path("src/parser/delegate_batch.rs"), Path("src/parser/source_authority.rs")):
    resolved = path if path.is_absolute() else Path.cwd() / path
    lines = resolved.read_text(encoding="utf-8").splitlines()
    if len(lines) >= 800:
        raise SystemExit(f"{path} exceeds the 800-line boundary")

print("accepted_design=1")
print("sole_final_seal_owner=1")
print("complete_relation_coverage=1")
print("implementation_boundary=1")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s3b-c-i0"
MODULE="$ROOT/src/parser/delegate_batch.rs"
RELATION="$ROOT/src/parser/delegate_source_relation.rs"
TARGET="$ROOT/src/parser/delegate_target_index.rs"
SEAL="$ROOT/src/parser/source_seal.rs"
TASK="$ROOT/docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-i0-implementation-task-2026-08-09.md"
SSOT="$ROOT/docs/development/current/main/design/parser-postpass-source-handoff-ssot.md"
README="$ROOT/src/parser/README.md"
REFERENCE="$ROOT/docs/reference/language/callable-contracts.md"
TASKMAP="$ROOT/docs/development/current/main/investigations/callable-contract-and-instance-call-implementation-task-map-2026-08-08.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$MODULE" "$RELATION" "$TARGET" "$SEAL" "$TASK" "$SSOT" "$README" "$REFERENCE" "$STATE" "$TASKMAP" "$INDEX"

python3 - "$MODULE" "$RELATION" "$TARGET" "$SEAL" "$TASK" "$SSOT" "$README" "$REFERENCE" "$STATE" "$TASKMAP" "$INDEX" <<'PY'
import sys
from pathlib import Path

module, relation, target, seal, task, ssot, readme, reference, state, taskmap, index = [
    Path(p).read_text(encoding="utf-8") for p in sys.argv[1:]
]

for needle in (
    "PreparedDelegatePostpassBatchV1",
    "prepare_all",
    "apply_staged_batches",
    "GeneratedDelegateSourceRelationV1",
    "validate_host_coverage",
    "staged-vs-actual placement receipt mismatch",
    "c_i0_preflights_all_hosts_before_any_ast_mutation",
    "c_i0_rejects_generated_name_collision_during_staging",
    "c_i0_rejects_staged_vs_actual_placement_mismatch",
    "c_i0_rejects_duplicate_parser_source_rows",
):
    if needle not in module:
        raise SystemExit(f"C-I0 implementation missing: {needle}")

for needle in (
    "ExistingTargetMethodSourceRefV1",
    "GeneratedDelegateSourceRelationV1",
    "generated_inventory_placement",
    "target_method_source_ref",
):
    if needle not in relation:
        raise SystemExit(f"relation transport missing: {needle}")

for needle in ("method_declaration", "method_source_relation"):
    if needle not in target:
        raise SystemExit(f"C-S1 descriptive target view missing: {needle}")

for forbidden in (
    "ParserBoxSourceSealV1::",
    "CallSlot",
    "ValueId",
    "provider::",
    "runtime::",
    "lower_delegate_exposes",
):
    if forbidden in module:
        raise SystemExit(f"C-I0 opened forbidden later authority: {forbidden}")

if "Status: closed implementation receipt" not in task:
    raise SystemExit("C-I0 implementation task is not closed")
for document, label in ((ssot, "SSOT"), (readme, "README"), (reference, "reference"), (taskmap, "task map")):
    for needle in ("R6-S3B-C-I0", "PreparedDelegatePostpassBatchV1", "implementation receipt"):
        if needle not in document:
            raise SystemExit(f"{label} missing C-I0 receipt: {needle}")

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
    raise SystemExit("CURRENT_STATE missing C-I0 closeout or the accepted next D0 stop")

if "frontend_parsed_box_source_seal_r6_s3b_c_i0_guard.sh" not in index:
    raise SystemExit("check index missing C-I0 implementation guard")

for path in (
    Path("src/parser/delegate_batch.rs"),
    Path("src/parser/delegate_source_relation.rs"),
    Path("src/parser/delegate_target_index.rs"),
    Path("src/parser/source_seal.rs"),
    Path("src/parser/source_authority.rs"),
):
    if len((Path.cwd() / path).read_text(encoding="utf-8").splitlines()) >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path}")

print("all_host_expose_preflight=1")
print("staged_atomic_commit=1")
print("relation_persistence=1")
print("focused_failure_matrix=1")
print("no_later_authority=1")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"

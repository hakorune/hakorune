#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-source-seal-r6-s3b-c-i0"
MODULE="$ROOT/src/parser/delegate_batch.rs"
RELATION="$ROOT/src/parser/delegate_source_relation.rs"
TARGET="$ROOT/src/parser/delegate_target_index.rs"
SEAL_MOD="$ROOT/src/parser/source_seal/mod.rs"
SEAL_MODEL="$ROOT/src/parser/source_seal/model.rs"
SEAL_GATE="$ROOT/src/parser/source_seal/gate_projection.rs"
SEAL_FINALIZE="$ROOT/src/parser/source_seal/finalize.rs"
TASK="$ROOT/docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-i0-implementation-task-2026-08-09.md"
SSOT="$ROOT/docs/development/current/main/design/parser-postpass-source-handoff-ssot.md"
README="$ROOT/src/parser/README.md"
REFERENCE="$ROOT/docs/reference/language/callable-contracts.md"
TASKMAP="$ROOT/docs/development/current/main/investigations/callable-contract-and-instance-call-implementation-task-map-2026-08-08.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$MODULE" "$RELATION" "$TARGET" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" "$TASK" "$SSOT" "$README" "$REFERENCE" "$STATE" "$TASKMAP" "$INDEX"

python3 - "$MODULE" "$RELATION" "$TARGET" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" "$TASK" "$SSOT" "$README" "$REFERENCE" "$STATE" "$TASKMAP" "$INDEX" <<'PY'
import sys
from pathlib import Path

module, relation, target = [Path(p).read_text(encoding="utf-8") for p in sys.argv[1:4]]
seal_paths = list(map(Path, sys.argv[4:8]))
seal = "\n".join(path.read_text(encoding="utf-8") for path in seal_paths)
task, ssot, readme, reference, state, taskmap, index = [
    Path(p).read_text(encoding="utf-8") for p in sys.argv[8:15]
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
active_implementation = all(
    needle in state
    for needle in (
        'work_mode = "fast"',
        'current_execution_row = "FRONTEND-PARSED-BOX-SOURCE-SEAL-R6-S3B-D-I0"',
        'current_blocker_token = "R6-S3B-D-I0:',
    )
)
if not (closeout_state or advanced_design_stop or active_implementation):
    raise SystemExit("CURRENT_STATE missing C-I0 closeout, D0 stop, or D-I0 implementation")

if "frontend_parsed_box_source_seal_r6_s3b_c_i0_guard.sh" not in index:
    raise SystemExit("check index missing C-I0 implementation guard")

for path in (
    Path("src/parser/delegate_batch.rs"),
    Path("src/parser/delegate_source_relation.rs"),
    Path("src/parser/delegate_target_index.rs"),
    *seal_paths,
    Path("src/parser/source_authority.rs"),
):
    resolved = path if path.is_absolute() else Path.cwd() / path
    if len(resolved.read_text(encoding="utf-8").splitlines()) >= 800:
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

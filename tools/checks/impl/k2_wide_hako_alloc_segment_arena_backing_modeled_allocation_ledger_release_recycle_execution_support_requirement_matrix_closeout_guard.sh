#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-requirement-matrix-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-requirement-matrix-closeout-ssot.md"
MATRIX_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-requirement-matrix-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-requirement-matrix-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_MANIFEST_INCLUDE="tools/checks/manifests/guard_rows/hako_alloc_closeout.toml"
CARD_328A="docs/development/current/main/phases/phase-293x/293x-943-MIMAP-328A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-SUPPORT-REQUIREMENT-MATRIX-INVENTORY.md"
CARD_329A="docs/development/current/main/phases/phase-293x/293x-944-MIMAP-329A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-SUPPORT-REQUIREMENT-MATRIX-DIAGNOSTICS.md"
CARD_330A="docs/development/current/main/phases/phase-293x/293x-945-MIMAP-330A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-SUPPORT-REQUIREMENT-MATRIX-CLOSEOUT.md"
CARD_331A="docs/development/current/main/phases/phase-293x/293x-946-MIMAP-331A-POST-RELEASE-RECYCLE-EXECUTION-SUPPORT-REQUIREMENT-MATRIX-CLOSEOUT-ROW-SELECTION.md"
GUARD_328A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_guard.sh"
GUARD_329A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-330A segment arena backing modeled allocation-ledger release/recycle execution support requirement matrix closeout"

guard_require_files "$TAG" "$SSOT" "$MATRIX_SSOT" "$DIAGNOSTICS_SSOT" "$TASKBOARD" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_MANIFEST_INCLUDE" "$CARD_328A" "$CARD_329A" "$CARD_330A" "$CARD_331A" "$GUARD_328A" "$GUARD_329A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$GUARD_328A" "$GUARD_329A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_328A" "MIMAP-328A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_329A" "MIMAP-329A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_330A" "MIMAP-330A closeout card must be landed"
guard_expect_in_file "$TAG" "MIMAP-331A" "$CARD_331A" "MIMAP-331A selection card must exist after closeout"
guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-330A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$MATRIX_SSOT" "MIMAP-328A matrix SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-329A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-requirement-matrix" "$SSOT" "closeout SSOT must name requirement matrix pack"
guard_expect_in_file "$TAG" "MIMAP-331A Post Release/Recycle Execution Support Requirement Matrix Closeout Row Selection" "$SSOT" "closeout SSOT must name next row"
guard_expect_in_file "$TAG" "id = \"MIMAP-328A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-328A"
guard_expect_in_file "$TAG" "id = \"MIMAP-329A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-329A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-requirement-matrix\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must assign requirement matrix closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-requirement-matrix-closeout\"" "$GUARD_MANIFEST_INCLUDE" "guard manifest must include MIMAP-330A closeout row"
guard_expect_fixed_in_file "$TAG" "$GUARD_328A" "$INDEX" "check index must list MIMAP-328A guard"
guard_expect_fixed_in_file "$TAG" "$GUARD_329A" "$INDEX" "check index must list MIMAP-329A guard"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-330A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-requirement-matrix --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "requirement matrix L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-328A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-328A"
guard_expect_in_file "$TAG" "MIMAP-329A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-329A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_328A" --level L2
bash "$GUARD_329A" --level L2

echo "[$TAG] ok"

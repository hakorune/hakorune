#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-unsupported-outcome-ledger-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-unsupported-outcome-ledger-closeout-ssot.md"
LEDGER_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-unsupported-outcome-ledger-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-unsupported-outcome-ledger-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_MANIFEST_INCLUDE="tools/checks/manifests/guard_rows/hako_alloc_closeout.toml"
CARD_320A="docs/development/current/main/phases/phase-293x/293x-935-MIMAP-320A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-UNSUPPORTED-OUTCOME-LEDGER.md"
CARD_321A="docs/development/current/main/phases/phase-293x/293x-936-MIMAP-321A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-UNSUPPORTED-OUTCOME-LEDGER-DIAGNOSTICS.md"
CARD_322A="docs/development/current/main/phases/phase-293x/293x-937-MIMAP-322A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-UNSUPPORTED-OUTCOME-LEDGER-CLOSEOUT.md"
CARD_323A="docs/development/current/main/phases/phase-293x/293x-938-MIMAP-323A-POST-RELEASE-RECYCLE-UNSUPPORTED-OUTCOME-LEDGER-CLOSEOUT-ROW-SELECTION.md"
GUARD_320A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_unsupported_outcome_ledger_guard.sh"
GUARD_321A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_unsupported_outcome_ledger_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_unsupported_outcome_ledger_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_unsupported_outcome_ledger_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-322A segment arena backing modeled allocation-ledger release/recycle execution unsupported outcome ledger closeout"

guard_require_files "$TAG" "$SSOT" "$LEDGER_SSOT" "$DIAGNOSTICS_SSOT" "$TASKBOARD" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_MANIFEST_INCLUDE" "$CARD_320A" "$CARD_321A" "$CARD_322A" "$CARD_323A" "$GUARD_320A" "$GUARD_321A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$GUARD_320A" "$GUARD_321A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_320A" "MIMAP-320A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_321A" "MIMAP-321A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_322A" "MIMAP-322A closeout card must be landed"
guard_expect_in_file "$TAG" "MIMAP-323A" "$CARD_323A" "MIMAP-323A selection card must exist after closeout"
guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-322A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$LEDGER_SSOT" "MIMAP-320A ledger SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-321A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-unsupported-outcome-ledger" "$SSOT" "closeout SSOT must name unsupported outcome pack"
guard_expect_in_file "$TAG" "MIMAP-323A post release/recycle unsupported outcome ledger closeout row selection" "$SSOT" "closeout SSOT must name next row"
guard_expect_in_file "$TAG" "id = \"MIMAP-320A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-320A"
guard_expect_in_file "$TAG" "id = \"MIMAP-321A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-321A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-unsupported-outcome-ledger\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must assign unsupported outcome closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-unsupported-outcome-ledger-closeout\"" "$GUARD_MANIFEST_INCLUDE" "guard manifest must include MIMAP-322A closeout row"
guard_expect_fixed_in_file "$TAG" "$GUARD_320A" "$INDEX" "check index must list MIMAP-320A guard"
guard_expect_fixed_in_file "$TAG" "$GUARD_321A" "$INDEX" "check index must list MIMAP-321A guard"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-322A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-unsupported-outcome-ledger --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "unsupported outcome ledger L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-320A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-320A"
guard_expect_in_file "$TAG" "MIMAP-321A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-321A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_320A" --level L2
bash "$GUARD_321A" --level L2

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-remaining-execution-prerequisite-ledger-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-remaining-execution-prerequisite-ledger-closeout-ssot.md"
LEDGER_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-remaining-execution-prerequisite-ledger-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_MANIFEST_INCLUDE="tools/checks/manifests/guard_rows/hako_alloc_closeout.toml"
CARD_342A="docs/development/current/main/phases/phase-293x/293x-957-MIMAP-342A-RELEASE-RECYCLE-REMAINING-EXECUTION-PREREQUISITE-LEDGER.md"
CARD_343A="docs/development/current/main/phases/phase-293x/293x-958-MIMAP-343A-REMAINING-EXECUTION-PREREQUISITE-LEDGER-CLOSEOUT.md"
GUARD_342A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_remaining_execution_prerequisite_ledger_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_remaining_execution_prerequisite_ledger_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_remaining_execution_prerequisite_ledger_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-343A remaining execution prerequisite ledger closeout"

guard_require_files "$TAG" "$SSOT" "$LEDGER_SSOT" "$TASKBOARD" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_MANIFEST_INCLUDE" "$CARD_342A" "$CARD_343A" "$GUARD_342A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$GUARD_342A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_342A" "MIMAP-342A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_343A" "MIMAP-343A closeout card must be landed after acceptance"
guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-343A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$LEDGER_SSOT" "MIMAP-342A ledger SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger-release-recycle-remaining-execution-prerequisite-ledger" "$SSOT" "closeout SSOT must name remaining prerequisite ledger pack"
guard_expect_in_file "$TAG" "MIMAP-344A No-Escape Pointer Residence Pilot" "$SSOT" "closeout SSOT must name next row"
guard_expect_in_file "$TAG" "id = \"MIMAP-342A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-342A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger-release-recycle-remaining-execution-prerequisite-ledger\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must assign remaining prerequisite ledger closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-remaining-execution-prerequisite-ledger-closeout\"" "$GUARD_MANIFEST_INCLUDE" "guard manifest must include MIMAP-343A closeout row"
guard_expect_fixed_in_file "$TAG" "$GUARD_342A" "$INDEX" "check index must list MIMAP-342A guard"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-343A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-recycle-remaining-execution-prerequisite-ledger --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "remaining execution prerequisite ledger L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-342A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-342A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_342A" --level L2

echo "[$TAG] ok"

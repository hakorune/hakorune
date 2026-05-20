#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite-closeout-ssot.md"
PREREQ_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_MANIFEST_INCLUDE="tools/checks/manifests/guard_rows/hako_alloc_closeout.toml"
CARD_336A="docs/development/current/main/phases/phase-293x/293x-951-MIMAP-336A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-POINTER-RESIDENCE-PREREQUISITE-INVENTORY.md"
CARD_337A="docs/development/current/main/phases/phase-293x/293x-952-MIMAP-337A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-POINTER-RESIDENCE-PREREQUISITE-DIAGNOSTICS.md"
CARD_338A="docs/development/current/main/phases/phase-293x/293x-953-MIMAP-338A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-POINTER-RESIDENCE-PREREQUISITE-CLOSEOUT.md"
CARD_339A="docs/development/current/main/phases/phase-293x/293x-954-MIMAP-339A-POST-RELEASE-RECYCLE-POINTER-RESIDENCE-PREREQUISITE-CLOSEOUT-ROW-SELECTION.md"
GUARD_336A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_residence_prerequisite_guard.sh"
GUARD_337A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_residence_prerequisite_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_residence_prerequisite_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_residence_prerequisite_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-338A segment arena backing modeled allocation-ledger release/recycle pointer residence prerequisite closeout"

guard_require_files "$TAG" "$SSOT" "$PREREQ_SSOT" "$DIAGNOSTICS_SSOT" "$TASKBOARD" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_MANIFEST_INCLUDE" "$CARD_336A" "$CARD_337A" "$CARD_338A" "$CARD_339A" "$GUARD_336A" "$GUARD_337A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$GUARD_336A" "$GUARD_337A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_336A" "MIMAP-336A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_337A" "MIMAP-337A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_338A" "MIMAP-338A closeout card must be landed"
guard_expect_in_file "$TAG" "MIMAP-339A" "$CARD_339A" "MIMAP-339A selection card must exist after closeout"
guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-338A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$PREREQ_SSOT" "MIMAP-336A prerequisite SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-337A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite" "$SSOT" "closeout SSOT must name pointer residence prerequisite pack"
guard_expect_in_file "$TAG" "MIMAP-339A Post Release/Recycle Pointer Residence Prerequisite Closeout Row Selection" "$SSOT" "closeout SSOT must name next row"
guard_expect_in_file "$TAG" "id = \"MIMAP-336A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-336A"
guard_expect_in_file "$TAG" "id = \"MIMAP-337A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-337A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must assign pointer residence prerequisite closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite-closeout\"" "$GUARD_MANIFEST_INCLUDE" "guard manifest must include MIMAP-338A closeout row"
guard_expect_fixed_in_file "$TAG" "$GUARD_336A" "$INDEX" "check index must list MIMAP-336A guard"
guard_expect_fixed_in_file "$TAG" "$GUARD_337A" "$INDEX" "check index must list MIMAP-337A guard"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-338A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-recycle-pointer-residence-prerequisite --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "pointer residence prerequisite L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-336A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-336A"
guard_expect_in_file "$TAG" "MIMAP-337A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-337A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_336A" --level L2
bash "$GUARD_337A" --level L2

echo "[$TAG] ok"

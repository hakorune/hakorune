#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-generation-prerequisite-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-generation-prerequisite-closeout-ssot.md"
PREREQ_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-generation-prerequisite-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-generation-prerequisite-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_MANIFEST_INCLUDE="tools/checks/manifests/guard_rows/hako_alloc_closeout.toml"
CARD_332A="docs/development/current/main/phases/phase-293x/293x-947-MIMAP-332A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-LIFECYCLE-GENERATION-PREREQUISITE-INVENTORY.md"
CARD_333A="docs/development/current/main/phases/phase-293x/293x-948-MIMAP-333A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-LIFECYCLE-GENERATION-PREREQUISITE-DIAGNOSTICS.md"
CARD_334A="docs/development/current/main/phases/phase-293x/293x-949-MIMAP-334A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-LIFECYCLE-GENERATION-PREREQUISITE-CLOSEOUT.md"
CARD_335A="docs/development/current/main/phases/phase-293x/293x-950-MIMAP-335A-POST-RELEASE-RECYCLE-LIFECYCLE-GENERATION-PREREQUISITE-CLOSEOUT-ROW-SELECTION.md"
GUARD_332A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_generation_prerequisite_guard.sh"
GUARD_333A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_generation_prerequisite_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_generation_prerequisite_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_generation_prerequisite_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-334A segment arena backing modeled allocation-ledger release/recycle lifecycle generation prerequisite closeout"

guard_require_files "$TAG" "$SSOT" "$PREREQ_SSOT" "$DIAGNOSTICS_SSOT" "$TASKBOARD" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_MANIFEST_INCLUDE" "$CARD_332A" "$CARD_333A" "$CARD_334A" "$CARD_335A" "$GUARD_332A" "$GUARD_333A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$GUARD_332A" "$GUARD_333A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_332A" "MIMAP-332A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_333A" "MIMAP-333A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_334A" "MIMAP-334A closeout card must be landed"
guard_expect_in_file "$TAG" "MIMAP-335A" "$CARD_335A" "MIMAP-335A selection card must exist after closeout"
guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-334A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$PREREQ_SSOT" "MIMAP-332A prerequisite SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-333A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-generation-prerequisite" "$SSOT" "closeout SSOT must name lifecycle generation prerequisite pack"
guard_expect_in_file "$TAG" "MIMAP-335A Post Release/Recycle Lifecycle Generation Prerequisite Closeout Row Selection" "$SSOT" "closeout SSOT must name next row"
guard_expect_in_file "$TAG" "id = \"MIMAP-332A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-332A"
guard_expect_in_file "$TAG" "id = \"MIMAP-333A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-333A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-generation-prerequisite\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must assign lifecycle generation prerequisite closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-generation-prerequisite-closeout\"" "$GUARD_MANIFEST_INCLUDE" "guard manifest must include MIMAP-334A closeout row"
guard_expect_fixed_in_file "$TAG" "$GUARD_332A" "$INDEX" "check index must list MIMAP-332A guard"
guard_expect_fixed_in_file "$TAG" "$GUARD_333A" "$INDEX" "check index must list MIMAP-333A guard"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-334A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-generation-prerequisite --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "lifecycle generation prerequisite L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-332A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-332A"
guard_expect_in_file "$TAG" "MIMAP-333A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-333A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_332A" --level L2
bash "$GUARD_333A" --level L2

echo "[$TAG] ok"

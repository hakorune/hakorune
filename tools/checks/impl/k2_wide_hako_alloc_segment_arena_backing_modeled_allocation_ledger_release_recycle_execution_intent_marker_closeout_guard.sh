#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker-closeout-ssot.md"
INTENT_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_MANIFEST_INCLUDE="tools/checks/manifests/guard_rows/hako_alloc_closeout.toml"
CARD_316A="docs/development/current/main/phases/phase-293x/293x-931-MIMAP-316A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-INTENT-MARKER-PREFLIGHT.md"
CARD_317A="docs/development/current/main/phases/phase-293x/293x-932-MIMAP-317A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-INTENT-MARKER-DIAGNOSTICS.md"
CARD_318A="docs/development/current/main/phases/phase-293x/293x-933-MIMAP-318A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-INTENT-MARKER-CLOSEOUT.md"
CARD_319A="docs/development/current/main/phases/phase-293x/293x-934-MIMAP-319A-POST-RELEASE-RECYCLE-EXECUTION-INTENT-MARKER-CLOSEOUT-ROW-SELECTION.md"
GUARD_316A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_intent_marker_guard.sh"
GUARD_317A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_intent_marker_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_intent_marker_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_intent_marker_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-318A segment arena backing modeled allocation-ledger release/recycle execution intent marker closeout"

guard_require_files "$TAG" "$SSOT" "$INTENT_SSOT" "$DIAGNOSTICS_SSOT" "$TASKBOARD" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_MANIFEST_INCLUDE" "$CARD_316A" "$CARD_317A" "$CARD_318A" "$CARD_319A" "$GUARD_316A" "$GUARD_317A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$GUARD_316A" "$GUARD_317A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_316A" "MIMAP-316A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_317A" "MIMAP-317A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_318A" "MIMAP-318A closeout card must be landed"
guard_expect_in_file "$TAG" "MIMAP-319A" "$CARD_319A" "MIMAP-319A selection card must exist after closeout"
guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-318A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INTENT_SSOT" "MIMAP-316A intent marker SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-317A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker" "$SSOT" "closeout SSOT must name intent marker pack"
guard_expect_in_file "$TAG" "MIMAP-319A post release/recycle execution intent marker closeout row selection" "$SSOT" "closeout SSOT must name next row"
guard_expect_in_file "$TAG" "id = \"MIMAP-316A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-316A"
guard_expect_in_file "$TAG" "id = \"MIMAP-317A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-317A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must assign intent marker closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker-closeout\"" "$GUARD_MANIFEST_INCLUDE" "guard manifest must include MIMAP-318A closeout row"
guard_expect_fixed_in_file "$TAG" "$GUARD_316A" "$INDEX" "check index must list MIMAP-316A guard"
guard_expect_fixed_in_file "$TAG" "$GUARD_317A" "$INDEX" "check index must list MIMAP-317A guard"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-318A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "execution intent marker L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-316A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-316A"
guard_expect_in_file "$TAG" "MIMAP-317A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-317A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_316A" --level L2
bash "$GUARD_317A" --level L2

echo "[$TAG] ok"

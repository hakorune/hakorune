#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate-closeout-ssot.md"
GATE_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_MANIFEST_INCLUDE="tools/checks/manifests/guard_rows/hako_alloc_closeout.toml"
CARD_324A="docs/development/current/main/phases/phase-293x/293x-939-MIMAP-324A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-SUPPORT-GATE-INVENTORY.md"
CARD_325A="docs/development/current/main/phases/phase-293x/293x-940-MIMAP-325A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-SUPPORT-GATE-DIAGNOSTICS.md"
CARD_326A="docs/development/current/main/phases/phase-293x/293x-941-MIMAP-326A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-SUPPORT-GATE-CLOSEOUT.md"
CARD_327A="docs/development/current/main/phases/phase-293x/293x-942-MIMAP-327A-POST-RELEASE-RECYCLE-EXECUTION-SUPPORT-GATE-CLOSEOUT-ROW-SELECTION.md"
GUARD_324A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_guard.sh"
GUARD_325A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-326A segment arena backing modeled allocation-ledger release/recycle execution support gate closeout"

guard_require_files "$TAG" "$SSOT" "$GATE_SSOT" "$DIAGNOSTICS_SSOT" "$TASKBOARD" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$GUARD_MANIFEST_INCLUDE" "$CARD_324A" "$CARD_325A" "$CARD_326A" "$CARD_327A" "$GUARD_324A" "$GUARD_325A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"
guard_require_exec_files "$TAG" "$GUARD_324A" "$GUARD_325A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_324A" "MIMAP-324A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_325A" "MIMAP-325A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_326A" "MIMAP-326A closeout card must be landed"
guard_expect_in_file "$TAG" "MIMAP-327A" "$CARD_327A" "MIMAP-327A selection card must exist after closeout"
guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-326A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$GATE_SSOT" "MIMAP-324A support gate SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-325A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate" "$SSOT" "closeout SSOT must name support gate pack"
guard_expect_in_file "$TAG" "MIMAP-327A Post Release/Recycle Execution Support Gate Closeout Row Selection" "$SSOT" "closeout SSOT must name next row"
guard_expect_in_file "$TAG" "id = \"MIMAP-324A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-324A"
guard_expect_in_file "$TAG" "id = \"MIMAP-325A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-325A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must assign support gate closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate-closeout\"" "$GUARD_MANIFEST_INCLUDE" "guard manifest must include MIMAP-326A closeout row"
guard_expect_fixed_in_file "$TAG" "$GUARD_324A" "$INDEX" "check index must list MIMAP-324A guard"
guard_expect_fixed_in_file "$TAG" "$GUARD_325A" "$INDEX" "check index must list MIMAP-325A guard"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-326A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-support-gate --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "support gate L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-324A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-324A"
guard_expect_in_file "$TAG" "MIMAP-325A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-325A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_324A" --level L2
bash "$GUARD_325A" --level L2

echo "[$TAG] ok"

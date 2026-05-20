#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-closeout-ssot.md"
SUMMARY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
GUARD_MANIFEST_INCLUDE="tools/checks/manifests/guard_rows/hako_alloc_closeout.toml"
CARD_308A="docs/development/current/main/phases/phase-293x/293x-911-MIMAP-308A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-APPLIED-STATE-SUMMARY-INVENTORY.md"
CARD_309A="docs/development/current/main/phases/phase-293x/293x-924-MIMAP-309A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-APPLIED-STATE-SUMMARY-DIAGNOSTICS.md"
CARD_310A="docs/development/current/main/phases/phase-293x/293x-925-MIMAP-310A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-APPLIED-STATE-SUMMARY-CLOSEOUT.md"
CARD_311A="docs/development/current/main/phases/phase-293x/293x-926-MIMAP-311A-POST-RELEASE-RECYCLE-APPLIED-STATE-SUMMARY-CLOSEOUT-ROW-SELECTION.md"
GUARD_308A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_applied_state_summary_guard.sh"
GUARD_309A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_applied_state_summary_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_applied_state_summary_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_applied_state_summary_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-310A segment arena backing modeled allocation-ledger release/recycle applied-state summary closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$SUMMARY_SSOT" \
  "$DIAGNOSTICS_SSOT" \
  "$TASKBOARD" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$PROOF_MANIFEST_INCLUDE" \
  "$GUARD_MANIFEST" \
  "$GUARD_MANIFEST_INCLUDE" \
  "$CARD_308A" \
  "$CARD_309A" \
  "$CARD_310A" \
  "$CARD_311A" \
  "$GUARD_308A" \
  "$GUARD_309A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_308A" "$GUARD_309A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_308A" "MIMAP-308A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_309A" "MIMAP-309A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_310A" "MIMAP-310A closeout card must be landed"
guard_expect_in_file "$TAG" "MIMAP-311A" "$CARD_311A" "MIMAP-311A selection card must exist after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-310A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$SUMMARY_SSOT" "MIMAP-308A summary SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-309A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary" "$SSOT" "closeout SSOT must name applied-state summary pack"
guard_expect_in_file "$TAG" "MIMAP-311A post release/recycle applied-state summary closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-310A" "$TASKBOARD" "taskboard must name applied-state summary closeout row"
guard_expect_in_file "$TAG" "MIMAP-311A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-308A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-308A"
guard_expect_in_file "$TAG" "id = \"MIMAP-309A\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must include MIMAP-309A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary\"" "$PROOF_MANIFEST_INCLUDE" "proof manifest must assign applied-state summary closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-closeout\"" "$GUARD_MANIFEST_INCLUDE" "guard manifest must include MIMAP-310A closeout row"
guard_expect_fixed_in_file "$TAG" "$GUARD_308A" "$INDEX" "check index must list MIMAP-308A guard"
guard_expect_fixed_in_file "$TAG" "$GUARD_309A" "$INDEX" "check index must list MIMAP-309A guard"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-310A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "applied-state summary L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-308A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-308A"
guard_expect_in_file "$TAG" "MIMAP-309A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-309A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_308A" --level L2
bash "$GUARD_309A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap310_applied_state_summary_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap310.mir.json"
exe_out="$tmp_dir/mimap310_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-applied-state-summary-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,0,1,0,1,98019005005,99019005005' "$run_log"
rg -F -q 'bytes=4096,4096,8192' "$run_log"
rg -F -q 'summary-counts=1,1,0,0,0,0,0' "$run_log"
rg -F -q 'owner=5,1,4,1,1,1,1,4' "$run_log"
rg -F -q 'rejected=0,1,0,2,1,3,2,0,4' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

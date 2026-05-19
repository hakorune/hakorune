#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-apply-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-apply-closeout-ssot.md"
INVENTORY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-apply-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-apply-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_288A="docs/development/current/main/phases/phase-293x/293x-891-MIMAP-288A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-APPLY-INVENTORY.md"
CARD_289A="docs/development/current/main/phases/phase-293x/293x-892-MIMAP-289A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-APPLY-DIAGNOSTICS.md"
CARD_290A="docs/development/current/main/phases/phase-293x/293x-893-MIMAP-290A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-APPLY-CLOSEOUT.md"
CARD_291A="docs/development/current/main/phases/phase-293x/293x-894-MIMAP-291A-POST-RELEASE-APPLY-CLOSEOUT-ROW-SELECTION.md"
GUARD_288A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_apply_guard.sh"
GUARD_289A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_apply_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_apply_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_apply_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-apply-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-290A segment arena backing modeled allocation-ledger release apply closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$INVENTORY_SSOT" \
  "$DIAGNOSTICS_SSOT" \
  "$TASKBOARD" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_288A" \
  "$CARD_289A" \
  "$CARD_290A" \
  "$CARD_291A" \
  "$GUARD_288A" \
  "$GUARD_289A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_288A" "$GUARD_289A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_288A" "MIMAP-288A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_289A" "MIMAP-289A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_290A" "MIMAP-290A closeout card must be landed"
guard_expect_in_file "$TAG" "MIMAP-291A" "$CARD_291A" "MIMAP-291A selection card must exist after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-290A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INVENTORY_SSOT" "MIMAP-288A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-289A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack:" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger-release-apply" "$SSOT" "closeout SSOT must name release-apply pack"
guard_expect_in_file "$TAG" "MIMAP-291A post release-apply closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-290A" "$TASKBOARD" "taskboard must name release-apply closeout row"
guard_expect_in_file "$TAG" "MIMAP-291A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-288A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-288A"
guard_expect_in_file "$TAG" "id = \"MIMAP-289A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-289A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger-release-apply\"" "$PROOF_MANIFEST" "proof manifest must assign release-apply closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-apply-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-290A closeout row"
guard_expect_in_file "$TAG" "$GUARD_288A" "$INDEX" "check index must list MIMAP-288A guard"
guard_expect_in_file "$TAG" "$GUARD_289A" "$INDEX" "check index must list MIMAP-289A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-290A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-apply --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "release-apply L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-288A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-288A"
guard_expect_in_file "$TAG" "MIMAP-289A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-289A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_288A" --level L2
bash "$GUARD_289A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap290_release_apply_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap290.mir.json"
exe_out="$tmp_dir/mimap290_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-apply-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,0,6,1,5,184,18,96018005005' "$run_log"
rg -F -q 'seen=1,1,1,1,1' "$run_log"
rg -F -q 'apply=93018005005,94018005005,95018005005,96018005005,4096,4096,8192' "$run_log"
rg -F -q 'present=1,1,1' "$run_log"
rg -F -q 'owner=2,1,1,1,1' "$run_log"
rg -F -q 'empty=0,1,0' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

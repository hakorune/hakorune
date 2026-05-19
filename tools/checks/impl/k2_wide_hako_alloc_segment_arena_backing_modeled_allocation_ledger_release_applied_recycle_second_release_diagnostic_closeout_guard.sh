#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-second-release-diagnostic-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-second-release-diagnostic-closeout-ssot.md"
RECYCLE_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-ssot.md"
SECOND_DIAGNOSTIC_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-second-release-diagnostic-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_292A="docs/development/current/main/phases/phase-293x/293x-895-MIMAP-292A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-APPLIED-RECYCLE-INVENTORY.md"
CARD_296A="docs/development/current/main/phases/phase-293x/293x-899-MIMAP-296A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-APPLIED-RECYCLE-SECOND-RELEASE-DIAGNOSTIC.md"
CARD_297A="docs/development/current/main/phases/phase-293x/293x-900-MIMAP-297A-POST-RELEASE-APPLIED-RECYCLE-SECOND-RELEASE-DIAGNOSTIC-ROW-SELECTION.md"
CARD_298A="docs/development/current/main/phases/phase-293x/293x-901-MIMAP-298A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-APPLIED-RECYCLE-SECOND-RELEASE-DIAGNOSTIC-CLOSEOUT.md"
CARD_299A="docs/development/current/main/phases/phase-293x/293x-902-MIMAP-299A-POST-RELEASE-APPLIED-RECYCLE-SECOND-RELEASE-DIAGNOSTIC-CLOSEOUT-ROW-SELECTION.md"
GUARD_296A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_second_release_diagnostic_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_second_release_diagnostic_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_second_release_diagnostic_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-second-release-diagnostic-proof/main.hako"

echo "[$TAG] checking MIMAP-298A segment arena backing modeled allocation-ledger release-applied recycle second-release diagnostic closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$RECYCLE_SSOT" \
  "$SECOND_DIAGNOSTIC_SSOT" \
  "$TASKBOARD" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_292A" \
  "$CARD_296A" \
  "$CARD_297A" \
  "$CARD_298A" \
  "$CARD_299A" \
  "$GUARD_296A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_296A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_292A" "MIMAP-292A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_296A" "MIMAP-296A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_297A" "MIMAP-297A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_298A" "MIMAP-298A closeout card must be landed"
guard_expect_in_file "$TAG" "MIMAP-299A" "$CARD_299A" "MIMAP-299A selection card must exist after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-298A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$RECYCLE_SSOT" "MIMAP-292A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$SECOND_DIAGNOSTIC_SSOT" "MIMAP-296A second-release diagnostic SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack:" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-second-release-diagnostic" "$SSOT" "closeout SSOT must name release-applied recycle pack"
guard_expect_in_file "$TAG" "MIMAP-299A post release-applied recycle second-release diagnostic closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-298A" "$TASKBOARD" "taskboard must name release-applied recycle second-release diagnostic closeout row"
guard_expect_in_file "$TAG" "MIMAP-299A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-296A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-296A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-second-release-diagnostic\"" "$PROOF_MANIFEST" "proof manifest must assign release-applied recycle second-release diagnostic closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-second-release-diagnostic-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-298A closeout row"
guard_expect_in_file "$TAG" "$GUARD_296A" "$INDEX" "check index must list MIMAP-296A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-298A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-second-release-diagnostic --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "release-applied recycle L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-296A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-296A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_296A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap298_release_applied_recycle_second_release_diagnostic_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap298.mir.json"
exe_out="$tmp_dir/mimap298_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-second-release-diagnostic-proof' "$run_log"
rg -F -q 'diag=1,0,1,4,1,1,1,0,190,19,97019005005' "$run_log"
rg -F -q 'recycle=1,0,1,96019005005,97019005005,4096,4096,8192' "$run_log"
rg -F -q 'present=1,1,1' "$run_log"
rg -F -q 'owner=3,1,2,1,1,0,2' "$run_log"
rg -F -q 'empty=0,1,0' "$run_log"
rg -F -q 'missing=0,2,0' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

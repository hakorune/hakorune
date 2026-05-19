#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-closeout-ssot.md"
INVENTORY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_292A="docs/development/current/main/phases/phase-293x/293x-895-MIMAP-292A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-APPLIED-RECYCLE-INVENTORY.md"
CARD_293A="docs/development/current/main/phases/phase-293x/293x-896-MIMAP-293A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-APPLIED-RECYCLE-DIAGNOSTICS.md"
CARD_294A="docs/development/current/main/phases/phase-293x/293x-897-MIMAP-294A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-APPLIED-RECYCLE-CLOSEOUT.md"
CARD_295A="docs/development/current/main/phases/phase-293x/293x-898-MIMAP-295A-POST-RELEASE-APPLIED-RECYCLE-CLOSEOUT-ROW-SELECTION.md"
GUARD_288A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_guard.sh"
GUARD_289A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-294A segment arena backing modeled allocation-ledger release-applied recycle closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$INVENTORY_SSOT" \
  "$DIAGNOSTICS_SSOT" \
  "$TASKBOARD" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_292A" \
  "$CARD_293A" \
  "$CARD_294A" \
  "$CARD_295A" \
  "$GUARD_288A" \
  "$GUARD_289A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_288A" "$GUARD_289A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_292A" "MIMAP-292A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_293A" "MIMAP-293A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_294A" "MIMAP-294A closeout card must be landed"
guard_expect_in_file "$TAG" "MIMAP-295A" "$CARD_295A" "MIMAP-295A selection card must exist after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-294A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INVENTORY_SSOT" "MIMAP-292A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-293A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack:" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger-release-applied-recycle" "$SSOT" "closeout SSOT must name release-applied recycle pack"
guard_expect_in_file "$TAG" "MIMAP-295A post release-applied recycle closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-294A" "$TASKBOARD" "taskboard must name release-applied recycle closeout row"
guard_expect_in_file "$TAG" "MIMAP-295A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-292A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-292A"
guard_expect_in_file "$TAG" "id = \"MIMAP-293A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-293A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger-release-applied-recycle\"" "$PROOF_MANIFEST" "proof manifest must assign release-applied recycle closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-294A closeout row"
guard_expect_in_file "$TAG" "$GUARD_288A" "$INDEX" "check index must list MIMAP-292A guard"
guard_expect_in_file "$TAG" "$GUARD_289A" "$INDEX" "check index must list MIMAP-293A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-294A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-applied-recycle --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "release-applied recycle L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-292A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-292A"
guard_expect_in_file "$TAG" "MIMAP-293A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-293A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_288A" --level L2
bash "$GUARD_289A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap294_release_applied_recycle_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap294.mir.json"
exe_out="$tmp_dir/mimap294_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,0,6,1,5,194,19,97019005005' "$run_log"
rg -F -q 'seen=1,1,1,1,1' "$run_log"
rg -F -q 'recycle=93019005005,94019005005,95019005005,96019005005,97019005005,4096,4096,8192' "$run_log"
rg -F -q 'present=1,1,1' "$run_log"
rg -F -q 'owner=2,1,1,1,1' "$run_log"
rg -F -q 'empty=0,1,0' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

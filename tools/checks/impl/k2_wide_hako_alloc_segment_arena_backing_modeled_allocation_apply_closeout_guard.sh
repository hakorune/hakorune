#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-apply-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-apply-closeout-ssot.md"
INVENTORY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-apply-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-apply-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_272A="docs/development/current/main/phases/phase-293x/293x-797-MIMAP-272A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-APPLY-INVENTORY.md"
CARD_273A="docs/development/current/main/phases/phase-293x/293x-798-MIMAP-273A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-APPLY-DIAGNOSTICS.md"
CARD_274A="docs/development/current/main/phases/phase-293x/293x-799-MIMAP-274A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-APPLY-CLOSEOUT.md"
CARD_275A="docs/development/current/main/phases/phase-293x/293x-800-MIMAP-275A-POST-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-APPLY-CLOSEOUT-ROW-SELECTION.md"
GUARD_272A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_guard.sh"
GUARD_273A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-apply-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-274A segment arena backing modeled allocation apply closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$INVENTORY_SSOT" \
  "$DIAGNOSTICS_SSOT" \
  "$TASKBOARD" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_272A" \
  "$CARD_273A" \
  "$CARD_274A" \
  "$CARD_275A" \
  "$GUARD_272A" \
  "$GUARD_273A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_272A" "$GUARD_273A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_272A" "MIMAP-272A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_273A" "MIMAP-273A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_274A" "MIMAP-274A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_275A" "MIMAP-275A must be landed after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-274A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INVENTORY_SSOT" "MIMAP-272A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-273A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack = segment-arena-backing-modeled-allocation-apply" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "MIMAP-275A post-segment-arena-backing-modeled-allocation-apply-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-274A segment arena backing modeled allocation apply closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-275A post-segment-arena-backing-modeled-allocation-apply-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-275A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-272A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-272A"
guard_expect_in_file "$TAG" "id = \"MIMAP-273A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-273A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-apply\"" "$PROOF_MANIFEST" "proof manifest must assign modeled allocation-apply closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-apply-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-274A closeout row"
guard_expect_in_file "$TAG" "$GUARD_272A" "$INDEX" "check index must list MIMAP-272A guard"
guard_expect_in_file "$TAG" "$GUARD_273A" "$INDEX" "check index must list MIMAP-273A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-274A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-apply --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "modeled allocation-apply L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-272A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-272A"
guard_expect_in_file "$TAG" "MIMAP-273A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-273A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_272A" --level L2
bash "$GUARD_273A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap274_allocation_apply_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap274.mir.json"
exe_out="$tmp_dir/mimap274_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-apply-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,5,6,1,5,1,5,110,11,1' "$run_log"
rg -F -q 'seen=1,1,1,1,1' "$run_log"
rg -F -q 'apply=91011005005,0,0,0,0' "$run_log"
rg -F -q 'present=1,1,1' "$run_log"
rg -F -q 'owner=2,1,1,1,1' "$run_log"
rg -F -q 'empty=0,1,0' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

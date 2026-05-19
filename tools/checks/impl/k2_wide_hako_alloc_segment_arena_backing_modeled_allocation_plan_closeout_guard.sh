#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-plan-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-plan-closeout-ssot.md"
INVENTORY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-plan-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-plan-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_268A="docs/development/current/main/phases/phase-293x/293x-793-MIMAP-268A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-PLAN-INVENTORY.md"
CARD_269A="docs/development/current/main/phases/phase-293x/293x-794-MIMAP-269A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-PLAN-DIAGNOSTICS.md"
CARD_270A="docs/development/current/main/phases/phase-293x/293x-795-MIMAP-270A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-PLAN-CLOSEOUT.md"
CARD_271A="docs/development/current/main/phases/phase-293x/293x-796-MIMAP-271A-POST-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-PLAN-CLOSEOUT-ROW-SELECTION.md"
GUARD_268A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_plan_guard.sh"
GUARD_269A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_plan_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_plan_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_plan_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-plan-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-270A segment arena backing modeled allocation plan closeout"

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
  "$CARD_268A" \
  "$CARD_269A" \
  "$CARD_270A" \
  "$CARD_271A" \
  "$GUARD_268A" \
  "$GUARD_269A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_268A" "$GUARD_269A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_268A" "MIMAP-268A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_269A" "MIMAP-269A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_270A" "MIMAP-270A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_271A" "MIMAP-271A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-270A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INVENTORY_SSOT" "MIMAP-268A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-269A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack = segment-arena-backing-modeled-allocation-plan" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "MIMAP-271A post-segment-arena-backing-modeled-allocation-plan-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-270A segment arena backing modeled allocation plan closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-271A post-segment-arena-backing-modeled-allocation-plan-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-271A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-268A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-268A"
guard_expect_in_file "$TAG" "id = \"MIMAP-269A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-269A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-plan\"" "$PROOF_MANIFEST" "proof manifest must assign modeled allocation-plan closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-plan-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-270A closeout row"
guard_expect_in_file "$TAG" "$GUARD_268A" "$INDEX" "check index must list MIMAP-268A guard"
guard_expect_in_file "$TAG" "$GUARD_269A" "$INDEX" "check index must list MIMAP-269A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-270A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-plan --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "modeled allocation-plan L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-268A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-268A"
guard_expect_in_file "$TAG" "MIMAP-269A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-269A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_268A" --level L2
bash "$GUARD_269A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap270_allocation_plan_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap270.mir.json"
exe_out="$tmp_dir/mimap270_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-plan-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,5,6,1,5,1,5,90,9,1' "$run_log"
rg -F -q 'seen=1,1,1,1,1' "$run_log"
rg -F -q 'plan=0,0,0,0,12288' "$run_log"
rg -F -q 'present=1,1,1' "$run_log"
rg -F -q 'owner=2,1,1,1,1' "$run_log"
rg -F -q 'empty=0,1,0' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

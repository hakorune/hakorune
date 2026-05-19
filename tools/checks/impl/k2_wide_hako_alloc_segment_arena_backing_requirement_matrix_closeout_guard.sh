#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-requirement-matrix-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-requirement-matrix-closeout-ssot.md"
INVENTORY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-requirement-matrix-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-requirement-matrix-diagnostics-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_240A="docs/development/current/main/phases/phase-293x/293x-763-MIMAP-240A-SEGMENT-ARENA-BACKING-SCALAR-REQUIREMENT-MATRIX-INVENTORY.md"
CARD_241A="docs/development/current/main/phases/phase-293x/293x-764-MIMAP-241A-SEGMENT-ARENA-BACKING-REQUIREMENT-MATRIX-DIAGNOSTICS.md"
CARD_242A="docs/development/current/main/phases/phase-293x/293x-765-MIMAP-242A-SEGMENT-ARENA-BACKING-REQUIREMENT-MATRIX-CLOSEOUT-PACK.md"
CARD_243A="docs/development/current/main/phases/phase-293x/293x-766-MIMAP-243A-POST-SEGMENT-ARENA-BACKING-REQUIREMENT-MATRIX-CLOSEOUT-ROW-SELECTION.md"
GUARD_240A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_guard.sh"
GUARD_241A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-requirement-matrix-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-242A segment arena backing requirement matrix closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$INVENTORY_SSOT" \
  "$DIAGNOSTICS_SSOT" \
  "$CADENCE" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_240A" \
  "$CARD_241A" \
  "$CARD_242A" \
  "$CARD_243A" \
  "$GUARD_240A" \
  "$GUARD_241A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_240A" "$GUARD_241A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_240A" "MIMAP-240A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_241A" "MIMAP-241A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_242A" "MIMAP-242A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_243A" "MIMAP-243A must be landed after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-242A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INVENTORY_SSOT" "MIMAP-240A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-241A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack = segment-arena-backing-requirement-matrix" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "MIMAP-243A post-segment-arena-backing-requirement-matrix-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-242A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-242A"
guard_expect_in_file "$TAG" "MIMAP-243A" "$GRANULARITY" "granularity SSOT must describe MIMAP-243A"
guard_expect_in_file "$TAG" "MIMAP-242A segment arena backing requirement matrix closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-243A post-segment-arena-backing-requirement-matrix-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-243A" "$TASKBOARD" "taskboard must name selected next row"
guard_expect_in_file "$TAG" "MIMAP-242A segment arena backing requirement matrix closeout pack" "$CADENCE" "cadence SSOT must anchor closeout row"

guard_expect_in_file "$TAG" "id = \"MIMAP-240A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-240A"
guard_expect_in_file "$TAG" "id = \"MIMAP-241A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-241A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-requirement-matrix\"" "$PROOF_MANIFEST" "proof manifest must assign requirement matrix closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-requirement-matrix-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-242A closeout row"
guard_expect_in_file "$TAG" "$GUARD_240A" "$INDEX" "check index must list MIMAP-240A guard"
guard_expect_in_file "$TAG" "$GUARD_241A" "$INDEX" "check index must list MIMAP-241A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-242A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-requirement-matrix --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "segment arena backing requirement matrix L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-240A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-240A"
guard_expect_in_file "$TAG" "MIMAP-241A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-241A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_240A" --level L2
bash "$GUARD_241A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap242_arena_matrix_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap242.mir.json"
exe_out="$tmp_dir/mimap242_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-requirement-matrix-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,11,12,1,11,8,11,70,7,1' "$run_log"
rg -F -q 'seen=1,1,1,1,1,1,1,1,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

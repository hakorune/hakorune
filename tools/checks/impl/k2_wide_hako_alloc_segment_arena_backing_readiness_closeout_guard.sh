#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-readiness-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-readiness-closeout-ssot.md"
INVENTORY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-readiness-inventory-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-readiness-diagnostics-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_236A="docs/development/current/main/phases/phase-293x/293x-759-MIMAP-236A-SEGMENT-ARENA-BACKING-READINESS-INVENTORY.md"
CARD_237A="docs/development/current/main/phases/phase-293x/293x-760-MIMAP-237A-SEGMENT-ARENA-BACKING-READINESS-DIAGNOSTICS.md"
CARD_238A="docs/development/current/main/phases/phase-293x/293x-761-MIMAP-238A-SEGMENT-ARENA-BACKING-READINESS-CLOSEOUT-PACK.md"
CARD_239A="docs/development/current/main/phases/phase-293x/293x-762-MIMAP-239A-POST-SEGMENT-ARENA-BACKING-READINESS-CLOSEOUT-ROW-SELECTION.md"
GUARD_236A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_readiness_inventory_guard.sh"
GUARD_237A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_readiness_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_readiness_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_readiness_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-readiness-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-238A segment arena backing readiness closeout"

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
  "$CARD_236A" \
  "$CARD_237A" \
  "$CARD_238A" \
  "$CARD_239A" \
  "$GUARD_236A" \
  "$GUARD_237A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_236A" "$GUARD_237A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_236A" "MIMAP-236A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_237A" "MIMAP-237A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_238A" "MIMAP-238A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_239A" "MIMAP-239A must be landed after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-238A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INVENTORY_SSOT" "MIMAP-236A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-237A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack = segment-arena-backing-readiness" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "MIMAP-239A post-segment-arena-backing-readiness-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-238A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-238A"
guard_expect_in_file "$TAG" "MIMAP-239A" "$GRANULARITY" "granularity SSOT must describe MIMAP-239A"
guard_expect_in_file "$TAG" "MIMAP-238A segment arena backing readiness closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-239A post-segment-arena-backing-readiness-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-239A" "$TASKBOARD" "taskboard must name selected next row"
guard_expect_in_file "$TAG" "MIMAP-238A segment arena backing readiness closeout pack" "$CADENCE" "cadence SSOT must anchor closeout row"

guard_expect_in_file "$TAG" "id = \"MIMAP-236A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-236A"
guard_expect_in_file "$TAG" "id = \"MIMAP-237A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-237A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-readiness\"" "$PROOF_MANIFEST" "proof manifest must assign arena readiness closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-readiness-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-238A closeout row"
guard_expect_in_file "$TAG" "$GUARD_236A" "$INDEX" "check index must list MIMAP-236A guard"
guard_expect_in_file "$TAG" "$GUARD_237A" "$INDEX" "check index must list MIMAP-237A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-238A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-readiness --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "segment arena backing readiness L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-236A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-236A"
guard_expect_in_file "$TAG" "MIMAP-237A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-237A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_236A" --level L2
bash "$GUARD_237A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap238_arena_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap238.mir.json"
exe_out="$tmp_dir/mimap238_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-readiness-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,8,9,1,8,6,1' "$run_log"
rg -F -q 'rejects=1,1,1,1,1,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

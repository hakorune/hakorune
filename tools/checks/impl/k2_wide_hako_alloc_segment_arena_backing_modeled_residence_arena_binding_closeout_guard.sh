#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-residence-arena-binding-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-residence-arena-binding-closeout-ssot.md"
INVENTORY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-residence-arena-binding-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-residence-arena-binding-diagnostics-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_252A="docs/development/current/main/phases/phase-293x/293x-775-MIMAP-252A-SEGMENT-ARENA-BACKING-MODELED-RESIDENCE-ARENA-BINDING-INVENTORY.md"
CARD_253A="docs/development/current/main/phases/phase-293x/293x-776-MIMAP-253A-SEGMENT-ARENA-BACKING-MODELED-RESIDENCE-ARENA-BINDING-DIAGNOSTICS.md"
CARD_254A="docs/development/current/main/phases/phase-293x/293x-777-MIMAP-254A-SEGMENT-ARENA-BACKING-MODELED-RESIDENCE-ARENA-BINDING-CLOSEOUT-PACK.md"
CARD_255A="docs/development/current/main/phases/phase-293x/293x-778-MIMAP-255A-POST-SEGMENT-ARENA-BACKING-MODELED-RESIDENCE-ARENA-BINDING-CLOSEOUT-ROW-SELECTION.md"
GUARD_252A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_residence_arena_binding_guard.sh"
GUARD_253A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_residence_arena_binding_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_residence_arena_binding_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_residence_arena_binding_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-residence-arena-binding-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-254A segment arena backing modeled residence arena-binding closeout"

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
  "$CARD_252A" \
  "$CARD_253A" \
  "$CARD_254A" \
  "$CARD_255A" \
  "$GUARD_252A" \
  "$GUARD_253A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_252A" "$GUARD_253A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_252A" "MIMAP-252A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_253A" "MIMAP-253A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_254A" "MIMAP-254A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_255A" "MIMAP-255A must be landed after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-254A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INVENTORY_SSOT" "MIMAP-252A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-253A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack = segment-arena-backing-modeled-residence-arena-binding" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "MIMAP-255A post-segment-arena-backing-modeled-residence-arena-binding-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-254A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-254A"
guard_expect_in_file "$TAG" "MIMAP-255A" "$GRANULARITY" "granularity SSOT must describe MIMAP-255A"
guard_expect_in_file "$TAG" "MIMAP-254A segment arena backing modeled residence arena-binding closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-255A post-segment-arena-backing-modeled-residence-arena-binding-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-255A" "$TASKBOARD" "taskboard must name selected next row"
guard_expect_in_file "$TAG" "MIMAP-254A segment arena backing modeled residence arena-binding closeout pack" "$CADENCE" "cadence SSOT must anchor closeout row"

guard_expect_in_file "$TAG" "id = \"MIMAP-252A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-252A"
guard_expect_in_file "$TAG" "id = \"MIMAP-253A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-253A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-residence-arena-binding\"" "$PROOF_MANIFEST" "proof manifest must assign modeled residence arena-binding closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-residence-arena-binding-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-254A closeout row"
guard_expect_in_file "$TAG" "$GUARD_252A" "$INDEX" "check index must list MIMAP-252A guard"
guard_expect_in_file "$TAG" "$GUARD_253A" "$INDEX" "check index must list MIMAP-253A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-254A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-residence-arena-binding --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "modeled residence arena-binding L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-252A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-252A"
guard_expect_in_file "$TAG" "MIMAP-253A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-253A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_252A" --level L2
bash "$GUARD_253A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap254_binding_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap254.mir.json"
exe_out="$tmp_dir/mimap254_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-residence-arena-binding-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,9,10,1,9,1,9,70,7,1' "$run_log"
rg -F -q 'seen=1,1,1,1,1,1,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

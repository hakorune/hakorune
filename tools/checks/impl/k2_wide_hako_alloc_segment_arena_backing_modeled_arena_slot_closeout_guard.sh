#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-arena-slot-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-arena-slot-closeout-ssot.md"
INVENTORY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-arena-slot-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-arena-slot-diagnostics-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_256A="docs/development/current/main/phases/phase-293x/293x-779-MIMAP-256A-SEGMENT-ARENA-BACKING-MODELED-ARENA-SLOT-INVENTORY.md"
CARD_257A="docs/development/current/main/phases/phase-293x/293x-780-MIMAP-257A-SEGMENT-ARENA-BACKING-MODELED-ARENA-SLOT-DIAGNOSTICS.md"
CARD_258A="docs/development/current/main/phases/phase-293x/293x-781-MIMAP-258A-SEGMENT-ARENA-BACKING-MODELED-ARENA-SLOT-CLOSEOUT-PACK.md"
CARD_259A="docs/development/current/main/phases/phase-293x/293x-782-MIMAP-259A-POST-SEGMENT-ARENA-BACKING-MODELED-ARENA-SLOT-CLOSEOUT-ROW-SELECTION.md"
GUARD_256A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_guard.sh"
GUARD_257A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-arena-slot-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-258A segment arena backing modeled arena-slot closeout"

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
  "$CARD_256A" \
  "$CARD_257A" \
  "$CARD_258A" \
  "$CARD_259A" \
  "$GUARD_256A" \
  "$GUARD_257A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_256A" "$GUARD_257A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_256A" "MIMAP-256A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_257A" "MIMAP-257A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_258A" "MIMAP-258A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_259A" "MIMAP-259A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-258A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INVENTORY_SSOT" "MIMAP-256A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-257A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack = segment-arena-backing-modeled-arena-slot" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "MIMAP-259A post-segment-arena-backing-modeled-arena-slot-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-258A segment arena backing modeled arena slot closeout pack" "$GRANULARITY" "granularity SSOT must describe MIMAP-258A"
guard_expect_in_file "$TAG" "MIMAP-259A" "$GRANULARITY" "granularity SSOT must describe MIMAP-259A"
guard_expect_in_file "$TAG" "MIMAP-258A segment arena backing modeled arena slot closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-259A post-segment-arena-backing-modeled-arena-slot-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-259A" "$TASKBOARD" "taskboard must name selected next row"
guard_expect_in_file "$TAG" "MIMAP-258A segment arena backing modeled arena slot closeout pack" "$CADENCE" "cadence SSOT must anchor closeout row"

guard_expect_in_file "$TAG" "id = \"MIMAP-256A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-256A"
guard_expect_in_file "$TAG" "id = \"MIMAP-257A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-257A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-arena-slot\"" "$PROOF_MANIFEST" "proof manifest must assign modeled arena-slot closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-arena-slot-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-258A closeout row"
guard_expect_in_file "$TAG" "$GUARD_256A" "$INDEX" "check index must list MIMAP-256A guard"
guard_expect_in_file "$TAG" "$GUARD_257A" "$INDEX" "check index must list MIMAP-257A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-258A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-arena-slot --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "modeled arena-slot L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-256A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-256A"
guard_expect_in_file "$TAG" "MIMAP-257A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-257A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_256A" --level L2
bash "$GUARD_257A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap258_arena_slot_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap258.mir.json"
exe_out="$tmp_dir/mimap258_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-arena-slot-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,7,8,1,7,1,7,70,7,1' "$run_log"
rg -F -q 'seen=1,1,1,1,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

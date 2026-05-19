#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-source-bridge-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-source-bridge-closeout-ssot.md"
INVENTORY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-source-bridge-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-source-bridge-diagnostics-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_260A="docs/development/current/main/phases/phase-293x/293x-783-MIMAP-260A-SEGMENT-ARENA-BACKING-MODELED-SOURCE-BRIDGE-INVENTORY.md"
CARD_261A="docs/development/current/main/phases/phase-293x/293x-784-MIMAP-261A-SEGMENT-ARENA-BACKING-MODELED-SOURCE-BRIDGE-DIAGNOSTICS.md"
CARD_262A="docs/development/current/main/phases/phase-293x/293x-785-MIMAP-262A-SEGMENT-ARENA-BACKING-MODELED-SOURCE-BRIDGE-CLOSEOUT.md"
CARD_263A="docs/development/current/main/phases/phase-293x/293x-786-MIMAP-263A-POST-SEGMENT-ARENA-BACKING-MODELED-SOURCE-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
GUARD_260A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_guard.sh"
GUARD_261A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-source-bridge-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-262A segment arena backing modeled source bridge closeout"

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
  "$CARD_260A" \
  "$CARD_261A" \
  "$CARD_262A" \
  "$CARD_263A" \
  "$GUARD_260A" \
  "$GUARD_261A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_260A" "$GUARD_261A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_260A" "MIMAP-260A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_261A" "MIMAP-261A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_262A" "MIMAP-262A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_263A" "MIMAP-263A row selection must be landed after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-262A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INVENTORY_SSOT" "MIMAP-260A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-261A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack = segment-arena-backing-modeled-source-bridge" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "MIMAP-263A post-segment-arena-backing-modeled-source-bridge-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-262A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-262A"
guard_expect_in_file "$TAG" "MIMAP-263A" "$GRANULARITY" "granularity SSOT must describe MIMAP-263A"
guard_expect_in_file "$TAG" "MIMAP-262A segment arena backing modeled source bridge closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-263A post-segment-arena-backing-modeled-source-bridge-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-263A" "$TASKBOARD" "taskboard must name selected next row"
guard_expect_in_file "$TAG" "MIMAP-262A segment arena backing modeled source bridge closeout pack" "$CADENCE" "cadence SSOT must anchor closeout row"

guard_expect_in_file "$TAG" "id = \"MIMAP-260A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-260A"
guard_expect_in_file "$TAG" "id = \"MIMAP-261A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-261A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-source-bridge\"" "$PROOF_MANIFEST" "proof manifest must assign modeled source bridge closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-source-bridge-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-262A closeout row"
guard_expect_in_file "$TAG" "$GUARD_260A" "$INDEX" "check index must list MIMAP-260A guard"
guard_expect_in_file "$TAG" "$GUARD_261A" "$INDEX" "check index must list MIMAP-261A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-262A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-source-bridge --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "modeled source bridge L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-260A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-260A"
guard_expect_in_file "$TAG" "MIMAP-261A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-261A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_260A" --level L2
bash "$GUARD_261A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap262_source_bridge_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap262.mir.json"
exe_out="$tmp_dir/mimap262_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-source-bridge-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,6,7,1,6,1,6,70,7,1' "$run_log"
rg -F -q 'seen=1,1,1,1,1,1' "$run_log"
rg -F -q 'source=70007004005,1,0,16384,4096,16' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

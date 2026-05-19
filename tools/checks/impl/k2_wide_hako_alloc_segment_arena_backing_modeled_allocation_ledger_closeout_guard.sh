#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-closeout-ssot.md"
INVENTORY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_276A="docs/development/current/main/phases/phase-293x/293x-801-MIMAP-276A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-INVENTORY.md"
CARD_277A="docs/development/current/main/phases/phase-293x/293x-802-MIMAP-277A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-DIAGNOSTICS.md"
CARD_278A="docs/development/current/main/phases/phase-293x/293x-803-MIMAP-278A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-CLOSEOUT.md"
CARD_279A="docs/development/current/main/phases/phase-293x/293x-804-MIMAP-279A-POST-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-CLOSEOUT-ROW-SELECTION.md"
GUARD_276A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_guard.sh"
GUARD_277A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-278A segment arena backing modeled allocation ledger closeout"

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
  "$CARD_276A" \
  "$CARD_277A" \
  "$CARD_278A" \
  "$CARD_279A" \
  "$GUARD_276A" \
  "$GUARD_277A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_276A" "$GUARD_277A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_276A" "MIMAP-276A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_277A" "MIMAP-277A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_278A" "MIMAP-278A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_279A" "MIMAP-279A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-278A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INVENTORY_SSOT" "MIMAP-276A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-277A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack:" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger" "$SSOT" "closeout SSOT must name allocation-ledger pack"
guard_expect_in_file "$TAG" "MIMAP-279A post-segment-arena-backing-modeled-allocation-ledger-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-278A segment arena backing modeled allocation ledger closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-279A post-segment-arena-backing-modeled-allocation-ledger-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-279A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-276A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-276A"
guard_expect_in_file "$TAG" "id = \"MIMAP-277A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-277A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger\"" "$PROOF_MANIFEST" "proof manifest must assign modeled allocation-ledger closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-278A closeout row"
guard_expect_in_file "$TAG" "$GUARD_276A" "$INDEX" "check index must list MIMAP-276A guard"
guard_expect_in_file "$TAG" "$GUARD_277A" "$INDEX" "check index must list MIMAP-277A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-278A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "modeled allocation-ledger L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-276A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-276A"
guard_expect_in_file "$TAG" "MIMAP-277A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-277A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_276A" --level L2
bash "$GUARD_277A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap278_allocation_ledger_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap278.mir.json"
exe_out="$tmp_dir/mimap278_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,5,6,1,5,1,5,130,13,1' "$run_log"
rg -F -q 'seen=1,1,1,1,1' "$run_log"
rg -F -q 'ledger=92013005005,0,4096,4096,8192' "$run_log"
rg -F -q 'present=1,1,1' "$run_log"
rg -F -q 'owner=2,1,1,1,1' "$run_log"
rg -F -q 'empty=0,1,0' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

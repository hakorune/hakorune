#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent-closeout-ssot.md"
INVENTORY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_284A="docs/development/current/main/phases/phase-293x/293x-887-MIMAP-284A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-INTENT-INVENTORY.md"
CARD_285A="docs/development/current/main/phases/phase-293x/293x-888-MIMAP-285A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-INTENT-DIAGNOSTICS.md"
CARD_286A="docs/development/current/main/phases/phase-293x/293x-889-MIMAP-286A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-INTENT-CLOSEOUT.md"
CARD_287A="docs/development/current/main/phases/phase-293x/293x-890-MIMAP-287A-POST-RELEASE-INTENT-CLOSEOUT-ROW-SELECTION.md"
GUARD_284A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_intent_guard.sh"
GUARD_285A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_intent_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_intent_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_intent_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-286A segment arena backing modeled allocation-ledger release intent closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$INVENTORY_SSOT" \
  "$DIAGNOSTICS_SSOT" \
  "$TASKBOARD" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_284A" \
  "$CARD_285A" \
  "$CARD_286A" \
  "$CARD_287A" \
  "$GUARD_284A" \
  "$GUARD_285A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_284A" "$GUARD_285A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_284A" "MIMAP-284A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_285A" "MIMAP-285A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_286A" "MIMAP-286A closeout card must be landed"
guard_expect_in_file "$TAG" "MIMAP-287A" "$CARD_287A" "MIMAP-287A selection card must exist after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-286A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INVENTORY_SSOT" "MIMAP-284A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-285A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack:" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger-release-intent" "$SSOT" "closeout SSOT must name release-intent pack"
guard_expect_in_file "$TAG" "MIMAP-287A post release-intent closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-286A" "$TASKBOARD" "taskboard must name release-intent closeout row"
guard_expect_in_file "$TAG" "MIMAP-287A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-284A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-284A"
guard_expect_in_file "$TAG" "id = \"MIMAP-285A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-285A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger-release-intent\"" "$PROOF_MANIFEST" "proof manifest must assign release-intent closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-286A closeout row"
guard_expect_in_file "$TAG" "$GUARD_284A" "$INDEX" "check index must list MIMAP-284A guard"
guard_expect_in_file "$TAG" "$GUARD_285A" "$INDEX" "check index must list MIMAP-285A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-286A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-intent --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "release-intent L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-284A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-284A"
guard_expect_in_file "$TAG" "MIMAP-285A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-285A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_284A" --level L2
bash "$GUARD_285A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap286_release_intent_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap286.mir.json"
exe_out="$tmp_dir/mimap286_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,0,6,1,5,164,16,95016005005' "$run_log"
rg -F -q 'seen=1,1,1,1,1' "$run_log"
rg -F -q 'intent=93016005005,94016005005,95016005005,4096,4096,8192' "$run_log"
rg -F -q 'present=1,1,1' "$run_log"
rg -F -q 'owner=2,1,1,1,1' "$run_log"
rg -F -q 'empty=0,1,0' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-closeout-ssot.md"
BRIDGE_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_304A="docs/development/current/main/phases/phase-293x/293x-907-MIMAP-304A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-CONTINUATION-APPLICATION-BRIDGE-INVENTORY.md"
CARD_305A="docs/development/current/main/phases/phase-293x/293x-908-MIMAP-305A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-CONTINUATION-APPLICATION-BRIDGE-DIAGNOSTICS.md"
CARD_306A="docs/development/current/main/phases/phase-293x/293x-909-MIMAP-306A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-CONTINUATION-APPLICATION-BRIDGE-CLOSEOUT.md"
CARD_307A="docs/development/current/main/phases/phase-293x/293x-910-MIMAP-307A-POST-RELEASE-RECYCLE-CONTINUATION-APPLICATION-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
GUARD_304A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_continuation_application_bridge_guard.sh"
GUARD_305A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_continuation_application_bridge_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_continuation_application_bridge_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_continuation_application_bridge_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-306A segment arena backing modeled allocation-ledger release/recycle continuation application bridge closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$BRIDGE_SSOT" \
  "$DIAGNOSTICS_SSOT" \
  "$TASKBOARD" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_304A" \
  "$CARD_305A" \
  "$CARD_306A" \
  "$CARD_307A" \
  "$GUARD_304A" \
  "$GUARD_305A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_304A" "$GUARD_305A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_304A" "MIMAP-304A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_305A" "MIMAP-305A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_306A" "MIMAP-306A closeout card must be landed"
guard_expect_in_file "$TAG" "MIMAP-307A" "$CARD_307A" "MIMAP-307A selection card must exist after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-306A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$BRIDGE_SSOT" "MIMAP-304A bridge SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-305A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge" "$SSOT" "closeout SSOT must name continuation application pack"
guard_expect_in_file "$TAG" "MIMAP-307A post release/recycle continuation application bridge closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-306A" "$TASKBOARD" "taskboard must name continuation application closeout row"
guard_expect_in_file "$TAG" "MIMAP-307A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-304A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-304A"
guard_expect_in_file "$TAG" "id = \"MIMAP-305A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-305A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge\"" "$PROOF_MANIFEST" "proof manifest must assign continuation application closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-306A closeout row"
guard_expect_in_file "$TAG" "$GUARD_304A" "$INDEX" "check index must list MIMAP-304A guard"
guard_expect_in_file "$TAG" "$GUARD_305A" "$INDEX" "check index must list MIMAP-305A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-306A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "continuation application bridge L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-304A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-304A"
guard_expect_in_file "$TAG" "MIMAP-305A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-305A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_304A" --level L2
bash "$GUARD_305A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap306_continuation_application_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap306.mir.json"
exe_out="$tmp_dir/mimap306_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,0,1,0,1,1,1,0,190,19,99019005005' "$run_log"
rg -F -q 'application=1,0,1,98019005005,99019005005,4096,4096,8192' "$run_log"
rg -F -q 'present=1,1,1,1' "$run_log"
rg -F -q 'owner=7,1,6,1,1,3,1,3' "$run_log"
rg -F -q 'rejected=1,3,3,1,3,4,1,3,5' "$run_log"
rg -F -q 'empty=0,1,0' "$run_log"
rg -F -q 'missing=0,2,0' "$run_log"
rg -F -q 'unknown=0,4,99019005006' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

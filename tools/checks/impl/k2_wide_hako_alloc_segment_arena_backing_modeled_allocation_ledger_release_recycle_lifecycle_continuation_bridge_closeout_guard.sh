#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-closeout-ssot.md"
BRIDGE_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-diagnostics-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_300A="docs/development/current/main/phases/phase-293x/293x-903-MIMAP-300A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-LIFECYCLE-CONTINUATION-BRIDGE-INVENTORY.md"
CARD_301A="docs/development/current/main/phases/phase-293x/293x-904-MIMAP-301A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-LIFECYCLE-CONTINUATION-BRIDGE-DIAGNOSTICS.md"
CARD_302A="docs/development/current/main/phases/phase-293x/293x-905-MIMAP-302A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-LIFECYCLE-CONTINUATION-BRIDGE-CLOSEOUT.md"
CARD_303A="docs/development/current/main/phases/phase-293x/293x-906-MIMAP-303A-POST-RELEASE-RECYCLE-LIFECYCLE-CONTINUATION-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
GUARD_300A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_continuation_bridge_guard.sh"
GUARD_301A="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_continuation_bridge_diagnostics_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_continuation_bridge_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_continuation_bridge_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"
APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-diagnostics-proof/main.hako"

echo "[$TAG] checking MIMAP-302A segment arena backing modeled allocation-ledger release/recycle lifecycle continuation bridge closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$BRIDGE_SSOT" \
  "$DIAGNOSTICS_SSOT" \
  "$TASKBOARD" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_300A" \
  "$CARD_301A" \
  "$CARD_302A" \
  "$CARD_303A" \
  "$GUARD_300A" \
  "$GUARD_301A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF" \
  "$APP"

guard_require_exec_files "$TAG" "$GUARD_300A" "$GUARD_301A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_300A" "MIMAP-300A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_301A" "MIMAP-301A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_302A" "MIMAP-302A closeout card must be landed"
guard_expect_in_file "$TAG" "MIMAP-303A" "$CARD_303A" "MIMAP-303A selection card must exist after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-302A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$BRIDGE_SSOT" "MIMAP-300A bridge SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-301A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "closeout_pack" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge" "$SSOT" "closeout SSOT must name lifecycle continuation pack"
guard_expect_in_file "$TAG" "MIMAP-303A post release/recycle lifecycle-continuation bridge closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-302A" "$TASKBOARD" "taskboard must name lifecycle continuation closeout row"
guard_expect_in_file "$TAG" "MIMAP-303A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-300A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-300A"
guard_expect_in_file "$TAG" "id = \"MIMAP-301A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-301A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge\"" "$PROOF_MANIFEST" "proof manifest must assign lifecycle continuation closeout pack"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-302A closeout row"
guard_expect_in_file "$TAG" "$GUARD_300A" "$INDEX" "check index must list MIMAP-300A guard"
guard_expect_in_file "$TAG" "$GUARD_301A" "$INDEX" "check index must list MIMAP-301A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-302A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "lifecycle continuation bridge L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-300A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-300A"
guard_expect_in_file "$TAG" "MIMAP-301A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-301A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_300A" --level L2
bash "$GUARD_301A" --level L2

tmp_dir="$(mktemp -d /tmp/hakorune_mimap302_lifecycle_continuation_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap302.mir.json"
exe_out="$tmp_dir/mimap302_exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-lifecycle-continuation-bridge-diagnostics-proof' "$run_log"
rg -F -q 'diag=1,0,1,0,1,1,1,0,190,19,98019005005' "$run_log"
rg -F -q 'bridge=1,0,1,97019005005,98019005005,4096,4096,8192' "$run_log"
rg -F -q 'present=1,1,1' "$run_log"
rg -F -q 'owner=5,1,4,1,1,2,0,3' "$run_log"
rg -F -q 'rejected=1,3,4,1,3,5' "$run_log"
rg -F -q 'empty=0,1,0' "$run_log"
rg -F -q 'missing=0,2,0' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-closeout-ssot.md"
BRIDGE_SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_196A="docs/development/current/main/phases/phase-293x/293x-719-MIMAP-196A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-BRIDGE.md"
CARD_197A="docs/development/current/main/phases/phase-293x/293x-720-MIMAP-197A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-BRIDGE-ROW-SELECTION.md"
CARD_198A="docs/development/current/main/phases/phase-293x/293x-721-MIMAP-198A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-BRIDGE-CLOSEOUT-PACK.md"
CARD_199A="docs/development/current/main/phases/phase-293x/293x-722-MIMAP-199A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-proof/test.sh"
SOURCE_LEDGER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"
RELEASE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_release_box.hako"
GUARD_196A="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_bridge_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_bridge_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_bridge_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-198A segment-map local-free reuse ledger release bridge closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$BRIDGE_SSOT" \
  "$CADENCE" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_196A" \
  "$CARD_197A" \
  "$CARD_198A" \
  "$CARD_199A" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$SOURCE_LEDGER" \
  "$RELEASE_OWNER" \
  "$GUARD_196A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_196A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_196A" "MIMAP-196A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_197A" "MIMAP-197A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_198A" "MIMAP-198A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_199A" "MIMAP-199A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-198A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$BRIDGE_SSOT" "MIMAP-196A bridge SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-map-local-free-reuse-ledger-release-bridge" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "--level L2" "$SSOT" "closeout SSOT must freeze L2 daily command"
guard_expect_in_file "$TAG" "representative L3 EXE evidence" "$SSOT" "closeout SSOT must require L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-199A post-segment-map-local-free-reuse-ledger-release-bridge-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "segment-map-local-free-reuse-ledger-release-bridge" "$CADENCE" "cadence SSOT must name release bridge pack"
guard_expect_in_file "$TAG" "representative L3 EXE evidence in MIMAP-198A" "$CADENCE" "cadence SSOT must anchor MIMAP-198A L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-196A" "$GRANULARITY" "granularity SSOT must describe MIMAP-196A"
guard_expect_in_file "$TAG" "MIMAP-198A" "$GRANULARITY" "granularity SSOT must describe MIMAP-198A"
guard_expect_in_file "$TAG" "MIMAP-199A" "$GRANULARITY" "granularity SSOT must describe MIMAP-199A"
guard_expect_in_file "$TAG" "MIMAP-198A segment-map local-free reuse ledger release bridge closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-199A post-segment-map-local-free-reuse-ledger-release-bridge-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-199A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-196A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-196A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-ledger-release-bridge\"" "$PROOF_MANIFEST" "proof manifest must assign release bridge pack"
guard_expect_in_file "$TAG" "exe = \"deferred-to-closeout\"" "$PROOF_MANIFEST" "MIMAP-196A EXE evidence must stay deferred to closeout"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-198A closeout row"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-ledger-release-bridge\"" "$GUARD_MANIFEST" "guard manifest must assign release bridge pack"
guard_expect_in_file "$TAG" "$GUARD_196A" "$INDEX" "check index must list MIMAP-196A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-198A closeout guard"
guard_expect_in_file "$TAG" "recordLocalFreeReuse" "$SOURCE_LEDGER" "closeout must keep source reuse ledger recording"
guard_expect_in_file "$TAG" "recordReuseLedgerRelease" "$RELEASE_OWNER" "closeout must keep release owner recording"

bash "$RUN_PROOF" --closeout-pack segment-map-local-free-reuse-ledger-release-bridge --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "segment-map local-free reuse ledger release bridge L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-196A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-196A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_196A" --level L2

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap198a_reuse_ledger_release_bridge_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap198a.mir.json"
exe_out="$tmp_dir/mimap198a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-proof' "$vm_log"
rg -F -q 'release=1,0,0,-1,70007004,70007002,70,7,4,1,0' "$vm_log"
rg -F -q 'duplicate=0,3,0' "$vm_log"
rg -F -q 'missing=0,1,-1' "$vm_log"
rg -F -q 'unsupported=0,4' "$vm_log"
rg -F -q 'reads=70007004,4' "$vm_log"
rg -F -q 'counts=4,1,3,1,1,1,1,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'check=1' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-proof' "$run_log"
rg -F -q 'release=1,0,0,-1,70007004,70007002,70,7,4,1,0' "$run_log"
rg -F -q 'duplicate=0,3,0' "$run_log"
rg -F -q 'missing=0,1,-1' "$run_log"
rg -F -q 'unsupported=0,4' "$run_log"
rg -F -q 'reads=70007004,4' "$run_log"
rg -F -q 'counts=4,1,3,1,1,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

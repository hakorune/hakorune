#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-closeout-ssot.md"
BRIDGE_SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_200A="docs/development/current/main/phases/phase-293x/293x-723-MIMAP-200A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLY-BRIDGE.md"
CARD_201A="docs/development/current/main/phases/phase-293x/293x-724-MIMAP-201A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLY-BRIDGE-ROW-SELECTION.md"
CARD_202A="docs/development/current/main/phases/phase-293x/293x-725-MIMAP-202A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLY-BRIDGE-CLOSEOUT-PACK.md"
CARD_203A="docs/development/current/main/phases/phase-293x/293x-726-MIMAP-203A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLY-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-proof/test.sh"
SOURCE_LEDGER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"
RELEASE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_release_box.hako"
GUARD_200A="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_apply_bridge_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_apply_bridge_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_apply_bridge_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-202A segment-map local-free reuse ledger release apply bridge closeout"

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
  "$CARD_200A" \
  "$CARD_201A" \
  "$CARD_202A" \
  "$CARD_203A" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$SOURCE_LEDGER" \
  "$RELEASE_OWNER" \
  "$GUARD_200A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_200A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_200A" "MIMAP-200A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_201A" "MIMAP-201A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_202A" "MIMAP-202A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_203A" "MIMAP-203A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-202A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$BRIDGE_SSOT" "MIMAP-200A bridge SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-map-local-free-reuse-ledger-release-apply-bridge" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "--level L2" "$SSOT" "closeout SSOT must freeze L2 daily command"
guard_expect_in_file "$TAG" "representative L3 EXE evidence" "$SSOT" "closeout SSOT must require L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-203A post-segment-map-local-free-reuse-ledger-release-apply-bridge-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "segment-map-local-free-reuse-ledger-release-apply-bridge" "$CADENCE" "cadence SSOT must name release apply bridge pack"
guard_expect_in_file "$TAG" "representative L3 EXE evidence in MIMAP-202A" "$CADENCE" "cadence SSOT must anchor MIMAP-202A L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-200A" "$GRANULARITY" "granularity SSOT must describe MIMAP-200A"
guard_expect_in_file "$TAG" "MIMAP-202A" "$GRANULARITY" "granularity SSOT must describe MIMAP-202A"
guard_expect_in_file "$TAG" "MIMAP-203A" "$GRANULARITY" "granularity SSOT must describe MIMAP-203A"
guard_expect_in_file "$TAG" "MIMAP-202A segment-map local-free reuse ledger release apply bridge closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-203A post-segment-map-local-free-reuse-ledger-release-apply-bridge-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-203A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-200A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-200A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-ledger-release-apply-bridge\"" "$PROOF_MANIFEST" "proof manifest must assign release apply bridge pack"
guard_expect_in_file "$TAG" "exe = \"deferred-to-closeout\"" "$PROOF_MANIFEST" "MIMAP-200A EXE evidence must stay deferred to closeout"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-202A closeout row"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-ledger-release-apply-bridge\"" "$GUARD_MANIFEST" "guard manifest must assign release apply bridge pack"
guard_expect_in_file "$TAG" "$GUARD_200A" "$INDEX" "check index must list MIMAP-200A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-202A closeout guard"
guard_expect_in_file "$TAG" "applyReuseLedgerRelease" "$SOURCE_LEDGER" "closeout must keep source ledger apply route"
guard_expect_in_file "$TAG" "recordReuseLedgerRelease" "$RELEASE_OWNER" "closeout must keep release owner recording"

bash "$RUN_PROOF" --closeout-pack segment-map-local-free-reuse-ledger-release-apply-bridge --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "segment-map local-free reuse ledger release apply bridge L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-200A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-200A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_200A" --level L2

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap202a_reuse_ledger_release_apply_bridge_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap202a.mir.json"
exe_out="$tmp_dir/mimap202a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-proof' "$vm_log"
rg -F -q 'apply=1,0,0,-1,70007004,70007002,70,7,4,1,0,0' "$vm_log"
rg -F -q 'duplicate=0,3,0' "$vm_log"
rg -F -q 'missing=0,1,-1' "$vm_log"
rg -F -q 'unsupported=0,5' "$vm_log"
rg -F -q 'reads=-1,-1' "$vm_log"
rg -F -q 'counts=4,1,3,1,1,1,1,0' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'check=1' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-proof' "$run_log"
rg -F -q 'apply=1,0,0,-1,70007004,70007002,70,7,4,1,0,0' "$run_log"
rg -F -q 'duplicate=0,3,0' "$run_log"
rg -F -q 'missing=0,1,-1' "$run_log"
rg -F -q 'unsupported=0,5' "$run_log"
rg -F -q 'reads=-1,-1' "$run_log"
rg -F -q 'counts=4,1,3,1,1,1,1,0' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

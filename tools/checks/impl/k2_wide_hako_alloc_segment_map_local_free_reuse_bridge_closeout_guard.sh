#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-bridge-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-bridge-closeout-ssot.md"
BRIDGE_SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-bridge-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_188A="docs/development/current/main/phases/phase-293x/293x-710-MIMAP-188A-SEGMENT-MAP-LOCAL-FREE-REUSE-BRIDGE.md"
CARD_189A="docs/development/current/main/phases/phase-293x/293x-711-MIMAP-189A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-BRIDGE-ROW-SELECTION.md"
CARD_190A="docs/development/current/main/phases/phase-293x/293x-712-MIMAP-190A-SEGMENT-MAP-LOCAL-FREE-REUSE-BRIDGE-CLOSEOUT-PACK.md"
CARD_191A="docs/development/current/main/phases/phase-293x/293x-713-MIMAP-191A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
APP="apps/hako-alloc-segment-map-local-free-reuse-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-bridge-proof/test.sh"
OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
SPAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako"
REUSE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_box.hako"
INTEGRATION_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_integration_box.hako"
PAGE_APPLY_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_page_apply_box.hako"
PAGE_OWNER="lang/src/hako_alloc/memory/page_box.hako"
GUARD_188A="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_bridge_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_bridge_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_map_local_free_reuse_bridge_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-190A segment-map local-free reuse bridge closeout"

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
  "$CARD_188A" \
  "$CARD_189A" \
  "$CARD_190A" \
  "$CARD_191A" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$OWNER" \
  "$SPAN_OWNER" \
  "$REUSE_OWNER" \
  "$INTEGRATION_OWNER" \
  "$PAGE_APPLY_OWNER" \
  "$PAGE_OWNER" \
  "$GUARD_188A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_188A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_188A" "MIMAP-188A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_189A" "MIMAP-189A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_190A" "MIMAP-190A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_191A" "MIMAP-191A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-190A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$BRIDGE_SSOT" "MIMAP-188A bridge SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-map-local-free-reuse-bridge" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "--level L2" "$SSOT" "closeout SSOT must freeze L2 daily command"
guard_expect_in_file "$TAG" "representative L3 EXE evidence" "$SSOT" "closeout SSOT must require L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-191A post-segment-map-local-free-reuse-bridge-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "segment-map-local-free-reuse-bridge" "$CADENCE" "cadence SSOT must name reuse bridge pack"
guard_expect_in_file "$TAG" "representative L3 EXE evidence in MIMAP-190A" "$CADENCE" "cadence SSOT must anchor MIMAP-190A L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-188A" "$GRANULARITY" "granularity SSOT must describe MIMAP-188A"
guard_expect_in_file "$TAG" "MIMAP-190A" "$GRANULARITY" "granularity SSOT must describe MIMAP-190A"
guard_expect_in_file "$TAG" "MIMAP-191A" "$GRANULARITY" "granularity SSOT must describe MIMAP-191A"
guard_expect_in_file "$TAG" "MIMAP-190A segment-map local-free reuse bridge closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-191A post-segment-map-local-free-reuse-bridge-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-191A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-188A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-188A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-bridge\"" "$PROOF_MANIFEST" "proof manifest must assign reuse bridge pack"
guard_expect_in_file "$TAG" "exe = \"deferred-to-closeout\"" "$PROOF_MANIFEST" "MIMAP-188A EXE evidence must stay deferred to closeout"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-map-local-free-reuse-bridge-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-190A closeout row"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-reuse-bridge\"" "$GUARD_MANIFEST" "guard manifest must assign reuse bridge pack"
guard_expect_in_file "$TAG" "$GUARD_188A" "$INDEX" "check index must list MIMAP-188A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-190A closeout guard"
guard_expect_in_file "$TAG" "finishReport" "$REUSE_OWNER" "closeout must keep reuse report construction local"

bash "$RUN_PROOF" --closeout-pack segment-map-local-free-reuse-bridge --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "segment-map local-free reuse bridge L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-188A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-188A"
rm -f /tmp/"$TAG".proof_dry_run

bash "$GUARD_188A" --level L2

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap190a_reuse_bridge_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap190a.mir.json"
exe_out="$tmp_dir/mimap190a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-bridge-proof' "$vm_log"
rg -F -q 'reuse=1,0,4,5,6,0,0,3,2,1' "$vm_log"
rg -F -q 'integration=0,70007002,70,7,2,5,3' "$vm_log"
rg -F -q 'missing=0,1,1' "$vm_log"
rg -F -q 'duplicate=0,1,1' "$vm_log"
rg -F -q 'partial=0,2,2' "$vm_log"
rg -F -q 'unsupported=0,1,1' "$vm_log"
rg -F -q 'recycled=1,0,4,6,2,1' "$vm_log"
rg -F -q 'counts=6,2,4,3,1,0,0' "$vm_log"
rg -F -q 'page=6,0,2,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'check=1' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-local-free-reuse-bridge-proof' "$run_log"
rg -F -q 'reuse=1,0,4,5,6,0,0,3,2,1' "$run_log"
rg -F -q 'integration=0,70007002,70,7,2,5,3' "$run_log"
rg -F -q 'missing=0,1,1' "$run_log"
rg -F -q 'duplicate=0,1,1' "$run_log"
rg -F -q 'partial=0,2,2' "$run_log"
rg -F -q 'unsupported=0,1,1' "$run_log"
rg -F -q 'recycled=1,0,4,6,2,1' "$run_log"
rg -F -q 'counts=6,2,4,3,1,0,0' "$run_log"
rg -F -q 'page=6,0,2,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

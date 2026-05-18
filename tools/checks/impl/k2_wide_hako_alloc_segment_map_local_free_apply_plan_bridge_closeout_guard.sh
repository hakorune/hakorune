#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-apply-plan-bridge-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-apply-plan-bridge-closeout-ssot.md"
BRIDGE_SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-apply-plan-bridge-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_176A="docs/development/current/main/phases/phase-293x/293x-698-MIMAP-176A-SEGMENT-MAP-LOCAL-FREE-APPLY-PLAN-BRIDGE.md"
CARD_177A="docs/development/current/main/phases/phase-293x/293x-699-MIMAP-177A-POST-SEGMENT-MAP-LOCAL-FREE-APPLY-PLAN-BRIDGE-ROW-SELECTION.md"
CARD_178A="docs/development/current/main/phases/phase-293x/293x-700-MIMAP-178A-SEGMENT-MAP-LOCAL-FREE-APPLY-PLAN-BRIDGE-CLOSEOUT-PACK.md"
CARD_179A="docs/development/current/main/phases/phase-293x/293x-701-MIMAP-179A-POST-SEGMENT-MAP-LOCAL-FREE-APPLY-PLAN-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
APP="apps/hako-alloc-segment-map-local-free-apply-plan-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-apply-plan-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-apply-plan-bridge-proof/test.sh"
OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
SPAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako"
CANDIDATE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_candidate_ledger_box.hako"
APPLY_PLAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_apply_plan_box.hako"
GUARD_176A="tools/checks/k2_wide_hako_alloc_segment_map_local_free_apply_plan_bridge_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_apply_plan_bridge_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_map_local_free_apply_plan_bridge_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-178A segment-map local-free apply-plan bridge closeout"

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
  "$CARD_176A" \
  "$CARD_177A" \
  "$CARD_178A" \
  "$CARD_179A" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$OWNER" \
  "$SPAN_OWNER" \
  "$CANDIDATE_OWNER" \
  "$APPLY_PLAN_OWNER" \
  "$GUARD_176A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_176A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_176A" "MIMAP-176A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_177A" "MIMAP-177A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_178A" "MIMAP-178A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_179A" "MIMAP-179A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-178A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$BRIDGE_SSOT" "MIMAP-176A bridge SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-map-local-free-apply-plan-bridge" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "--level L2" "$SSOT" "closeout SSOT must freeze L2 daily command"
guard_expect_in_file "$TAG" "representative L3 EXE evidence" "$SSOT" "closeout SSOT must require L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-179A post-segment-map-local-free-apply-plan-bridge-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "segment-map-local-free-apply-plan-bridge" "$CADENCE" "cadence SSOT must name apply-plan bridge pack"
guard_expect_in_file "$TAG" "MIMAP-176A" "$GRANULARITY" "granularity SSOT must describe MIMAP-176A"
guard_expect_in_file "$TAG" "MIMAP-178A" "$GRANULARITY" "granularity SSOT must describe MIMAP-178A"
guard_expect_in_file "$TAG" "MIMAP-179A" "$GRANULARITY" "granularity SSOT must describe MIMAP-179A"
guard_expect_in_file "$TAG" "MIMAP-178A segment-map local-free apply-plan bridge closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-179A post-segment-map-local-free-apply-plan-bridge-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-179A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-176A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-176A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-apply-plan-bridge\"" "$PROOF_MANIFEST" "proof manifest must assign apply-plan bridge pack"
guard_expect_in_file "$TAG" "exe = \"deferred-to-closeout\"" "$PROOF_MANIFEST" "MIMAP-176A EXE evidence must stay deferred to closeout"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-map-local-free-apply-plan-bridge-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-178A closeout row"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-apply-plan-bridge\"" "$GUARD_MANIFEST" "guard manifest must assign apply-plan bridge pack"
guard_expect_in_file "$TAG" "$GUARD_176A" "$INDEX" "check index must list MIMAP-176A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-178A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-map-local-free-apply-plan-bridge --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "segment-map local-free apply-plan bridge L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-176A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-176A"
rm -f /tmp/"$TAG".proof_dry_run

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$SPAN_OWNER" "$CANDIDATE_OWNER" "$APPLY_PLAN_OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "apply-plan bridge closeout must keep real free/free-list/page-state/raw pointer/concurrency/segment-map/atomics/page-source inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$SPAN_OWNER" "$CANDIDATE_OWNER" "$APPLY_PLAN_OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "apply-plan bridge closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-local-free-apply-plan-bridge-proof|SegmentMapLocalFreeApplyPlan|recordLocalFreeApplyPlan' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "apply-plan bridge app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap178a_apply_plan_bridge_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap178a.mir.json"
exe_out="$tmp_dir/mimap178a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-apply-plan-bridge-proof' "$vm_log"
rg -F -q 'candidate_first=1,0,0,-1,70007002,70,7,2,5,3' "$vm_log"
rg -F -q 'plan_first=1,0,0,-1,0,70007002,70,7,2,5,3,1,1,1' "$vm_log"
rg -F -q 'plan_missing=0,2,-1,1' "$vm_log"
rg -F -q 'plan_duplicate=0,3,0,1' "$vm_log"
rg -F -q 'plan_recycled=1,0,1,-1,1,70007002,70,7,2,5,3,1,2,2' "$vm_log"
rg -F -q 'plan_unsupported=0,5,1' "$vm_log"
rg -F -q 'plan_counts=5,2,2,3,0,1,1,0,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-local-free-apply-plan-bridge-proof' "$run_log"
rg -F -q 'candidate_first=1,0,0,-1,70007002,70,7,2,5,3' "$run_log"
rg -F -q 'plan_first=1,0,0,-1,0,70007002,70,7,2,5,3,1,1,1' "$run_log"
rg -F -q 'plan_missing=0,2,-1,1' "$run_log"
rg -F -q 'plan_duplicate=0,3,0,1' "$run_log"
rg -F -q 'plan_recycled=1,0,1,-1,1,70007002,70,7,2,5,3,1,2,2' "$run_log"
rg -F -q 'plan_unsupported=0,5,1' "$run_log"
rg -F -q 'plan_counts=5,2,2,3,0,1,1,0,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

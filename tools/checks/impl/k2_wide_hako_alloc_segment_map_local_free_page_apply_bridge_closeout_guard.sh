#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-page-apply-bridge-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-page-apply-bridge-closeout-ssot.md"
BRIDGE_SSOT="docs/development/current/main/design/hako-alloc-segment-map-local-free-page-apply-bridge-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_180A="docs/development/current/main/phases/phase-293x/293x-702-MIMAP-180A-SEGMENT-MAP-LOCAL-FREE-PAGE-APPLY-BRIDGE.md"
CARD_181A="docs/development/current/main/phases/phase-293x/293x-703-MIMAP-181A-POST-SEGMENT-MAP-LOCAL-FREE-PAGE-APPLY-BRIDGE-ROW-SELECTION.md"
CARD_182A="docs/development/current/main/phases/phase-293x/293x-704-MIMAP-182A-SEGMENT-MAP-LOCAL-FREE-PAGE-APPLY-BRIDGE-CLOSEOUT-PACK.md"
CARD_183A="docs/development/current/main/phases/phase-293x/293x-705-MIMAP-183A-POST-SEGMENT-MAP-LOCAL-FREE-PAGE-APPLY-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
APP="apps/hako-alloc-segment-map-local-free-page-apply-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-page-apply-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-page-apply-bridge-proof/test.sh"
OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
SPAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako"
CANDIDATE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_candidate_ledger_box.hako"
APPLY_PLAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_apply_plan_box.hako"
PAGE_APPLY_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_page_apply_box.hako"
PAGE_OWNER="lang/src/hako_alloc/memory/page_box.hako"
GUARD_180A="tools/checks/k2_wide_hako_alloc_segment_map_local_free_page_apply_bridge_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_page_apply_bridge_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_map_local_free_page_apply_bridge_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-182A segment-map local-free page-apply bridge closeout"

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
  "$CARD_180A" \
  "$CARD_181A" \
  "$CARD_182A" \
  "$CARD_183A" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$OWNER" \
  "$SPAN_OWNER" \
  "$CANDIDATE_OWNER" \
  "$APPLY_PLAN_OWNER" \
  "$PAGE_APPLY_OWNER" \
  "$PAGE_OWNER" \
  "$GUARD_180A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_180A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_180A" "MIMAP-180A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_181A" "MIMAP-181A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_182A" "MIMAP-182A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_183A" "MIMAP-183A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-182A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$BRIDGE_SSOT" "MIMAP-180A bridge SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-map-local-free-page-apply-bridge" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "--level L2" "$SSOT" "closeout SSOT must freeze L2 daily command"
guard_expect_in_file "$TAG" "representative L3 EXE evidence" "$SSOT" "closeout SSOT must require L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-183A post-segment-map-local-free-page-apply-bridge-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "segment-map-local-free-page-apply-bridge" "$CADENCE" "cadence SSOT must name page-apply bridge pack"
guard_expect_in_file "$TAG" "MIMAP-180A" "$GRANULARITY" "granularity SSOT must describe MIMAP-180A"
guard_expect_in_file "$TAG" "MIMAP-182A" "$GRANULARITY" "granularity SSOT must describe MIMAP-182A"
guard_expect_in_file "$TAG" "MIMAP-183A" "$GRANULARITY" "granularity SSOT must describe MIMAP-183A"
guard_expect_in_file "$TAG" "MIMAP-182A segment-map local-free page-apply bridge closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-183A post-segment-map-local-free-page-apply-bridge-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-183A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-180A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-180A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-page-apply-bridge\"" "$PROOF_MANIFEST" "proof manifest must assign page-apply bridge pack"
guard_expect_in_file "$TAG" "exe = \"deferred-to-closeout\"" "$PROOF_MANIFEST" "MIMAP-180A EXE evidence must stay deferred to closeout"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-map-local-free-page-apply-bridge-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-182A closeout row"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-local-free-page-apply-bridge\"" "$GUARD_MANIFEST" "guard manifest must assign page-apply bridge pack"
guard_expect_in_file "$TAG" "$GUARD_180A" "$INDEX" "check index must list MIMAP-180A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-182A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-map-local-free-page-apply-bridge --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "segment-map local-free page-apply bridge L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-180A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-180A"
rm -f /tmp/"$TAG".proof_dry_run

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$SPAN_OWNER" "$CANDIDATE_OWNER" "$APPLY_PLAN_OWNER" "$PAGE_APPLY_OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "page-apply bridge closeout must keep real free/free-list/raw pointer/concurrency/segment-map/atomics/page-source inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$SPAN_OWNER" "$CANDIDATE_OWNER" "$APPLY_PLAN_OWNER" "$PAGE_APPLY_OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "page-apply bridge closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-local-free-page-apply-bridge-proof|SegmentMapLocalFreePageApply|recordLocalFreePageApply' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "page-apply bridge app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap182a_page_apply_bridge_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap182a.mir.json"
exe_out="$tmp_dir/mimap182a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-page-apply-bridge-proof' "$vm_log"
rg -F -q 'plan_first=1,0,0,-1,0,70007002,70,7,2,5,3' "$vm_log"
rg -F -q 'page_first=1,0,0,-1,0,70007002,70,7,2,5,3,3,6,3,0,3' "$vm_log"
rg -F -q 'page_missing=0,2,-1,1' "$vm_log"
rg -F -q 'page_duplicate=0,3,0,1' "$vm_log"
rg -F -q 'page_wrong=0,4,1' "$vm_log"
rg -F -q 'page_unsupported=0,6,1' "$vm_log"
rg -F -q 'page_recycled=1,0,1,-1,1,70007002,70,7,2,5,3,3,6,3,0,3' "$vm_log"
rg -F -q 'page_counts=6,2,2,4,0,1,1,1,0,1' "$vm_log"
rg -F -q 'page_state=3,2,3,5' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-local-free-page-apply-bridge-proof' "$run_log"
rg -F -q 'plan_first=1,0,0,-1,0,70007002,70,7,2,5,3' "$run_log"
rg -F -q 'page_first=1,0,0,-1,0,70007002,70,7,2,5,3,3,6,3,0,3' "$run_log"
rg -F -q 'page_missing=0,2,-1,1' "$run_log"
rg -F -q 'page_duplicate=0,3,0,1' "$run_log"
rg -F -q 'page_wrong=0,4,1' "$run_log"
rg -F -q 'page_unsupported=0,6,1' "$run_log"
rg -F -q 'page_recycled=1,0,1,-1,1,70007002,70,7,2,5,3,3,6,3,0,3' "$run_log"
rg -F -q 'page_counts=6,2,2,4,0,1,1,1,0,1' "$run_log"
rg -F -q 'page_state=3,2,3,5' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

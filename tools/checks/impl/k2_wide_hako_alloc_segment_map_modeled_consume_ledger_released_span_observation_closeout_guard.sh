#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-closeout-ssot.md"
OBS_SSOT="docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_168A="docs/development/current/main/phases/phase-293x/293x-690-MIMAP-168A-SEGMENT-MAP-MODELED-CONSUME-LEDGER-RELEASED-SPAN-OBSERVATION-ROUTE.md"
CARD_169A="docs/development/current/main/phases/phase-293x/293x-691-MIMAP-169A-POST-SEGMENT-MAP-MODELED-CONSUME-LEDGER-RELEASED-SPAN-OBSERVATION-ROW-SELECTION.md"
CARD_170A="docs/development/current/main/phases/phase-293x/293x-692-MIMAP-170A-SEGMENT-MAP-MODELED-CONSUME-LEDGER-RELEASED-SPAN-OBSERVATION-CLOSEOUT-PACK.md"
CARD_171A="docs/development/current/main/phases/phase-293x/293x-693-MIMAP-171A-POST-SEGMENT-MAP-MODELED-CONSUME-LEDGER-RELEASED-SPAN-OBSERVATION-CLOSEOUT-ROW-SELECTION.md"
APP="apps/hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-proof/test.sh"
OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
SPAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako"
GUARD_168A="tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_span_observation_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_span_observation_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_span_observation_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-170A segment-map modeled consume ledger released-span observation closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$OBS_SSOT" \
  "$CADENCE" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_168A" \
  "$CARD_169A" \
  "$CARD_170A" \
  "$CARD_171A" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$OWNER" \
  "$SPAN_OWNER" \
  "$GUARD_168A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_168A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_168A" "MIMAP-168A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_169A" "MIMAP-169A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_170A" "MIMAP-170A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_171A" "MIMAP-171A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-170A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$OBS_SSOT" "MIMAP-168A observation SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-map-consume-ledger-released-span" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "--level L2" "$SSOT" "closeout SSOT must freeze L2 daily command"
guard_expect_in_file "$TAG" "representative L3 EXE evidence" "$SSOT" "closeout SSOT must require L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-171A post-segment-map-modeled-consume-ledger-released-span-observation-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "segment-map-consume-ledger-released-span" "$CADENCE" "cadence SSOT must name released-span pack"
guard_expect_in_file "$TAG" "MIMAP-168A" "$GRANULARITY" "granularity SSOT must describe MIMAP-168A"
guard_expect_in_file "$TAG" "MIMAP-170A" "$GRANULARITY" "granularity SSOT must describe MIMAP-170A"
guard_expect_in_file "$TAG" "MIMAP-171A" "$GRANULARITY" "granularity SSOT must describe MIMAP-171A"
guard_expect_in_file "$TAG" "MIMAP-170A segment-map modeled consume ledger released-span observation closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-171A post-segment-map-modeled-consume-ledger-released-span-observation-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-171A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-168A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-168A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-consume-ledger-released-span\"" "$PROOF_MANIFEST" "proof manifest must assign released-span closeout pack"
guard_expect_in_file "$TAG" "exe = \"deferred-to-closeout\"" "$PROOF_MANIFEST" "MIMAP-168A EXE evidence must stay deferred to closeout"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-170A closeout row"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-consume-ledger-released-span\"" "$GUARD_MANIFEST" "guard manifest must assign released-span pack"
guard_expect_in_file "$TAG" "$GUARD_168A" "$INDEX" "check index must list MIMAP-168A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-170A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-map-consume-ledger-released-span --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "segment-map consume-ledger released-span L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-168A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-168A"
rm -f /tmp/"$TAG".proof_dry_run

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$SPAN_OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "released-span closeout must keep real allocation/free/free-list/raw pointer/concurrency/segment-map/atomics/page-source inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$SPAN_OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "released-span closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-proof|ReleasedSpanObservation|recordReleasedSpan' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "released-span observation app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap170a_segment_map_released_span_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap170a.mir.json"
exe_out="$tmp_dir/mimap170a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-proof' "$vm_log"
rg -F -q 'release_first=1,0,0,70007002,70,7,2,5,3,1' "$vm_log"
rg -F -q 'first=1,0,0,-1,70007002,70,7,2,5,3,1,1' "$vm_log"
rg -F -q 'missing=0,2,-1,1' "$vm_log"
rg -F -q 'duplicate=0,3,0,1' "$vm_log"
rg -F -q 'recycled=1,0,1,-1,70007002,70,7,2,5,3,2,2' "$vm_log"
rg -F -q 'unsupported=0,4,1' "$vm_log"
rg -F -q 'counts=5,2,2,3,0,1,1,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-proof' "$run_log"
rg -F -q 'release_first=1,0,0,70007002,70,7,2,5,3,1' "$run_log"
rg -F -q 'first=1,0,0,-1,70007002,70,7,2,5,3,1,1' "$run_log"
rg -F -q 'missing=0,2,-1,1' "$run_log"
rg -F -q 'duplicate=0,3,0,1' "$run_log"
rg -F -q 'recycled=1,0,1,-1,70007002,70,7,2,5,3,2,2' "$run_log"
rg -F -q 'unsupported=0,4,1' "$run_log"
rg -F -q 'counts=5,2,2,3,0,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

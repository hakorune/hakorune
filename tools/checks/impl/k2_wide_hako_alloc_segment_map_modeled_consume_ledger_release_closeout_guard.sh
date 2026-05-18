#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-modeled-consume-ledger-release-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-release-closeout-ssot.md"
RELEASE_SSOT="docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-release-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_161A="docs/development/current/main/phases/phase-293x/293x-683-MIMAP-161A-SEGMENT-MAP-MODELED-CONSUME-LEDGER-RELEASE-ROUTE.md"
CARD_162A="docs/development/current/main/phases/phase-293x/293x-684-MIMAP-162A-SEGMENT-MAP-MODELED-CONSUME-LEDGER-RELEASE-CLOSEOUT-PACK.md"
CARD_163A="docs/development/current/main/phases/phase-293x/293x-685-MIMAP-163A-POST-SEGMENT-MAP-MODELED-CONSUME-LEDGER-RELEASE-CLOSEOUT-ROW-SELECTION.md"
APP="apps/hako-alloc-segment-map-modeled-consume-ledger-release-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-modeled-consume-ledger-release-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-modeled-consume-ledger-release-proof/test.sh"
OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
GUARD_161A="tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_release_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_release_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_release_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-162A segment-map modeled consume ledger release closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$RELEASE_SSOT" \
  "$CADENCE" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_161A" \
  "$CARD_162A" \
  "$CARD_163A" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$OWNER" \
  "$GUARD_161A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_161A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_161A" "MIMAP-161A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_162A" "MIMAP-162A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_163A" "MIMAP-163A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-162A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$RELEASE_SSOT" "MIMAP-161A release SSOT must stay accepted"
guard_expect_in_file "$TAG" "segment-map-consume-ledger-release" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "--level L2" "$SSOT" "closeout SSOT must freeze L2 daily command"
guard_expect_in_file "$TAG" "representative L3 EXE evidence" "$SSOT" "closeout SSOT must require L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-163A post-segment-map-modeled-consume-ledger-release-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "segment-map-consume-ledger-release" "$CADENCE" "cadence SSOT must name release pack"
guard_expect_in_file "$TAG" "MIMAP-161A" "$GRANULARITY" "granularity SSOT must describe MIMAP-161A"
guard_expect_in_file "$TAG" "MIMAP-162A" "$GRANULARITY" "granularity SSOT must describe MIMAP-162A"
guard_expect_in_file "$TAG" "MIMAP-163A" "$GRANULARITY" "granularity SSOT must describe MIMAP-163A"
guard_expect_in_file "$TAG" "MIMAP-162A segment-map modeled consume ledger release closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-163A post-segment-map-modeled-consume-ledger-release-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-163A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-161A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-161A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-consume-ledger-release\"" "$PROOF_MANIFEST" "proof manifest must assign release closeout pack"
guard_expect_in_file "$TAG" "exe = \"deferred-to-closeout\"" "$PROOF_MANIFEST" "MIMAP-161A EXE evidence must stay deferred to closeout"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-map-modeled-consume-ledger-release-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-162A closeout row"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-consume-ledger-release\"" "$GUARD_MANIFEST" "guard manifest must assign release pack"
guard_expect_in_file "$TAG" "$GUARD_161A" "$INDEX" "check index must list MIMAP-161A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-162A closeout guard"

bash "$RUN_PROOF" --closeout-pack segment-map-consume-ledger-release --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "segment-map consume-ledger release L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-161A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-161A"
rm -f /tmp/"$TAG".proof_dry_run

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "release closeout must keep real free/raw pointer/concurrency/segment-map/atomics/page-source inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "release closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-modeled-consume-ledger-release-proof|HakoAllocSegmentMapModeledConsumeLedgerRelease|releaseConsumedToken' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "release app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap162a_segment_map_release_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap162a.mir.json"
exe_out="$tmp_dir/mimap162a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-modeled-consume-ledger-release-proof' "$vm_log"
rg -F -q 'release_first=1,0,0,0,70007002,70,7,2,1,0,1,0,3,1' "$vm_log"
rg -F -q 'blocked=0,4,4,-1,70007002' "$vm_log"
rg -F -q 'rejects=1,2,3,4,1,2,3,4' "$vm_log"
rg -F -q 'release_counts=5,1,4,1,1,1,1,70007002,4,0' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-modeled-consume-ledger-release-proof' "$run_log"
rg -F -q 'release_first=1,0,0,0,70007002,70,7,2,1,0,1,0,3,1' "$run_log"
rg -F -q 'blocked=0,4,4,-1,70007002' "$run_log"
rg -F -q 'rejects=1,2,3,4,1,2,3,4' "$run_log"
rg -F -q 'release_counts=5,1,4,1,1,1,1,70007002,4,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
case "$VALIDATION_LEVEL" in
  L0|L1|L2|L3) ;;
  L4)
    echo "[$TAG] ERROR: MIMAP-232A is first-pattern L3; L4 belongs to future batch packs" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-proof/test.sh"
CARD_230A="docs/development/current/main/phases/phase-293x/293x-753-MIMAP-230A-SOURCE-RELEASE-LEDGER-LIFECYCLE-KEY-MIGRATION-CLOSEOUT-PACK.md"
CARD_231A="docs/development/current/main/phases/phase-293x/293x-754-MIMAP-231A-POST-SOURCE-RELEASE-LEDGER-LIFECYCLE-KEY-MIGRATION-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-755-MIMAP-232A-SOURCE-LIFECYCLE-KEYED-RELEASE-APPLY-RECYCLE-CONTINUATION-BRIDGE.md"
CARD_233A="docs/development/current/main/phases/phase-293x/293x-756-MIMAP-233A-SOURCE-LIFECYCLE-KEYED-RELEASE-APPLY-RECYCLE-CONTINUATION-DIAGNOSTICS.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
REUSE_LEDGER_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"
LIFECYCLE_RELEASE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_ledger_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_apply_recycle_continuation_guard.sh"

printf '[%s] checking MIMAP-232A lifecycle-keyed release apply/recycle continuation\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_230A" \
  "$CARD_231A" \
  "$CARD" \
  "$CARD_233A" \
  "$DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$REUSE_LEDGER_OWNER" \
  "$LIFECYCLE_RELEASE_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_230A" "MIMAP-230A closeout must be landed before continuation"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_231A" "MIMAP-231A selection must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-232A card must be landed"
guard_expect_in_file "$TAG" 'Status: (landed|selected current)' "$CARD_233A" "MIMAP-233A must be selected current or landed after MIMAP-232A"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-232A design must be accepted"
guard_expect_in_file "$TAG" 'modeled_reuse_token` as an explicit backref' "$DESIGN" "design must name modeled token backref"
guard_expect_in_file "$TAG" 'MIMAP-232A' "$PLAN" "granularity SSOT must describe MIMAP-232A"
guard_expect_in_file "$TAG" 'MIMAP-233A' "$PLAN" "granularity SSOT must describe MIMAP-233A"
guard_expect_in_file "$TAG" 'MIMAP-232A source lifecycle-keyed release apply/recycle continuation bridge' "$JOINT" "joint order must name MIMAP-232A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-232A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-232A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-232A"
guard_expect_in_file "$TAG" 'validation_profile = "first-pattern"' "$PROOF_MANIFEST" "MIMAP-232A must be first-pattern validation"
guard_expect_in_file "$TAG" 'exe = "auto"' "$PROOF_MANIFEST" "MIMAP-232A EXE evidence must be enabled"
guard_expect_in_file "$TAG" 'applyReuseLedgerLifecycleKeyedRelease' "$REUSE_LEDGER_OWNER" "reuse ledger owner must expose lifecycle-keyed apply entry"
guard_expect_in_file "$TAG" 'applyReuseLedgerLifecycleKeyedRelease' "$MEMORY_README" "memory README must name lifecycle-keyed apply entry"
guard_expect_in_file "$TAG" 'source_modeled_allocation_token: i64 = -1' "$LIFECYCLE_RELEASE_OWNER" "lifecycle-keyed release report must carry apply-compatible source field"
guard_expect_in_file "$TAG" 'check "mimap232a source lifecycle-keyed release apply recycle continuation bridge"' "$APP" "MIMAP-232A proof must use labelled check block"

if rg -n 'recordReuseLedgerRelease[[:space:]]*\(' "$APP" >/tmp/"$TAG".old_release_use 2>&1; then
  if ! rg -n 'old_release = old_release_ledger.recordReuseLedgerRelease' "$APP" >/dev/null 2>&1 || \
     ! rg -n 'old_second_release = old_release_ledger.recordReuseLedgerRelease' "$APP" >/dev/null 2>&1; then
    echo "[$TAG] ERROR: MIMAP-232A may use old release owner only as isolated setup/precondition fixture input" >&2
    cat /tmp/"$TAG".old_release_use >&2
    rm -f /tmp/"$TAG".old_release_use
    exit 1
  fi
fi
rm -f /tmp/"$TAG".old_release_use

if rg -n 'realLifecycle|generationToken|migrateReleaseLedger|releaseLedgerKeyMigration' \
  "$APP" "$REUSE_LEDGER_OWNER" >/tmp/"$TAG".real_lifecycle_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-232A must not define real lifecycle/generation or broad migration machinery" >&2
  cat /tmp/"$TAG".real_lifecycle_leak >&2
  rm -f /tmp/"$TAG".real_lifecycle_leak
  exit 1
fi
rm -f /tmp/"$TAG".real_lifecycle_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$APP" "$REUSE_LEDGER_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-232A must keep real execution/raw pointer/concurrency/segment-map/atomics/page-source/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-proof|applyReuseLedgerLifecycleKeyedRelease|LifecycleKeyedReleaseApplyRecycle' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-232A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap232_lifecycle_apply_recycle.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap232.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-proof' "$vm_log"
rg -F -q 'base=70007004,70007004002,3' "$vm_log"
rg -F -q 'apply=1,0,1,70007004,0' "$vm_log"
rg -F -q 'continued=1,0,2,70007004,1' "$vm_log"
rg -F -q 'reads=-1,-1,70007004,4' "$vm_log"
rg -F -q 'rejects=0,4,0,5' "$vm_log"
rg -F -q 'counts=4,3,1,1,3,1,3,2,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'check=1' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

if ! pure_first_guard_level_allows_mir "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"

python3 - "$mir_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
required = {
    "main",
    "Main.nextReuseReport/4",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedger.applyReuseLedgerLifecycleKeyedRelease/2",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleKeyedReleaseLedger.recordLifecycleKeyedRelease/3",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleKeyedReleaseLedgerReport")
if report is None:
    raise SystemExit("missing lifecycle-keyed release ledger report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "did_release",
    "reason",
    "reuse_lifecycle_token",
    "modeled_reuse_token",
    "source_modeled_allocation_token",
    "segment_id",
    "page_id",
    "reused_block_id",
    "release_key_migrated",
    "lifecycle_keyed_release_ledger_present",
):
    if name not in fields:
        raise SystemExit(f"missing lifecycle-keyed release ledger report field: {name}")

print("[mimap232a-mir-json] ok")
PY

if ! pure_first_guard_level_allows_exe "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

pure_first_guard_build_toolchain

exe_out="$tmp_dir/mimap232.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-proof' "$run_log"
rg -F -q 'base=70007004,70007004002,3' "$run_log"
rg -F -q 'apply=1,0,1,70007004,0' "$run_log"
rg -F -q 'continued=1,0,2,70007004,1' "$run_log"
rg -F -q 'reads=-1,-1,70007004,4' "$run_log"
rg -F -q 'rejects=0,4,0,5' "$run_log"
rg -F -q 'counts=4,3,1,1,3,1,3,2,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok level=$VALIDATION_LEVEL"

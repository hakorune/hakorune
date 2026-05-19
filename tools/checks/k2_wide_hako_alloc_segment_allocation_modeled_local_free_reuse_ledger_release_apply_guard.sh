#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply-proof/main.hako"
APP_README="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply-proof/README.md"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-644-MIMAP-138A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLY-ROUTE.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"
RELEASE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_release_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_guard.sh"

printf '[%s] checking MIMAP-138A segment allocation modeled local-free reuse ledger release apply route\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$MODULE" \
  "$OWNER" \
  "$RELEASE_OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-138A card must be landed"
guard_expect_in_file "$TAG" 'MIMAP-138A' "$PLAN" "granularity SSOT must describe MIMAP-138A"
guard_expect_in_file "$TAG" 'MIMAP-138A segment allocation modeled local-free reuse ledger release apply route' "$JOINT" "joint order must name MIMAP-138A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-138A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-138A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-138A"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_reuse_ledger_box = "memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"' "$MODULE" "hako module must export reuse ledger owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_reuse_ledger_box.hako` owns' "$MEMORY_README" "memory README must define reuse ledger owner"
guard_expect_in_file "$TAG" 'applyReuseLedgerRelease' "$OWNER" "reuse ledger owner must expose release apply route"
guard_expect_in_file "$TAG" 'release_apply_count_after: usize = 0' "$OWNER" "release apply count must be exact usize"
guard_expect_in_file "$TAG" 'release_apply_reject_count_after: usize = 0' "$OWNER" "release apply reject count must be exact usize"
guard_expect_in_file "$TAG" 'ledger_live_count_after: usize = 0' "$OWNER" "release apply ledger live count must be exact usize"
guard_expect_in_file "$TAG" 'release_apply_attempt_count: usize = 0' "$OWNER" "release apply attempt counter must be exact usize"
guard_expect_in_file "$TAG" 'release_apply_count: usize = 0' "$OWNER" "release apply counter must be exact usize"
guard_expect_in_file "$TAG" 'release_apply_reject_count: usize = 0' "$OWNER" "release apply reject counter must be exact usize"
guard_expect_in_file "$TAG" 'release_apply_upstream_reject_count: usize = 0' "$OWNER" "release apply upstream reject counter must be exact usize"
guard_expect_in_file "$TAG" 'release_apply_invalid_shape_reject_count: usize = 0' "$OWNER" "release apply invalid-shape reject counter must be exact usize"
guard_expect_in_file "$TAG" 'release_apply_duplicate_reject_count: usize = 0' "$OWNER" "release apply duplicate reject counter must be exact usize"
guard_expect_in_file "$TAG" 'release_apply_missing_reject_count: usize = 0' "$OWNER" "release apply missing reject counter must be exact usize"
guard_expect_in_file "$TAG" 'release_apply_execution_reject_count: usize = 0' "$OWNER" "release apply execution reject counter must be exact usize"
guard_expect_in_file "$TAG" 'release_apply_raw_pointer_reject_count: usize = 0' "$OWNER" "release apply raw-pointer reject counter must be exact usize"
guard_expect_in_file "$TAG" 'release_apply_backend_matcher_reject_count: usize = 0' "$OWNER" "release apply backend-matcher reject counter must be exact usize"
guard_expect_in_file "$TAG" 'modeled_reuse_token: i64 = -1' "$OWNER" "release apply token sentinel must remain i64"
guard_expect_in_file "$TAG" 'source_modeled_allocation_token: i64 = -1' "$OWNER" "release apply source token sentinel must remain i64"
guard_expect_in_file "$TAG" 'reused_block_id: i64 = -1' "$OWNER" "release apply reused block id must remain i64"
guard_expect_in_file "$TAG" 'local_free_reuse_ledger_release_apply_present' "$OWNER" "release apply report must expose presence flag"
guard_expect_in_file "$TAG" 'check "mimap138a segment allocation modeled local-free reuse ledger release apply route"' "$APP" "MIMAP-138A proof must use labelled check block"

if rg -n 'segment_allocation_modeled_ledger_box|recordModeledConsume|releaseModeledToken' "$OWNER" >/tmp/"$TAG".bump_ledger_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-138A owner must not widen or depend on the bump-shaped modeled ledger" >&2
  cat /tmp/"$TAG".bump_ledger_leak >&2
  rm -f /tmp/"$TAG".bump_ledger_leak
  exit 1
fi
rm -f /tmp/"$TAG".bump_ledger_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-138A must not add real segment execution, raw pointer, concurrency, segment-map, atomics, page-source/OS release seams, or backend-visible page release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'page\.(free|local_free|block_used)' "$OWNER" >/tmp/"$TAG".page_array_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-138A owner must not mutate page arrays directly" >&2
  cat /tmp/"$TAG".page_array_leak >&2
  rm -f /tmp/"$TAG".page_array_leak
  exit 1
fi
rm -f /tmp/"$TAG".page_array_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-138A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply-proof|LocalFreeReuseLedgerReleaseApply|applyReuseLedgerRelease' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-138A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-138A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap138a_local_free_reuse_ledger_release_apply.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap138a.mir.json"
exe_out="$tmp_dir/mimap138a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply-proof' "$vm_log"
rg -F -q 'apply=1,0,0,60018004,0' "$vm_log"
rg -F -q 'duplicate=0,3,0' "$vm_log"
rg -F -q 'missing=0,4,-1' "$vm_log"
rg -F -q 'unsupported=0,5' "$vm_log"
rg -F -q 'reads=-1,-1' "$vm_log"
rg -F -q 'counts=4,1,3,1,1,1,0' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

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
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedger.applyReuseLedgerRelease/2",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedger.findAnyIndex/1",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedger.findIndex/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReleaseApplyReport")
if report is None:
    raise SystemExit("missing local-free reuse ledger release apply report typed object plan")
owner = plans.get("HakoAllocSegmentAllocationModeledLocalFreeReuseLedger")
if owner is None:
    raise SystemExit("missing local-free reuse ledger owner typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
usize_fields = (
    "release_apply_count_after",
    "release_apply_reject_count_after",
    "ledger_live_count_after",
)
i64_fields = (
    "did_apply",
    "reason",
    "row_index",
    "existing_index",
    "modeled_reuse_token",
    "source_modeled_allocation_token",
    "segment_id",
    "page_id",
    "reused_block_id",
    "local_free_reuse_ledger_release_apply_present",
)
for name in usize_fields:
    field = fields.get(name)
    if field is None:
        raise SystemExit(f"missing local-free reuse ledger release apply field: {name}")
    if field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"bad usize local-free reuse ledger release apply field {name}: {field}")

for name in i64_fields:
    field = fields.get(name)
    if field is None:
        raise SystemExit(f"missing local-free reuse ledger release apply field: {name}")
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad i64 local-free reuse ledger release apply field {name}: {field}")

for name in (
    "would_execute_real_segment_allocation",
    "would_execute_real_segment_free",
    "would_directly_mutate_page_arrays",
    "would_use_raw_pointer",
    "would_use_segment_map",
    "would_allocate_arena_backing",
):
    if name not in fields:
        raise SystemExit(f"missing local-free reuse ledger release apply field: {name}")

owner_fields = {field.get("name"): field for field in owner.get("fields", [])}
owner_usize_fields = (
    "release_apply_attempt_count",
    "release_apply_count",
    "release_apply_reject_count",
    "release_apply_upstream_reject_count",
    "release_apply_invalid_shape_reject_count",
    "release_apply_duplicate_reject_count",
    "release_apply_missing_reject_count",
    "release_apply_execution_reject_count",
    "release_apply_raw_pointer_reject_count",
    "release_apply_segment_map_reject_count",
    "release_apply_arena_reject_count",
    "release_apply_atomic_bitmap_reject_count",
    "release_apply_osvm_reject_count",
    "release_apply_thread_reject_count",
    "release_apply_provider_reject_count",
    "release_apply_backend_matcher_reject_count",
)
owner_i64_fields = (
    "last_token",
    "last_reason",
    "last_index",
)
for name in owner_usize_fields:
    field = owner_fields.get(name)
    if field is None:
        raise SystemExit(f"missing local-free reuse ledger owner field: {name}")
    if field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"bad usize local-free reuse ledger owner field {name}: {field}")

for name in owner_i64_fields:
    field = owner_fields.get(name)
    if field is None:
        raise SystemExit(f"missing local-free reuse ledger owner field: {name}")
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad i64 local-free reuse ledger owner field {name}: {field}")

print("[mimap138a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply-proof' "$run_log"
rg -F -q 'apply=1,0,0,60018004,0' "$run_log"
rg -F -q 'duplicate=0,3,0' "$run_log"
rg -F -q 'missing=0,4,-1' "$run_log"
rg -F -q 'unsupported=0,5' "$run_log"
rg -F -q 'reads=-1,-1' "$run_log"
rg -F -q 'counts=4,1,3,1,1,1,0' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

printf '[%s] ok\n' "$TAG"

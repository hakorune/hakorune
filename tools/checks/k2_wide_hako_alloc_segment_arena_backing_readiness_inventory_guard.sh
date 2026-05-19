#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-readiness-inventory"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -eq 0 ]; then
  VALIDATION_LEVEL="L2"
else
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
fi
case "$VALIDATION_LEVEL" in
  L0|L1|L2) ;;
  L3|L4)
    echo "[$TAG] ERROR: MIMAP-236A is inventory-only and defers L3/L4 evidence to future closeout rows" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-readiness-inventory-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-readiness-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-readiness-inventory-proof/test.sh"
CARD_234A="docs/development/current/main/phases/phase-293x/293x-757-MIMAP-234A-SOURCE-LIFECYCLE-KEYED-RELEASE-APPLY-RECYCLE-CONTINUATION-CLOSEOUT-PACK.md"
CARD_235A="docs/development/current/main/phases/phase-293x/293x-758-MIMAP-235A-POST-SOURCE-LIFECYCLE-KEYED-RELEASE-APPLY-RECYCLE-CONTINUATION-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-759-MIMAP-236A-SEGMENT-ARENA-BACKING-READINESS-INVENTORY.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-readiness-inventory-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/segment_arena_backing_readiness_inventory_box.hako"
CONTINUATION_DIAGNOSTIC_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_apply_recycle_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_readiness_inventory_guard.sh"

printf '[%s] checking MIMAP-236A segment arena backing readiness inventory\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_234A" \
  "$CARD_235A" \
  "$CARD" \
  "$DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$OWNER" \
  "$CONTINUATION_DIAGNOSTIC_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_234A" "MIMAP-234A closeout must be landed before arena readiness"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_235A" "MIMAP-235A selection must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-236A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-236A design must be accepted"
guard_expect_in_file "$TAG" 'Reason Vocabulary' "$DESIGN" "MIMAP-236A design must define reasons"
guard_expect_in_file "$TAG" 'MIMAP-236A' "$PLAN" "granularity SSOT must describe MIMAP-236A"
guard_expect_in_file "$TAG" 'MIMAP-237A' "$PLAN" "granularity SSOT must describe MIMAP-237A"
guard_expect_in_file "$TAG" 'MIMAP-236A segment arena backing readiness inventory' "$JOINT" "joint order must name MIMAP-236A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-236A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-236A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-236A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-236A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-236A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_readiness_inventory_box' "$MODULE" "module must export owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_readiness_inventory_box.hako' "$MEMORY_README" "memory README must name owner"
guard_expect_in_file "$TAG" 'classifyReadiness' "$OWNER" "owner must expose readiness classifier"
guard_expect_in_file "$TAG" 'arena_backing_readiness_present: i64 = 1' "$OWNER" "report must publish presence bit"
guard_expect_in_file "$TAG" 'slice_count: usize = 0' "$OWNER" "readiness slice count must be exact usize after HAKO-ALLOC-USIZE-FIELD-GROUP-032"
guard_expect_in_file "$TAG" 'committed_slices: usize = 0' "$OWNER" "readiness committed slices must be exact usize after HAKO-ALLOC-USIZE-FIELD-GROUP-032"
guard_expect_in_file "$TAG" 'free_slices: usize = 0' "$OWNER" "readiness free slices must be exact usize after HAKO-ALLOC-USIZE-FIELD-GROUP-032"
guard_expect_in_file "$TAG" 'page_size: usize = 0' "$OWNER" "readiness page size must be exact usize after HAKO-ALLOC-USIZE-FIELD-GROUP-032"
guard_expect_in_file "$TAG" 'required_alignment: i64 = 0' "$OWNER" "readiness alignment must remain i64"
guard_expect_in_file "$TAG" 'segment_id: i64 = -1' "$OWNER" "readiness id sentinel must remain i64"
guard_expect_in_file "$TAG" 'check "mimap236a segment arena backing readiness inventory"' "$APP" "proof must use labelled check block"

if rg -n 'allocateArena|ArenaBackingAlloc|arenaBackingAllocate|rawPointer|pointer_member|lookupSegment[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-236A must keep arena/raw/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-readiness-inventory-proof|SegmentArenaBackingReadinessInventory|arenaBackingReadiness' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-236A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap236_arena_readiness.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap236.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-readiness-inventory-proof' "$vm_log"
rg -F -q 'base=70007004,70007004002,1' "$vm_log"
rg -F -q 'readiness=1,0,70,7,16,8,8,8,4096,1' "$vm_log"
rg -F -q 'rejects=1,2,3,4,5,6,7,8' "$vm_log"
rg -F -q 'counts=9,1,8,1,1,1,1,1,1,1,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleKeyedReleaseApplyRecycleDiagnostic.observeApplyRecycleDiagnostics/2",
    "HakoAllocSegmentArenaBackingReadinessInventory.classifyReadiness/14",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingReadinessInventoryReport")
if report is None:
    raise SystemExit("missing arena backing readiness report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "accepted",
    "reason",
    "segment_id",
    "arena_id",
    "slice_count",
    "committed_slices",
    "free_slices",
    "required_alignment",
    "page_size",
    "lifecycle_continuation_observed",
    "lifecycle_keyed_apply_seen",
    "arena_backing_readiness_present",
    "scalar_arena_backing_model",
    "would_allocate_arena_backing",
    "would_use_raw_pointer",
    "would_use_segment_map",
    "would_execute_atomic_bitmap",
    "would_call_osvm",
    "would_run_thread",
    "would_activate_provider",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing arena backing readiness report field: {name}")

for name in (
    "slice_count",
    "committed_slices",
    "free_slices",
    "page_size",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"readiness {name} must be exact usize storage: {field}")

for name in (
    "accepted",
    "reason",
    "segment_id",
    "arena_id",
    "required_alignment",
    "lifecycle_continuation_observed",
    "arena_backing_readiness_present",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"readiness {name} must remain i64 storage: {field}")

print("[mimap236a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

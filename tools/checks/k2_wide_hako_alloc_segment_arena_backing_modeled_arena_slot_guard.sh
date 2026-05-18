#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-arena-slot"
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
    echo "[$TAG] ERROR: MIMAP-256A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-arena-slot-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-arena-slot-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-arena-slot-proof/test.sh"
CARD_254A="docs/development/current/main/phases/phase-293x/293x-777-MIMAP-254A-SEGMENT-ARENA-BACKING-MODELED-RESIDENCE-ARENA-BINDING-CLOSEOUT-PACK.md"
CARD_255A="docs/development/current/main/phases/phase-293x/293x-778-MIMAP-255A-POST-SEGMENT-ARENA-BACKING-MODELED-RESIDENCE-ARENA-BINDING-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-779-MIMAP-256A-SEGMENT-ARENA-BACKING-MODELED-ARENA-SLOT-INVENTORY.md"
BINDING_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-residence-arena-binding-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-arena-slot-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
BINDING_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_residence_arena_binding_box.hako"
OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_arena_slot_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_guard.sh"

printf '[%s] checking MIMAP-256A segment arena backing modeled arena slot\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_254A" \
  "$CARD_255A" \
  "$CARD" \
  "$BINDING_SSOT" \
  "$DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$CADENCE" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$BINDING_OWNER" \
  "$OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_254A" "MIMAP-254A closeout must be landed before modeled arena-slot inventory"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_255A" "MIMAP-255A selection must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-256A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$BINDING_SSOT" "MIMAP-252A arena-binding design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-256A modeled arena-slot design must be accepted"
guard_expect_in_file "$TAG" 'modeled arena slot accepted' "$DESIGN" "MIMAP-256A reason vocabulary must be present"
guard_expect_in_file "$TAG" 'MIMAP-256A segment arena backing modeled arena slot inventory' "$PLAN" "granularity SSOT must describe MIMAP-256A"
guard_expect_in_file "$TAG" 'MIMAP-256A segment arena backing modeled arena slot inventory' "$JOINT" "joint order must name MIMAP-256A"
guard_expect_in_file "$TAG" 'segment-arena-backing-modeled-arena-slot' "$CADENCE" "validation cadence must name MIMAP-256A closeout pack"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-256A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-256A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-256A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-256A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-256A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_arena_slot_box' "$MODULE" "module must export modeled arena-slot owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_arena_slot_box.hako' "$MEMORY_README" "memory README must name modeled arena-slot owner"
guard_expect_in_file "$TAG" 'recordArenaSlot' "$OWNER" "modeled arena-slot owner must expose record route"
guard_expect_in_file "$TAG" 'check "mimap256a segment arena backing modeled arena slot"' "$APP" "proof must use labelled check block"

if rg -n 'lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-256A must keep pointer lookup/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-arena-slot-proof|ModeledArenaSlot|modeledArenaSlot' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-256A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap256_arena_slot.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap256.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-arena-slot-proof' "$vm_log"
rg -F -q 'slot=1,0,1,70,7,3,70007004002,70007004002,70007004005,2,2048,4096,8192,16' "$vm_log"
rg -F -q 'geometry=16,8,4,16,4096,1' "$vm_log"
rg -F -q 'rejects=1,2,3,4,5,6,7' "$vm_log"
rg -F -q 'counts=8,1,7,1,1,1,1,1,1,1,7,0' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$vm_log"
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
    "Main.makeBinding/2",
    "HakoAllocSegmentArenaBackingModeledArenaSlotInventory.recordArenaSlot/6",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingModeledArenaSlotReport")
if report is None:
    raise SystemExit("missing modeled arena-slot report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "accepted",
    "reason",
    "arena_slot_present",
    "modeled_arena_slot_present",
    "row_index",
    "segment_id",
    "arena_id",
    "residence_token",
    "binding_token",
    "arena_slot_token",
    "slot_index",
    "requested_bytes",
    "padded_bytes",
    "slot_capacity",
    "closed_substrate_blocker_count",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing modeled arena-slot field: {name}")

print("[mimap256a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

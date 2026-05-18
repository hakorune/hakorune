#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-source-bridge"
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
    echo "[$TAG] ERROR: MIMAP-260A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-source-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-source-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-source-bridge-proof/test.sh"
CARD_258A="docs/development/current/main/phases/phase-293x/293x-781-MIMAP-258A-SEGMENT-ARENA-BACKING-MODELED-ARENA-SLOT-CLOSEOUT-PACK.md"
CARD_259A="docs/development/current/main/phases/phase-293x/293x-782-MIMAP-259A-POST-SEGMENT-ARENA-BACKING-MODELED-ARENA-SLOT-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-783-MIMAP-260A-SEGMENT-ARENA-BACKING-MODELED-SOURCE-BRIDGE-INVENTORY.md"
SLOT_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-arena-slot-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-source-bridge-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
SLOT_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_arena_slot_box.hako"
OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_source_bridge_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_guard.sh"

printf '[%s] checking MIMAP-260A segment arena backing modeled source bridge\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_258A" \
  "$CARD_259A" \
  "$CARD" \
  "$SLOT_SSOT" \
  "$DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$SLOT_OWNER" \
  "$OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_258A" "MIMAP-258A closeout must be landed before source bridge"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_259A" "MIMAP-259A selection must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-260A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$SLOT_SSOT" "MIMAP-256A arena-slot design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-260A source bridge design must be accepted"
guard_expect_in_file "$TAG" 'modeled source bridge accepted' "$DESIGN" "MIMAP-260A reason vocabulary must be present"
guard_expect_in_file "$TAG" 'MIMAP-260A segment arena backing modeled source bridge inventory' "$PLAN" "granularity SSOT must describe MIMAP-260A"
guard_expect_in_file "$TAG" 'MIMAP-260A segment arena backing modeled source bridge inventory' "$JOINT" "joint order must name MIMAP-260A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-260A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-260A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-260A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-260A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-260A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_source_bridge_box' "$MODULE" "module must export source bridge owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_source_bridge_box.hako' "$MEMORY_README" "memory README must name source bridge owner"
guard_expect_in_file "$TAG" 'recordSourceBridge' "$OWNER" "source bridge owner must expose record route"
guard_expect_in_file "$TAG" 'check "mimap260a segment arena backing modeled source bridge"' "$APP" "proof must use labelled check block"

if rg -n 'lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-260A must keep pointer lookup/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-source-bridge-proof|ModeledSourceBridge|modeledSourceBridge' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-260A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap260_source_bridge.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap260.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-source-bridge-proof' "$vm_log"
rg -F -q 'source=1,0,1,70,7,3,70007004005,1,90007004005,16384,4096,16' "$vm_log"
rg -F -q 'slot=2,2048,4096,8192,16,1' "$vm_log"
rg -F -q 'rejects=1,2,3,4,5,6' "$vm_log"
rg -F -q 'counts=7,1,6,1,1,1,1,1,1,6,0' "$vm_log"
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
    "Main.makeSlot/2",
    "HakoAllocSegmentArenaBackingModeledSourceBridgeInventory.recordSourceBridge/6",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingModeledSourceBridgeReport")
if report is None:
    raise SystemExit("missing modeled source bridge report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "accepted",
    "reason",
    "source_bridge_present",
    "modeled_source_bridge_present",
    "arena_slot_token",
    "source_kind",
    "source_token",
    "source_capacity",
    "source_committed_bytes",
    "source_alignment",
    "closed_substrate_blocker_count",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing modeled source bridge field: {name}")

print("[mimap260a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

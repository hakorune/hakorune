#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-arena-slot-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-257A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-arena-slot-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-arena-slot-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-arena-slot-diagnostics-proof/test.sh"
CARD_256A="docs/development/current/main/phases/phase-293x/293x-779-MIMAP-256A-SEGMENT-ARENA-BACKING-MODELED-ARENA-SLOT-INVENTORY.md"
CARD="docs/development/current/main/phases/phase-293x/293x-780-MIMAP-257A-SEGMENT-ARENA-BACKING-MODELED-ARENA-SLOT-DIAGNOSTICS.md"
DESIGN_256A="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-arena-slot-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-arena-slot-diagnostics-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
INVENTORY_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_arena_slot_box.hako"
DIAGNOSTIC_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_arena_slot_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_diagnostics_guard.sh"

printf '[%s] checking MIMAP-257A segment arena backing modeled arena slot diagnostics\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_256A" \
  "$CARD" \
  "$DESIGN_256A" \
  "$DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$CADENCE" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$INVENTORY_OWNER" \
  "$DIAGNOSTIC_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_256A" "MIMAP-256A inventory must be landed before diagnostics"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-257A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_256A" "MIMAP-256A arena-slot design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-257A diagnostics design must be accepted"
guard_expect_in_file "$TAG" 'observer-only diagnostics' "$CARD" "MIMAP-257A card must call out observer-only diagnostics"
guard_expect_in_file "$TAG" 'MIMAP-257A granularity' "$PLAN" "granularity SSOT must describe MIMAP-257A"
guard_expect_in_file "$TAG" 'MIMAP-257A segment arena backing modeled arena slot diagnostics' "$JOINT" "joint order must name MIMAP-257A"
guard_expect_in_file "$TAG" 'segment-arena-backing-modeled-arena-slot' "$CADENCE" "cadence SSOT must define arena-slot family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-257A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-257A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-257A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-257A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-257A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_arena_slot_diagnostic_box' "$MODULE" "module must export diagnostic owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_arena_slot_diagnostic_box.hako' "$MEMORY_README" "memory README must name diagnostic owner"
guard_expect_in_file "$TAG" 'observeArenaSlotDiagnostics' "$DIAGNOSTIC_OWNER" "diagnostic owner must expose observer route"
guard_expect_in_file "$TAG" 'diagnostic_present: i64 = 1' "$DIAGNOSTIC_OWNER" "diagnostic report must publish presence bit"
guard_expect_in_file "$TAG" 'check "mimap257a segment arena backing modeled arena slot diagnostics"' "$APP" "proof must use labelled check block"

if rg -n 'recordArenaSlot|me\.(inventory_count|accepted_count|reject_count|missing_binding_reject_count|rejected_binding_reject_count|invalid_binding_token_reject_count|invalid_residence_token_reject_count|invalid_geometry_reject_count|invalid_slot_shape_reject_count|closed_substrate_reject_count)[[:space:]]*\+=' \
  "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".mutation_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-257A diagnostic owner must not record arena-slot rows or mutate inventory counters" >&2
  cat /tmp/"$TAG".mutation_leak >&2
  rm -f /tmp/"$TAG".mutation_leak
  exit 1
fi
rm -f /tmp/"$TAG".mutation_leak

if rg -n 'lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-257A must keep pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-arena-slot-diagnostics-proof|ModeledArenaSlotDiagnostic|modeledArenaSlotDiagnostic' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-257A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap257_arena_slot_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap257.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-arena-slot-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,7,8,1,7,1,7,70,7,1' "$vm_log"
rg -F -q 'seen=1,1,1,1,1,1,1' "$vm_log"
rg -F -q 'owner=2,1,1,1,1' "$vm_log"
rg -F -q 'empty=0,1,0' "$vm_log"
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
    "HakoAllocSegmentArenaBackingModeledArenaSlotDiagnostic.observeArenaSlotDiagnostics/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingModeledArenaSlotDiagnosticReport")
if report is None:
    raise SystemExit("missing modeled arena-slot diagnostic report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "observed",
    "reason",
    "diagnostic_present",
    "inventory_count",
    "accepted_count",
    "reject_count",
    "closed_substrate_reject_seen",
    "arena_slot_present",
    "modeled_arena_slot_present",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing modeled arena-slot diagnostic field: {name}")

print("[mimap257a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-readiness-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-237A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-readiness-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-readiness-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-readiness-diagnostics-proof/test.sh"
CARD_236A="docs/development/current/main/phases/phase-293x/293x-759-MIMAP-236A-SEGMENT-ARENA-BACKING-READINESS-INVENTORY.md"
CARD="docs/development/current/main/phases/phase-293x/293x-760-MIMAP-237A-SEGMENT-ARENA-BACKING-READINESS-DIAGNOSTICS.md"
DESIGN_236A="docs/development/current/main/design/hako-alloc-segment-arena-backing-readiness-inventory-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-readiness-diagnostics-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
INVENTORY_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_readiness_inventory_box.hako"
DIAGNOSTIC_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_readiness_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_readiness_diagnostics_guard.sh"

printf '[%s] checking MIMAP-237A segment arena backing readiness diagnostics\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_236A" \
  "$CARD" \
  "$DESIGN_236A" \
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

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_236A" "MIMAP-236A inventory must be landed before diagnostics"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-237A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_236A" "MIMAP-236A inventory design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-237A diagnostics design must be accepted"
guard_expect_in_file "$TAG" 'observer-only diagnostics' "$CARD" "MIMAP-237A card must call out observer-only diagnostics"
guard_expect_in_file "$TAG" 'MIMAP-237A' "$PLAN" "granularity SSOT must describe MIMAP-237A"
guard_expect_in_file "$TAG" 'MIMAP-238A' "$PLAN" "granularity SSOT must describe MIMAP-238A"
guard_expect_in_file "$TAG" 'MIMAP-237A segment arena backing readiness diagnostics' "$JOINT" "joint order must name MIMAP-237A"
guard_expect_in_file "$TAG" 'segment arena backing readiness rows' "$CADENCE" "cadence SSOT must define arena readiness family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-237A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-237A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-237A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-237A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-237A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_readiness_diagnostic_box' "$MODULE" "module must export diagnostic owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_readiness_diagnostic_box.hako' "$MEMORY_README" "memory README must name diagnostic owner"
guard_expect_in_file "$TAG" 'observeReadinessDiagnostics' "$DIAGNOSTIC_OWNER" "diagnostic owner must expose observer route"
guard_expect_in_file "$TAG" 'diagnostic_present: i64 = 1' "$DIAGNOSTIC_OWNER" "diagnostic report must publish presence bit"
guard_expect_in_file "$TAG" 'check "mimap237a segment arena backing readiness diagnostics"' "$APP" "proof must use labelled check block"

if rg -n 'classifyReadiness|me\.(accepted_count|reject_count)[[:space:]]*\+=' \
  "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".mutation_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-237A diagnostic owner must not classify readiness or mutate inventory counters" >&2
  cat /tmp/"$TAG".mutation_leak >&2
  rm -f /tmp/"$TAG".mutation_leak
  exit 1
fi
rm -f /tmp/"$TAG".mutation_leak

if rg -n 'allocateArena|ArenaBackingAlloc|arenaBackingAllocate|rawPointer|pointer_member|lookupSegment[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-237A must keep arena/raw/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-readiness-diagnostics-proof|SegmentArenaBackingReadinessDiagnostic|arenaBackingReadinessDiagnostic' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-237A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap237_arena_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap237.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-readiness-diagnostics-proof' "$vm_log"
rg -F -q 'base=70007004,70007004002,1' "$vm_log"
rg -F -q 'diag=1,8,9,1,8,6,1' "$vm_log"
rg -F -q 'rejects=1,1,1,1,1,1,1,1' "$vm_log"
rg -F -q 'last=8,70,7,0,8' "$vm_log"
rg -F -q 'observer=2,1,1,1,1' "$vm_log"
rg -F -q 'empty=0,1,0' "$vm_log"
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
    "HakoAllocSegmentArenaBackingReadinessInventory.classifyReadiness/14",
    "HakoAllocSegmentArenaBackingReadinessDiagnostic.observeReadinessDiagnostics/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingReadinessDiagnosticReport")
if report is None:
    raise SystemExit("missing arena backing readiness diagnostic report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "observed",
    "reason",
    "diagnostic_present",
    "blocked_requirement_reject_count",
    "continuation_reject_seen",
    "invalid_shape_reject_seen",
    "arena_requirement_reject_seen",
    "raw_pointer_requirement_reject_seen",
    "segment_map_requirement_reject_seen",
    "atomic_bitmap_requirement_reject_seen",
    "osvm_requirement_reject_seen",
    "provider_requirement_reject_seen",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing arena backing readiness diagnostic report field: {name}")

print("[mimap237a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

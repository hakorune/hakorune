#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger"
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
    echo "[$TAG] ERROR: MIMAP-276A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-proof/test.sh"
CARD_272A="docs/development/current/main/phases/phase-293x/293x-797-MIMAP-272A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-APPLY-INVENTORY.md"
CARD_274A="docs/development/current/main/phases/phase-293x/293x-799-MIMAP-274A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-APPLY-CLOSEOUT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-801-MIMAP-276A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-INVENTORY.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
APPLY_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_apply_box.hako"
LEDGER_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_guard.sh"

printf '[%s] checking MIMAP-276A segment arena backing modeled allocation ledger\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_272A" \
  "$CARD_274A" \
  "$CARD" \
  "$DESIGN" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$APPLY_OWNER" \
  "$LEDGER_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_272A" "MIMAP-272A allocation apply must be landed before ledger"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_274A" "MIMAP-274A allocation apply closeout must be landed before ledger"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-276A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-276A allocation ledger design must be accepted"
guard_expect_in_file "$TAG" 'modeled allocation ledger' "$CARD" "MIMAP-276A card must call out modeled allocation ledger"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-276A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-276A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-276A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-276A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-276A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_allocation_ledger_box' "$MODULE" "module must export allocation ledger owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_ledger_box.hako' "$MEMORY_README" "memory README must name allocation ledger owner"
guard_expect_in_file "$TAG" 'recordAllocationLedger' "$LEDGER_OWNER" "allocation ledger owner must expose record route"
guard_expect_in_file "$TAG" 'duplicate_ledger_token_reject_count' "$LEDGER_OWNER" "allocation ledger owner must reject duplicate ledger token"
guard_expect_in_file "$TAG" 'source_capacity: usize = 0' "$LEDGER_OWNER" "allocation ledger source capacity must be exact usize"
guard_expect_in_file "$TAG" 'applied_backing_bytes: usize = 0' "$LEDGER_OWNER" "allocation ledger applied backing bytes must be exact usize"
guard_expect_in_file "$TAG" 'remaining_source_bytes: usize = 0' "$LEDGER_OWNER" "allocation ledger remaining source bytes must be exact usize"
guard_expect_in_file "$TAG" 'ledger_token: i64 = 0' "$LEDGER_OWNER" "allocation ledger token must remain i64"
guard_expect_in_file "$TAG" 'row_index: i64 = -1' "$LEDGER_OWNER" "allocation ledger row sentinel must remain i64"
guard_expect_in_file "$TAG" 'check "mimap276a segment arena backing modeled allocation ledger"' "$APP" "proof must use labelled check block"

if rg -n 'lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$LEDGER_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-276A must keep pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-proof|ModeledAllocationLedger|modeledAllocationLedger' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-276A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap276_allocation_ledger.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap276.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-proof' "$vm_log"
rg -F -q 'ledger=1,0,1,120,12,3,70012005005,1,90012005005,91012005005,92012005005,93012005005,4096,4096,12288,8192' "$vm_log"
rg -F -q 'rejects=1,2,3,4,5' "$vm_log"
rg -F -q 'counts=6,1,5,1,1,1,1,1,5,93012005005' "$vm_log"
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
    "Main.makeApply/2",
    "HakoAllocSegmentArenaBackingModeledAllocationApplyInventory.recordAllocationApply/4",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerInventory.recordAllocationLedger/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingModeledAllocationLedgerReport")
if report is None:
    raise SystemExit("missing modeled allocation ledger report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
required_fields = (
    "accepted",
    "reason",
    "allocation_ledger_present",
    "modeled_allocation_ledger_present",
    "ledger_token",
    "apply_token",
    "applied_backing_bytes",
    "applied_committed_bytes",
    "remaining_source_bytes",
    "would_add_backend_matcher",
)
for name in required_fields:
    if name not in fields:
        raise SystemExit(f"missing modeled allocation ledger field: {name}")

usize_fields = {
    "source_capacity",
    "source_committed_bytes",
    "source_uncommitted_bytes",
    "padded_bytes",
    "slot_capacity",
    "planned_backing_bytes",
    "planned_committed_bytes",
    "applied_backing_bytes",
    "applied_committed_bytes",
    "remaining_source_bytes",
}
for name in usize_fields:
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"allocation ledger {name} must be exact usize storage: {field}")

for name in ("reason", "row_index", "ledger_token"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"allocation ledger {name} must remain i64 storage: {field}")

print("[mimap276a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

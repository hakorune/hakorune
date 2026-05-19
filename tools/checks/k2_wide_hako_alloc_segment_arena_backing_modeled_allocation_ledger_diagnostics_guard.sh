#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-277A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-diagnostics-proof/test.sh"
CARD_276A="docs/development/current/main/phases/phase-293x/293x-801-MIMAP-276A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-INVENTORY.md"
CARD="docs/development/current/main/phases/phase-293x/293x-802-MIMAP-277A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-DIAGNOSTICS.md"
DESIGN_276A="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
INVENTORY_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_box.hako"
DIAGNOSTIC_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_diagnostics_guard.sh"

printf '[%s] checking MIMAP-277A segment arena backing modeled allocation ledger diagnostics\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_276A" \
  "$CARD" \
  "$DESIGN_276A" \
  "$DESIGN" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$INVENTORY_OWNER" \
  "$DIAGNOSTIC_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_276A" "MIMAP-276A allocation ledger inventory must be landed before diagnostics"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-277A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_276A" "MIMAP-276A allocation ledger design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-277A diagnostics design must be accepted"
guard_expect_in_file "$TAG" 'observer-only' "$CARD" "MIMAP-277A card must call out observer-only diagnostics"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-277A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-277A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-277A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-277A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-277A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_allocation_ledger_diagnostic_box' "$MODULE" "module must export diagnostic owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_ledger_diagnostic_box.hako' "$MEMORY_README" "memory README must name diagnostic owner"
guard_expect_in_file "$TAG" 'record HakoAllocSegmentArenaBackingModeledAllocationLedgerDiagnosticReportFields' "$DIAGNOSTIC_OWNER" "diagnostic owner must use local ReportFields record payload"
guard_expect_in_file "$TAG" 'observeAllocationLedgerDiagnostics' "$DIAGNOSTIC_OWNER" "diagnostic owner must expose observer route"
guard_expect_in_file "$TAG" 'diagnostic_present: i64 = 1' "$DIAGNOSTIC_OWNER" "diagnostic report must publish presence bit"
guard_expect_in_file "$TAG" 'check "mimap277a segment arena backing modeled allocation ledger diagnostics"' "$APP" "proof must use labelled check block"

if rg -n 'recordAllocationLedger|me\.(inventory_count|accepted_count|reject_count|missing_apply_reject_count|rejected_apply_reject_count|invalid_ledger_token_reject_count|duplicate_ledger_token_reject_count|closed_substrate_reject_count)[[:space:]]*\+=' \
  "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".mutation_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-277A diagnostic owner must not record allocation-ledger rows or mutate inventory counters" >&2
  cat /tmp/"$TAG".mutation_leak >&2
  rm -f /tmp/"$TAG".mutation_leak
  exit 1
fi
rm -f /tmp/"$TAG".mutation_leak

if rg -n 'lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-277A must keep pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-diagnostics-proof|ModeledAllocationLedgerDiagnostic|modeledAllocationLedgerDiagnostic' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-277A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap277_allocation_ledger_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap277.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,5,6,1,5,1,5,130,13,1' "$vm_log"
rg -F -q 'seen=1,1,1,1,1' "$vm_log"
rg -F -q 'ledger=92013005005,0,4096,4096,8192' "$vm_log"
rg -F -q 'present=1,1,1' "$vm_log"
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
    "Main.makeApply/2",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerInventory.recordAllocationLedger/2",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerDiagnostic.observeAllocationLedgerDiagnostics/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
record_names = set()
for decl in data.get("record_decls", []):
    if isinstance(decl, str):
        record_names.add(decl)
    elif isinstance(decl, dict):
        record_names.add(decl.get("name"))
if "HakoAllocSegmentArenaBackingModeledAllocationLedgerDiagnosticReportFields" not in record_names:
    raise SystemExit("missing modeled allocation ledger diagnostic ReportFields record declaration")

report = plans.get("HakoAllocSegmentArenaBackingModeledAllocationLedgerDiagnosticReport")
if report is None:
    raise SystemExit("missing modeled allocation ledger diagnostic report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "observed",
    "reason",
    "diagnostic_present",
    "inventory_count",
    "accepted_count",
    "reject_count",
    "closed_substrate_reject_seen",
    "allocation_ledger_present",
    "modeled_allocation_ledger_present",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing modeled allocation ledger diagnostic field: {name}")

print("[mimap277a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

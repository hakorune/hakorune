#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker"
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
    echo "[$TAG] ERROR: MIMAP-316A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker-proof/test.sh"
CARD_314A="docs/development/current/main/phases/phase-293x/293x-929-MIMAP-314A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-READINESS-MATRIX-CLOSEOUT.md"
CARD_315A="docs/development/current/main/phases/phase-293x/293x-930-MIMAP-315A-POST-RELEASE-RECYCLE-EXECUTION-READINESS-MATRIX-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-931-MIMAP-316A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-RECYCLE-EXECUTION-INTENT-MARKER-PREFLIGHT.md"
DESIGN_312A="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-readiness-matrix-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MATRIX_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_box.hako"
INTENT_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_intent_marker_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_intent_marker_guard.sh"

printf '[%s] checking MIMAP-316A segment arena backing modeled allocation-ledger release/recycle execution intent marker\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_314A" "$CARD_315A" "$CARD" "$DESIGN_312A" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$MATRIX_OWNER" "$INTENT_OWNER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_314A" "MIMAP-314A closeout must be landed before execution intent marker"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_315A" "MIMAP-315A selection must be landed before execution intent marker"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-316A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_312A" "MIMAP-312A matrix design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-316A design must be accepted"
guard_expect_in_file "$TAG" 'execution remains unsupported' "$DESIGN" "MIMAP-316A design must keep execution unsupported"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-316A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-316A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-316A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-316A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-316A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_intent_marker_box' "$MODULE" "module must export execution intent marker owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_intent_marker_box.hako' "$MEMORY_README" "memory README must name execution intent marker owner"
guard_expect_in_file "$TAG" 'record HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionIntentMarkerReportFields' "$INTENT_OWNER" "intent owner must use local ReportFields record payload"
guard_expect_in_file "$TAG" 'makeExecutionIntentMarkerReport' "$INTENT_OWNER" "intent owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'recordExecutionIntentMarker' "$INTENT_OWNER" "intent owner must expose marker route"
guard_expect_in_file "$TAG" 'execution_supported: i64 = 0' "$INTENT_OWNER" "execution must remain unsupported"
guard_expect_in_file "$TAG" 'unsupported_execution_requirement: i64 = 1' "$INTENT_OWNER" "unsupported execution requirement must be explicit"
guard_expect_in_file "$TAG" 'applied_backing_bytes: usize' "$INTENT_OWNER" "intent report must mirror backing bytes as usize"
guard_expect_in_file "$TAG" 'applied_committed_bytes: usize' "$INTENT_OWNER" "intent report must mirror committed bytes as usize"
guard_expect_in_file "$TAG" 'remaining_source_bytes: usize' "$INTENT_OWNER" "intent report must mirror remaining bytes as usize"

if rg -n 'realLifecycle|generationToken|lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' "$INTENT_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-316A intent owner must keep lifecycle/pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker-proof|ModeledAllocationLedgerReleaseRecycleExecutionIntentMarker|modeledAllocationLedgerReleaseRecycleExecutionIntentMarker' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-316A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap316_execution_intent_marker.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap316.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker-proof' "$vm_log"
rg -F -q 'intent=1,0,1,1,1,0,1' "$vm_log"
rg -F -q 'tokens=97019005005,98019005005,99019005005' "$vm_log"
rg -F -q 'bytes=4096,4096,8192' "$vm_log"
rg -F -q 'owner=5,1,4,1,1,1,1,1,4' "$vm_log"
rg -F -q 'rejected=1,2,3,4,8' "$vm_log"
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

with open(sys.argv[1], encoding="utf-8") as fh:
    data = json.load(fh)
functions = {fn.get("name"): fn for fn in data.get("functions", [])}
required = {
    "main",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionIntentMarker.makeExecutionIntentMarkerReport/1",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionIntentMarker.recordExecutionIntentMarker/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionIntentMarkerReport")
if report is None:
    raise SystemExit("missing execution intent marker report typed object plan")
target = "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionIntentMarkerReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing execution intent marker ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in ("applied_backing_bytes", "applied_committed_bytes", "remaining_source_bytes"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"{name} must be exact usize storage: {field}")
for name in ("execution_supported", "unsupported_execution_requirement"):
    if name not in fields:
        raise SystemExit(f"missing execution intent marker field: {name}")
print("[mimap316a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

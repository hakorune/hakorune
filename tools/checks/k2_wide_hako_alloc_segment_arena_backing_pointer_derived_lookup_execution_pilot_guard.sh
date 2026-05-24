#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-pointer-derived-lookup-execution-pilot"
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
    echo "[$TAG] ERROR: MIMAP-346A defers L3/L4 evidence to a closeout or backend-facing route change" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-pointer-derived-lookup-execution-pilot-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-pointer-derived-lookup-execution-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-pointer-derived-lookup-execution-pilot-proof/test.sh"
CARD_345A="docs/development/current/main/phases/phase-293x/293x-960-MIMAP-345A-ARENA-BACKING-HANDLE-PILOT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-961-MIMAP-346A-POINTER-DERIVED-LOOKUP-EXECUTION-PILOT.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-pointer-derived-lookup-execution-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE_INDEX="lang/src/hako_alloc/memory/MODULE_INDEX.md"
OWNER="lang/src/hako_alloc/memory/segment_arena_backing_pointer_derived_lookup_execution_pilot_box.hako"
HANDLE_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_handle_pilot_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_pointer_derived_lookup_execution_pilot_guard.sh"

printf '[%s] checking MIMAP-346A segment arena backing pointer-derived lookup execution pilot\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_345A" "$CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$MODULE_INDEX" "$OWNER" "$HANDLE_OWNER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_345A" "MIMAP-345A arena backing handle pilot must be landed before pointer-derived lookup pilot"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$CARD" "MIMAP-346A card must be current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-346A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-346A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-346A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-346A"
guard_expect_in_file "$TAG" 'row_kind = "first-real-seam"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-346A must be marked as first-real-seam"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-346A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_pointer_derived_lookup_execution_pilot_box' "$MODULE" "module must export pointer-derived lookup owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_pointer_derived_lookup_execution_pilot_box.hako' "$MODULE_INDEX" "memory module index must name pointer-derived lookup owner"
guard_expect_in_file "$TAG" 'record HakoAllocSegmentArenaBackingPointerDerivedLookupExecutionPilotReportFields' "$OWNER" "lookup owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makePointerDerivedLookupExecutionPilotReport' "$OWNER" "lookup owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'recordPointerDerivedLookup' "$OWNER" "lookup owner must expose bounded pointer-derived lookup route"
guard_expect_in_file "$TAG" 'lookup_count: usize = 0' "$OWNER" "lookup owner-local counter must be exact usize"
guard_expect_in_file "$TAG" 'accepted_count: usize = 0' "$OWNER" "lookup owner-local accepted counter must be exact usize"
guard_expect_in_file "$TAG" 'invalid_lookup_reject_count: usize = 0' "$OWNER" "lookup owner-local reject counters must be exact usize"
guard_expect_in_file "$TAG" 'lookup_count: i64' "$OWNER" "lookup report mirror must stay signed in this row"
guard_expect_in_file "$TAG" 'lookup_result_token: i64' "$OWNER" "lookup report must carry lookup result token"
guard_expect_in_file "$TAG" 'report_applied_backing_bytes: usize' "$OWNER" "lookup report must mirror backing bytes as usize"
guard_expect_in_file "$TAG" 'would_dereference: 0' "$OWNER" "lookup owner must explicitly keep dereference closed"

if rg -n 'pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-346A owner must keep deref/arena-release/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'pointer-derived-lookup-execution-pilot-proof|PointerDerivedLookupExecutionPilot|pointerDerivedLookupExecutionPilot' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-346A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap346_pointer_lookup.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap346.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-segment-arena-backing-pointer-derived-lookup-execution-pilot-proof' "$vm_log"
rg -F -q 'lookup=1,0,1,1,1,99019005044,1,99019005033,99019005022' "$vm_log"
rg -F -q 'owner=7,1,6,1,1,1,1,1,1,6' "$vm_log"
rg -F -q 'tokens=97019005005,98019005005,99019005005' "$vm_log"
rg -F -q 'bytes=4096,4096,8192' "$vm_log"
rg -F -q 'rejected=1,2,3,4,5,6' "$vm_log"
rg -F -q 'closed=0,0,0,0,0,0,0,0,0' "$vm_log"
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
    "HakoAllocSegmentArenaBackingPointerDerivedLookupExecutionPilot.makePointerDerivedLookupExecutionPilotReport/1",
    "HakoAllocSegmentArenaBackingPointerDerivedLookupExecutionPilot.recordPointerDerivedLookup/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
owner = plans.get("HakoAllocSegmentArenaBackingPointerDerivedLookupExecutionPilot")
if owner is None:
    raise SystemExit("missing pointer-derived lookup execution pilot owner typed object plan")
owner_fields = {field.get("name"): field for field in owner.get("fields", [])}
field = owner_fields.get("lookup_count")
if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
    raise SystemExit(f"owner lookup_count must be exact usize storage: {field}")
for name in (
    "accepted_count",
    "reject_count",
    "missing_handle_reject_count",
    "rejected_handle_reject_count",
    "invalid_pointer_token_reject_count",
    "invalid_handle_reject_count",
    "invalid_lookup_reject_count",
    "closed_execution_reject_count",
):
    field = owner_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"{name} must be exact usize owner storage: {field}")
field = owner_fields.get("last_reason")
if field is None or field.get("declared_type") != "i64":
    raise SystemExit(f"last_reason must remain i64 owner storage: {field}")
report = plans.get("HakoAllocSegmentArenaBackingPointerDerivedLookupExecutionPilotReport")
if report is None:
    raise SystemExit("missing pointer-derived lookup execution pilot report typed object plan")
target = "HakoAllocSegmentArenaBackingPointerDerivedLookupExecutionPilotReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing pointer-derived lookup execution pilot ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in ("report_applied_backing_bytes", "report_applied_committed_bytes", "report_remaining_source_bytes"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"{name} must be exact usize storage: {field}")
for name in ("lookup_result_token", "lookup_result_valid", "arena_handle_token"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
field = fields.get("lookup_count")
if field is None or field.get("declared_type") != "i64":
    raise SystemExit(f"report lookup_count mirror must remain i64: {field}")
print("[mimap346a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

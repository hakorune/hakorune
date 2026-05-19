#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-apply-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-273A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-apply-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-allocation-apply-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-allocation-apply-diagnostics-proof/test.sh"
CARD_272A="docs/development/current/main/phases/phase-293x/293x-797-MIMAP-272A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-APPLY-INVENTORY.md"
CARD="docs/development/current/main/phases/phase-293x/293x-798-MIMAP-273A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-APPLY-DIAGNOSTICS.md"
DESIGN_272A="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-apply-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-apply-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
INVENTORY_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_apply_box.hako"
DIAGNOSTIC_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_apply_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_diagnostics_guard.sh"

printf '[%s] checking MIMAP-273A segment arena backing modeled allocation apply diagnostics\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_272A" \
  "$CARD" \
  "$DESIGN_272A" \
  "$DESIGN" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$INVENTORY_OWNER" \
  "$DIAGNOSTIC_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_272A" "MIMAP-272A allocation apply inventory must be landed before diagnostics"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-273A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_272A" "MIMAP-272A allocation apply design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-273A diagnostics design must be accepted"
guard_expect_in_file "$TAG" 'observer-only diagnostics' "$CARD" "MIMAP-273A card must call out observer-only diagnostics"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-273A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-273A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-273A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-273A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-273A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_allocation_apply_diagnostic_box' "$MODULE" "module must export diagnostic owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_apply_diagnostic_box.hako' "$MEMORY_README" "memory README must name diagnostic owner"
guard_expect_in_file "$TAG" 'record HakoAllocSegmentArenaBackingModeledAllocationApplyDiagnosticReportFields' "$DIAGNOSTIC_OWNER" "diagnostic owner must use local ReportFields record payload"
guard_expect_in_file "$TAG" 'observeAllocationApplyDiagnostics' "$DIAGNOSTIC_OWNER" "diagnostic owner must expose observer route"
guard_expect_in_file "$TAG" 'diagnostic_present: i64 = 1' "$DIAGNOSTIC_OWNER" "diagnostic report must publish presence bit"
guard_expect_in_file "$TAG" 'last_report_applied_backing_bytes: i64' "$DIAGNOSTIC_OWNER" "allocation-apply diagnostic mirror bytes must remain i64 in HAKO-ALLOC-USIZE-FIELD-GROUP-010"
guard_expect_in_file "$TAG" 'last_report_applied_committed_bytes: i64' "$DIAGNOSTIC_OWNER" "allocation-apply diagnostic mirror committed bytes must remain i64 in HAKO-ALLOC-USIZE-FIELD-GROUP-010"
guard_expect_in_file "$TAG" 'last_report_remaining_source_bytes: i64' "$DIAGNOSTIC_OWNER" "allocation-apply diagnostic mirror remaining bytes must remain i64 in HAKO-ALLOC-USIZE-FIELD-GROUP-010"
guard_expect_in_file "$TAG" 'check "mimap273a segment arena backing modeled allocation apply diagnostics"' "$APP" "proof must use labelled check block"

if rg -n 'recordAllocationApply|me\.(inventory_count|accepted_count|reject_count|missing_plan_reject_count|rejected_plan_reject_count|invalid_apply_token_reject_count|invalid_apply_geometry_reject_count|closed_substrate_reject_count)[[:space:]]*\+=' \
  "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".mutation_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-273A diagnostic owner must not record allocation-apply rows or mutate inventory counters" >&2
  cat /tmp/"$TAG".mutation_leak >&2
  rm -f /tmp/"$TAG".mutation_leak
  exit 1
fi
rm -f /tmp/"$TAG".mutation_leak

if rg -n 'lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-273A must keep pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-allocation-apply-diagnostics-proof|ModeledAllocationApplyDiagnostic|modeledAllocationApplyDiagnostic' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-273A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap273_allocation_apply_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap273.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-apply-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,5,6,1,5,1,5,110,11,1' "$vm_log"
rg -F -q 'seen=1,1,1,1,1' "$vm_log"
rg -F -q 'apply=91011005005,0,0,0,0' "$vm_log"
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
    "Main.makePlan/2",
    "HakoAllocSegmentArenaBackingModeledAllocationApplyInventory.recordAllocationApply/4",
    "HakoAllocSegmentArenaBackingModeledAllocationApplyDiagnostic.observeAllocationApplyDiagnostics/2",
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
if "HakoAllocSegmentArenaBackingModeledAllocationApplyDiagnosticReportFields" not in record_names:
    raise SystemExit("missing modeled allocation apply diagnostic ReportFields record declaration")

report = plans.get("HakoAllocSegmentArenaBackingModeledAllocationApplyDiagnosticReport")
if report is None:
    raise SystemExit("missing modeled allocation apply diagnostic report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
required_fields = (
    "observed",
    "reason",
    "diagnostic_present",
    "inventory_count",
    "accepted_count",
    "reject_count",
    "closed_substrate_reject_seen",
    "allocation_apply_present",
    "modeled_allocation_apply_present",
    "would_add_backend_matcher",
)
for name in required_fields:
    if name not in fields:
        raise SystemExit(f"missing modeled allocation apply diagnostic field: {name}")

for name in (
    "last_report_applied_backing_bytes",
    "last_report_applied_committed_bytes",
    "last_report_remaining_source_bytes",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"allocation apply diagnostic mirror {name} must remain i64 storage: {field}")

record_decl = None
for decl in data.get("record_decls", []):
    if isinstance(decl, dict) and decl.get("name") == "HakoAllocSegmentArenaBackingModeledAllocationApplyDiagnosticReportFields":
        record_decl = decl
        break
if record_decl is None:
    raise SystemExit("missing allocation apply diagnostic ReportFields record details")

record_fields = {
    field.get("name"): field
    for field in record_decl.get("field_decls", [])
}
for name in (
    "last_report_applied_backing_bytes",
    "last_report_applied_committed_bytes",
    "last_report_remaining_source_bytes",
):
    field = record_fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"allocation apply diagnostic ReportFields {name} must remain declared i64: {field}")

print("[mimap273a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

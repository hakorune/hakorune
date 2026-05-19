#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-source-accounting-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-265A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-source-accounting-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-source-accounting-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-source-accounting-diagnostics-proof/test.sh"
CARD_264A="docs/development/current/main/phases/phase-293x/293x-787-MIMAP-264A-SEGMENT-ARENA-BACKING-MODELED-SOURCE-ACCOUNTING-INVENTORY.md"
CARD="docs/development/current/main/phases/phase-293x/293x-788-MIMAP-265A-SEGMENT-ARENA-BACKING-MODELED-SOURCE-ACCOUNTING-DIAGNOSTICS.md"
DESIGN_264A="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-source-accounting-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-source-accounting-diagnostics-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
INVENTORY_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_source_accounting_box.hako"
DIAGNOSTIC_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_source_accounting_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_accounting_diagnostics_guard.sh"

printf '[%s] checking MIMAP-265A segment arena backing modeled source accounting diagnostics\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_264A" \
  "$CARD" \
  "$DESIGN_264A" \
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

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_264A" "MIMAP-264A inventory must be landed before diagnostics"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-265A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_264A" "MIMAP-264A accounting design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-265A diagnostics design must be accepted"
guard_expect_in_file "$TAG" 'observer-only diagnostics' "$CARD" "MIMAP-265A card must call out observer-only diagnostics"
guard_expect_in_file "$TAG" 'MIMAP-265A granularity' "$PLAN" "granularity SSOT must describe MIMAP-265A"
guard_expect_in_file "$TAG" 'MIMAP-265A segment arena backing modeled source accounting diagnostics' "$JOINT" "joint order must name MIMAP-265A"
guard_expect_in_file "$TAG" 'MIMAP-265A segment arena backing modeled source accounting diagnostics' "$CADENCE" "cadence SSOT must name MIMAP-265A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-265A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-265A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-265A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-265A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-265A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_source_accounting_diagnostic_box' "$MODULE" "module must export diagnostic owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_source_accounting_diagnostic_box.hako' "$MEMORY_README" "memory README must name diagnostic owner"
guard_expect_in_file "$TAG" 'record HakoAllocSegmentArenaBackingModeledSourceAccountingDiagnosticReportFields' "$DIAGNOSTIC_OWNER" "diagnostic owner must use local ReportFields record payload"
guard_expect_in_file "$TAG" 'observeSourceAccountingDiagnostics' "$DIAGNOSTIC_OWNER" "diagnostic owner must expose observer route"
guard_expect_in_file "$TAG" 'diagnostic_present: i64 = 1' "$DIAGNOSTIC_OWNER" "diagnostic report must publish presence bit"
guard_expect_in_file "$TAG" 'last_report_source_capacity: i64' "$DIAGNOSTIC_OWNER" "source-accounting diagnostic capacity mirror must remain i64 in HAKO-ALLOC-USIZE-FIELD-GROUP-018"
guard_expect_in_file "$TAG" 'last_report_accounted_padded_bytes: i64' "$DIAGNOSTIC_OWNER" "source-accounting diagnostic padded mirror must remain i64 in HAKO-ALLOC-USIZE-FIELD-GROUP-018"
guard_expect_in_file "$TAG" 'last_report_available_after_padded_bytes: i64' "$DIAGNOSTIC_OWNER" "source-accounting diagnostic available mirror must remain i64 in HAKO-ALLOC-USIZE-FIELD-GROUP-018"
guard_expect_in_file "$TAG" 'check "mimap265a segment arena backing modeled source accounting diagnostics"' "$APP" "proof must use labelled check block"

if rg -n 'recordSourceAccounting|me\.(inventory_count|accepted_count|reject_count|missing_bridge_reject_count|rejected_bridge_reject_count|invalid_source_token_reject_count|invalid_accounting_geometry_reject_count|closed_substrate_reject_count)[[:space:]]*\+=' \
  "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".mutation_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-265A diagnostic owner must not record source accounting rows or mutate inventory counters" >&2
  cat /tmp/"$TAG".mutation_leak >&2
  rm -f /tmp/"$TAG".mutation_leak
  exit 1
fi
rm -f /tmp/"$TAG".mutation_leak

if rg -n 'lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-265A must keep pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-source-accounting-diagnostics-proof|ModeledSourceAccountingDiagnostic|modeledSourceAccountingDiagnostic' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-265A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap265_source_accounting_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap265.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-source-accounting-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,5,6,1,5,1,5,70,7,1' "$vm_log"
rg -F -q 'seen=1,1,1,1,1' "$vm_log"
rg -F -q 'account=0,16384,4096,0,0,0' "$vm_log"
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
    "Main.makeSource/2",
    "HakoAllocSegmentArenaBackingModeledSourceAccountingInventory.recordSourceAccounting/1",
    "HakoAllocSegmentArenaBackingModeledSourceAccountingDiagnostic.observeSourceAccountingDiagnostics/2",
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
if "HakoAllocSegmentArenaBackingModeledSourceAccountingDiagnosticReportFields" not in record_names:
    raise SystemExit("missing modeled source accounting diagnostic ReportFields record declaration")

report = plans.get("HakoAllocSegmentArenaBackingModeledSourceAccountingDiagnosticReport")
if report is None:
    raise SystemExit("missing modeled source accounting diagnostic report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "observed",
    "reason",
    "diagnostic_present",
    "inventory_count",
    "accepted_count",
    "reject_count",
    "closed_substrate_reject_seen",
    "source_accounting_present",
    "modeled_source_accounting_present",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing modeled source accounting diagnostic field: {name}")

for name in (
    "last_report_source_capacity",
    "last_report_source_committed_bytes",
    "last_report_source_uncommitted_bytes",
    "last_report_accounted_padded_bytes",
    "last_report_available_after_padded_bytes",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"source accounting diagnostic mirror {name} must remain i64 storage: {field}")

record_decl = None
for decl in data.get("record_decls", []):
    if isinstance(decl, dict) and decl.get("name") == "HakoAllocSegmentArenaBackingModeledSourceAccountingDiagnosticReportFields":
        record_decl = decl
        break
if record_decl is None:
    raise SystemExit("missing source accounting diagnostic ReportFields record details")

record_fields = {
    field.get("name"): field
    for field in record_decl.get("field_decls", [])
}
for name in (
    "last_report_source_capacity",
    "last_report_source_committed_bytes",
    "last_report_source_uncommitted_bytes",
    "last_report_accounted_padded_bytes",
    "last_report_available_after_padded_bytes",
):
    field = record_fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"source accounting diagnostic ReportFields {name} must remain declared i64: {field}")

print("[mimap265a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

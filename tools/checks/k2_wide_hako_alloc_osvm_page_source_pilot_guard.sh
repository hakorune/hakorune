#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-osvm-page-source-pilot"
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
    echo "[$TAG] ERROR: MIMAP-349A defers L3/L4 evidence to a closeout or provider-facing route change" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-osvm-page-source-pilot-proof/main.hako"
APP_README="apps/hako-alloc-osvm-page-source-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-osvm-page-source-pilot-proof/test.sh"
CARD_348A="docs/development/current/main/phases/phase-293x/293x-963-MIMAP-348A-ATOMIC-BITMAP-PILOT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-964-MIMAP-349A-OSVM-PAGE-SOURCE-PILOT.md"
DESIGN="docs/development/current/main/design/hako-alloc-osvm-page-source-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/osvm_page_source_pilot_box.hako"
BITMAP_OWNER="lang/src/hako_alloc/memory/atomic_bitmap_pilot_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_osvm_page_source_pilot_guard.sh"

printf '[%s] checking MIMAP-349A OSVM/page-source pilot\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_348A" "$CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$BITMAP_OWNER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_348A" "MIMAP-348A atomic bitmap pilot must be landed before OSVM/page-source pilot"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$CARD" "MIMAP-349A card must be current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-349A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-349A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-349A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-349A"
guard_expect_in_file "$TAG" 'row_kind = "first-real-seam"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-349A must be marked as first-real-seam"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-349A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'memory.osvm_page_source_pilot_box' "$MODULE" "module must export OSVM/page-source owner"
guard_expect_in_file "$TAG" 'osvm_page_source_pilot_box.hako' "$MEMORY_README" "memory README must name OSVM/page-source owner"
guard_expect_in_file "$TAG" 'record HakoAllocOSVMPageSourcePilotReportFields' "$OWNER" "OSVM/page-source owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeOSVMPageSourcePilotReport' "$OWNER" "OSVM/page-source owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'recordOSVMPageSourceFact' "$OWNER" "OSVM/page-source owner must expose page-source fact route"
guard_expect_in_file "$TAG" 'page_source_token: i64' "$OWNER" "OSVM/page-source report must carry page-source token"
guard_expect_in_file "$TAG" 'report_applied_backing_bytes: usize' "$OWNER" "OSVM/page-source report must mirror backing bytes as usize"
guard_expect_in_file "$TAG" 'would_dereference: 0' "$OWNER" "OSVM/page-source owner must explicitly keep dereference closed"
guard_expect_in_file "$TAG" 'would_call_osvm: page_source_present' "$OWNER" "OSVM/page-source owner must make the opened seam explicit"

if rg -n 'pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator|replace_process_allocator' "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-349A owner must keep deref/thread/provider/replacement seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'osvm-page-source-pilot-proof|OSVMPageSourcePilot|osvmPageSourcePilot' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-349A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap349_osvm_page.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap349.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-osvm-page-source-pilot-proof' "$vm_log"
rg -F -q 'page=1,0,1,1,1,99019005077,1,99019005066,99019005055,99019005044' "$vm_log"
rg -F -q 'owner=7,1,6,1,1,1,1,1,1,6' "$vm_log"
rg -F -q 'tokens=97019005005,98019005005,99019005005' "$vm_log"
rg -F -q 'bytes=4096,4096,8192' "$vm_log"
rg -F -q 'rejected=1,2,3,4,5,6' "$vm_log"
rg -F -q 'closed=0,0,0,1,1,1,0,0,0' "$vm_log"
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
    "HakoAllocOSVMPageSourcePilot.makeOSVMPageSourcePilotReport/1",
    "HakoAllocOSVMPageSourcePilot.recordOSVMPageSourceFact/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocOSVMPageSourcePilotReport")
if report is None:
    raise SystemExit("missing OSVM/page-source pilot report typed object plan")
target = "HakoAllocOSVMPageSourcePilotReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing OSVM/page-source pilot ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in ("report_applied_backing_bytes", "report_applied_committed_bytes", "report_remaining_source_bytes"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"{name} must be exact usize storage: {field}")
for name in ("page_source_token", "page_source_token_valid", "bitmap_token"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap349a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

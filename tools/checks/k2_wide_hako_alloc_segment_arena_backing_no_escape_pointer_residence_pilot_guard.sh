#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-no-escape-pointer-residence-pilot"
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
    echo "[$TAG] ERROR: MIMAP-344A defers L3/L4 evidence to a closeout or backend-facing route change" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-no-escape-pointer-residence-pilot-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-no-escape-pointer-residence-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-no-escape-pointer-residence-pilot-proof/test.sh"
CARD_343A="docs/development/current/main/phases/phase-293x/293x-958-MIMAP-343A-REMAINING-EXECUTION-PREREQUISITE-LEDGER-CLOSEOUT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-959-MIMAP-344A-NO-ESCAPE-POINTER-RESIDENCE-PILOT.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-no-escape-pointer-residence-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/segment_arena_backing_no_escape_pointer_residence_pilot_box.hako"
LEDGER_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_remaining_execution_prerequisite_ledger_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_no_escape_pointer_residence_pilot_guard.sh"

printf '[%s] checking MIMAP-344A segment arena backing no-escape pointer residence pilot\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_343A" "$CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$LEDGER_OWNER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_343A" "MIMAP-343A closeout must be landed before no-escape pointer residence pilot"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-344A card must be landed after no-escape pointer residence pilot is accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-344A design must be accepted"
guard_expect_in_file "$TAG" 'private proof-scope token' "$DESIGN" "MIMAP-344A design must keep pointer token proof-scope"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-344A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-344A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-344A"
guard_expect_in_file "$TAG" 'row_kind = "first-real-seam"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-344A must be marked as first-real-seam"
guard_expect_in_file "$TAG" 'first_pattern = true' "$PROOF_MANIFEST_INCLUDE" "MIMAP-344A must mark first pattern"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-344A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_no_escape_pointer_residence_pilot_box' "$MODULE" "module must export no-escape pointer residence pilot owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_no_escape_pointer_residence_pilot_box.hako' "$MEMORY_README" "memory README must name no-escape pointer residence pilot owner"
guard_expect_in_file "$TAG" 'record HakoAllocSegmentArenaBackingNoEscapePointerResidencePilotReportFields' "$OWNER" "pilot owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeNoEscapePointerResidencePilotReport' "$OWNER" "pilot owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'recordNoEscapePointerResidence' "$OWNER" "pilot owner must expose no-escape pointer residence route"
guard_expect_in_file "$TAG" 'private_pointer_token: i64' "$OWNER" "pilot report must carry private pointer token"
guard_expect_in_file "$TAG" 'report_applied_backing_bytes: usize' "$OWNER" "pilot must mirror backing bytes as usize"

if rg -n 'lookupByPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-344A owner must keep lookup/deref/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'no-escape-pointer-residence-pilot-proof|NoEscapePointerResidencePilot|noEscapePointerResidencePilot' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-344A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap344_no_escape_pointer_residence.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap344.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-segment-arena-backing-no-escape-pointer-residence-pilot-proof' "$vm_log"
rg -F -q 'residence=1,0,1,1,1,99019005022,1,190,1,1' "$vm_log"
rg -F -q 'owner=5,1,4,1,1,1,1,0,4' "$vm_log"
rg -F -q 'tokens=97019005005,98019005005,99019005005' "$vm_log"
rg -F -q 'bytes=4096,4096,8192' "$vm_log"
rg -F -q 'rejected=0,1,0,2,0,3,0,4' "$vm_log"
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
    "HakoAllocSegmentArenaBackingNoEscapePointerResidencePilot.makeNoEscapePointerResidencePilotReport/1",
    "HakoAllocSegmentArenaBackingNoEscapePointerResidencePilot.recordNoEscapePointerResidence/5",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingNoEscapePointerResidencePilotReport")
if report is None:
    raise SystemExit("missing no-escape pointer residence pilot report typed object plan")
target = "HakoAllocSegmentArenaBackingNoEscapePointerResidencePilotReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing no-escape pointer residence pilot ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in ("report_applied_backing_bytes", "report_applied_committed_bytes", "report_remaining_source_bytes"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"{name} must be exact usize storage: {field}")
for name in ("private_pointer_token", "token_valid", "non_dereferenceable"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap344a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

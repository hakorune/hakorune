#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-atomic-bitmap-pilot"
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
    echo "[$TAG] ERROR: MIMAP-348A defers L3/L4 evidence to a closeout or backend-facing route change" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-atomic-bitmap-pilot-proof/main.hako"
APP_README="apps/hako-alloc-atomic-bitmap-pilot-proof/README.md"
APP_TEST="apps/hako-alloc-atomic-bitmap-pilot-proof/test.sh"
CARD_347A="docs/development/current/main/phases/phase-293x/293x-962-MIMAP-347A-SEGMENT-MAP-MUTATION-PILOT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-963-MIMAP-348A-ATOMIC-BITMAP-PILOT.md"
DESIGN="docs/development/current/main/design/hako-alloc-atomic-bitmap-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST_INCLUDE="tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/atomic_bitmap_pilot_box.hako"
MUTATION_OWNER="lang/src/hako_alloc/memory/segment_map_mutation_pilot_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_atomic_bitmap_pilot_guard.sh"

printf '[%s] checking MIMAP-348A atomic bitmap pilot\n' "$TAG"

guard_require_files "$TAG" "$APP" "$APP_README" "$APP_TEST" "$CARD_347A" "$CARD" "$DESIGN" "$INDEX" "$PROOF_MANIFEST_INCLUDE" "$MODULE" "$MEMORY_README" "$OWNER" "$MUTATION_OWNER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_347A" "MIMAP-347A segment-map mutation pilot must be landed before atomic bitmap pilot"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$CARD" "MIMAP-348A card must be current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-348A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-348A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-348A"' "$PROOF_MANIFEST_INCLUDE" "proof manifest must list MIMAP-348A"
guard_expect_in_file "$TAG" 'row_kind = "first-real-seam"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-348A must be marked as first-real-seam"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_INCLUDE" "MIMAP-348A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'memory.atomic_bitmap_pilot_box' "$MODULE" "module must export atomic bitmap owner"
guard_expect_in_file "$TAG" 'atomic_bitmap_pilot_box.hako' "$MEMORY_README" "memory README must name atomic bitmap owner"
guard_expect_in_file "$TAG" 'record HakoAllocAtomicBitmapPilotReportFields' "$OWNER" "atomic bitmap owner must use ReportFields record payload"
guard_expect_in_file "$TAG" 'makeAtomicBitmapPilotReport' "$OWNER" "atomic bitmap owner must expose ReportFields helper"
guard_expect_in_file "$TAG" 'recordAtomicBitmapFact' "$OWNER" "atomic bitmap owner must expose bitmap fact route"
guard_expect_in_file "$TAG" 'bitmap_token: i64' "$OWNER" "atomic bitmap report must carry bitmap token"
guard_expect_in_file "$TAG" 'report_applied_backing_bytes: usize' "$OWNER" "atomic bitmap report must mirror backing bytes as usize"
guard_expect_in_file "$TAG" 'would_dereference: 0' "$OWNER" "atomic bitmap owner must explicitly keep dereference closed"
guard_expect_in_file "$TAG" 'would_execute_atomic_bitmap: bitmap_present' "$OWNER" "atomic bitmap owner must make the opened seam explicit"

if rg -n 'pointer_member|dereference[[:space:]]*\(|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-348A owner must keep real atomics/deref/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'atomic-bitmap-pilot-proof|AtomicBitmapPilot|atomicBitmapPilot' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-348A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap348_atomic_bitmap.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/mimap348.mir.json"
vm_log="$tmp_dir/vm.log"

pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"
rg -F -q 'hako-alloc-atomic-bitmap-pilot-proof' "$vm_log"
rg -F -q 'bitmap=1,0,1,1,1,99019005066,1,99019005055,99019005044,99019005033,99019005022' "$vm_log"
rg -F -q 'owner=7,1,6,1,1,1,1,1,1,6' "$vm_log"
rg -F -q 'tokens=97019005005,98019005005,99019005005' "$vm_log"
rg -F -q 'bytes=4096,4096,8192' "$vm_log"
rg -F -q 'rejected=1,2,3,4,5,6' "$vm_log"
rg -F -q 'closed=0,0,0,1,1,0,0,0,0' "$vm_log"
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
    "HakoAllocAtomicBitmapPilot.makeAtomicBitmapPilotReport/1",
    "HakoAllocAtomicBitmapPilot.recordAtomicBitmapFact/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")
plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocAtomicBitmapPilotReport")
if report is None:
    raise SystemExit("missing atomic bitmap pilot report typed object plan")
target = "HakoAllocAtomicBitmapPilotReportFields"
if not any((decl.get("name") if isinstance(decl, dict) else decl) == target for decl in data.get("record_decls", [])):
    raise SystemExit("missing atomic bitmap pilot ReportFields record")
fields = {field.get("name"): field for field in report.get("fields", [])}
for name in ("report_applied_backing_bytes", "report_applied_committed_bytes", "report_remaining_source_bytes"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"{name} must be exact usize storage: {field}")
for name in ("bitmap_token", "bitmap_token_valid", "mutation_token"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"{name} must be i64: {field}")
print("[mimap348a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

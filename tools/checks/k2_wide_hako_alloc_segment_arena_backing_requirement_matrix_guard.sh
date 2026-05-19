#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-requirement-matrix"
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
    echo "[$TAG] ERROR: MIMAP-240A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-requirement-matrix-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-requirement-matrix-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-requirement-matrix-proof/test.sh"
CARD_238A="docs/development/current/main/phases/phase-293x/293x-761-MIMAP-238A-SEGMENT-ARENA-BACKING-READINESS-CLOSEOUT-PACK.md"
CARD="docs/development/current/main/phases/phase-293x/293x-763-MIMAP-240A-SEGMENT-ARENA-BACKING-SCALAR-REQUIREMENT-MATRIX-INVENTORY.md"
CARD_241A="docs/development/current/main/phases/phase-293x/293x-764-MIMAP-241A-SEGMENT-ARENA-BACKING-REQUIREMENT-MATRIX-DIAGNOSTICS.md"
DESIGN_READINESS="docs/development/current/main/design/hako-alloc-segment-arena-backing-readiness-inventory-ssot.md"
DESIGN_DIAGNOSTICS="docs/development/current/main/design/hako-alloc-segment-arena-backing-readiness-diagnostics-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-requirement-matrix-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
READINESS_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_readiness_inventory_box.hako"
DIAGNOSTIC_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_readiness_diagnostic_box.hako"
OWNER="lang/src/hako_alloc/memory/segment_arena_backing_requirement_matrix_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_guard.sh"

printf '[%s] checking MIMAP-240A segment arena backing scalar requirement matrix\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_238A" \
  "$CARD" \
  "$CARD_241A" \
  "$DESIGN_READINESS" \
  "$DESIGN_DIAGNOSTICS" \
  "$DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$CADENCE" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$READINESS_OWNER" \
  "$DIAGNOSTIC_OWNER" \
  "$OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_238A" "MIMAP-238A closeout must be landed before requirement matrix"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-240A card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_241A" "MIMAP-241A diagnostics must be landed after MIMAP-240A"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_READINESS" "MIMAP-236A readiness design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_DIAGNOSTICS" "MIMAP-237A diagnostics design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-240A requirement matrix design must be accepted"
guard_expect_in_file "$TAG" 'Reason Vocabulary' "$DESIGN" "MIMAP-240A design must define reason vocabulary"
guard_expect_in_file "$TAG" 'MIMAP-240A' "$PLAN" "granularity SSOT must describe MIMAP-240A"
guard_expect_in_file "$TAG" 'MIMAP-241A' "$PLAN" "granularity SSOT must describe MIMAP-241A"
guard_expect_in_file "$TAG" 'MIMAP-240A segment arena backing scalar requirement matrix inventory' "$JOINT" "joint order must name MIMAP-240A"
guard_expect_in_file "$TAG" 'segment arena backing requirement matrix rows' "$CADENCE" "cadence SSOT must define requirement matrix family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-240A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-240A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-240A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-240A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-240A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_requirement_matrix_box' "$MODULE" "module must export requirement matrix owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_requirement_matrix_box.hako' "$MEMORY_README" "memory README must name requirement matrix owner"
guard_expect_in_file "$TAG" 'recordRequirementMatrix' "$OWNER" "owner must expose requirement matrix recorder"
guard_expect_in_file "$TAG" 'scalar_requirement_matrix_present: i64 = 1' "$OWNER" "report must publish scalar matrix presence bit"
guard_expect_in_file "$TAG" 'slice_count: usize = 0' "$OWNER" "requirement matrix slice count must be exact usize after HAKO-ALLOC-USIZE-FIELD-GROUP-030"
guard_expect_in_file "$TAG" 'committed_slices: usize = 0' "$OWNER" "requirement matrix committed slices must be exact usize after HAKO-ALLOC-USIZE-FIELD-GROUP-030"
guard_expect_in_file "$TAG" 'free_slices: usize = 0' "$OWNER" "requirement matrix free slices must be exact usize after HAKO-ALLOC-USIZE-FIELD-GROUP-030"
guard_expect_in_file "$TAG" 'page_size: usize = 0' "$OWNER" "requirement matrix page size must be exact usize after HAKO-ALLOC-USIZE-FIELD-GROUP-030"
guard_expect_in_file "$TAG" 'required_alignment: i64 = 0' "$OWNER" "requirement matrix alignment must remain i64"
guard_expect_in_file "$TAG" 'segment_id: i64 = -1' "$OWNER" "requirement matrix id sentinel must remain i64"
guard_expect_in_file "$TAG" 'check "mimap240a segment arena backing scalar requirement matrix"' "$APP" "proof must use labelled check block"

if rg -n 'allocateArena|ArenaBackingAlloc|arenaBackingAllocate|rawPointer|pointer_member|lookupSegment[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-240A must keep arena/raw/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-requirement-matrix-proof|SegmentArenaBackingRequirementMatrix|arenaBackingRequirementMatrix' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-240A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap240_arena_matrix.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap240.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-requirement-matrix-proof' "$vm_log"
rg -F -q 'base=70007004,70007004002,1' "$vm_log"
rg -F -q 'matrix=1,0,70,7,16,8,8,8,4096,1,0' "$vm_log"
rg -F -q 'rejects=1,2,3,4,5,6,7,8,9,10,11' "$vm_log"
rg -F -q 'flags=1,1,1,1,1,1,1,1' "$vm_log"
rg -F -q 'counts=12,1,11,1,1,1,1,1,1,1,1,1,1,1' "$vm_log"
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
    "HakoAllocSegmentArenaBackingRequirementMatrixInventory.recordRequirementMatrix/10",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingRequirementMatrixReport")
if report is None:
    raise SystemExit("missing arena backing requirement matrix report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "accepted",
    "reason",
    "matrix_present",
    "scalar_requirement_matrix_present",
    "readiness_present",
    "diagnostic_present",
    "readiness_reason",
    "diagnostic_reason",
    "segment_id",
    "arena_id",
    "slice_count",
    "committed_slices",
    "free_slices",
    "required_alignment",
    "page_size",
    "geometry_valid",
    "requires_arena_backing",
    "requires_raw_pointer",
    "requires_segment_map",
    "requires_atomic_bitmap",
    "requires_osvm",
    "requires_worker",
    "requires_provider",
    "requires_backend_matcher",
    "blocked_requirement_count",
    "would_allocate_arena_backing",
    "would_use_raw_pointer",
    "would_use_segment_map",
    "would_execute_atomic_bitmap",
    "would_call_osvm",
    "would_run_thread",
    "would_activate_provider",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing arena backing requirement matrix report field: {name}")

for name in (
    "slice_count",
    "committed_slices",
    "free_slices",
    "page_size",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"requirement matrix {name} must be exact usize storage: {field}")

for name in (
    "accepted",
    "reason",
    "segment_id",
    "arena_id",
    "required_alignment",
    "geometry_valid",
    "requires_arena_backing",
    "blocked_requirement_count",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"requirement matrix {name} must remain i64 storage: {field}")

print("[mimap240a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-requirement-matrix-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-241A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-requirement-matrix-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-requirement-matrix-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-requirement-matrix-diagnostics-proof/test.sh"
CARD_240A="docs/development/current/main/phases/phase-293x/293x-763-MIMAP-240A-SEGMENT-ARENA-BACKING-SCALAR-REQUIREMENT-MATRIX-INVENTORY.md"
CARD="docs/development/current/main/phases/phase-293x/293x-764-MIMAP-241A-SEGMENT-ARENA-BACKING-REQUIREMENT-MATRIX-DIAGNOSTICS.md"
CARD_242A="docs/development/current/main/phases/phase-293x/293x-765-MIMAP-242A-SEGMENT-ARENA-BACKING-REQUIREMENT-MATRIX-CLOSEOUT-PACK.md"
DESIGN_240A="docs/development/current/main/design/hako-alloc-segment-arena-backing-requirement-matrix-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-requirement-matrix-diagnostics-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
INVENTORY_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_requirement_matrix_box.hako"
DIAGNOSTIC_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_requirement_matrix_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_diagnostics_guard.sh"

printf '[%s] checking MIMAP-241A segment arena backing requirement matrix diagnostics\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_240A" \
  "$CARD" \
  "$CARD_242A" \
  "$DESIGN_240A" \
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

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_240A" "MIMAP-240A inventory must be landed before diagnostics"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-241A card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_242A" "MIMAP-242A closeout must be landed after MIMAP-241A"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_240A" "MIMAP-240A matrix design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-241A diagnostics design must be accepted"
guard_expect_in_file "$TAG" 'observer-only diagnostics' "$CARD" "MIMAP-241A card must call out observer-only diagnostics"
guard_expect_in_file "$TAG" 'MIMAP-241A' "$PLAN" "granularity SSOT must describe MIMAP-241A"
guard_expect_in_file "$TAG" 'MIMAP-242A' "$PLAN" "granularity SSOT must describe MIMAP-242A"
guard_expect_in_file "$TAG" 'MIMAP-241A segment arena backing requirement matrix diagnostics' "$JOINT" "joint order must name MIMAP-241A"
guard_expect_in_file "$TAG" 'segment arena backing requirement matrix rows' "$CADENCE" "cadence SSOT must define requirement matrix family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-241A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-241A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-241A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-241A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-241A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_requirement_matrix_diagnostic_box' "$MODULE" "module must export diagnostic owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_requirement_matrix_diagnostic_box.hako' "$MEMORY_README" "memory README must name diagnostic owner"
guard_expect_in_file "$TAG" 'observeRequirementMatrixDiagnostics' "$DIAGNOSTIC_OWNER" "diagnostic owner must expose observer route"
guard_expect_in_file "$TAG" 'diagnostic_present: i64 = 1' "$DIAGNOSTIC_OWNER" "diagnostic report must publish presence bit"
guard_expect_in_file "$TAG" 'check "mimap241a segment arena backing requirement matrix diagnostics"' "$APP" "proof must use labelled check block"

if rg -n 'recordRequirementMatrix|me\.(inventory_count|accepted_count|reject_count|readiness_reject_count|diagnostic_reject_count|geometry_reject_count|arena_requirement_reject_count|raw_pointer_requirement_reject_count|segment_map_requirement_reject_count|atomic_bitmap_requirement_reject_count|osvm_requirement_reject_count|worker_requirement_reject_count|provider_requirement_reject_count|backend_matcher_requirement_reject_count)[[:space:]]*\+=' \
  "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".mutation_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-241A diagnostic owner must not record matrix rows or mutate inventory counters" >&2
  cat /tmp/"$TAG".mutation_leak >&2
  rm -f /tmp/"$TAG".mutation_leak
  exit 1
fi
rm -f /tmp/"$TAG".mutation_leak

if rg -n 'allocateArena|ArenaBackingAlloc|arenaBackingAllocate|rawPointer|pointer_member|lookupSegment[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-241A must keep arena/raw/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-requirement-matrix-diagnostics-proof|SegmentArenaBackingRequirementMatrixDiagnostic|arenaBackingRequirementMatrixDiagnostic' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-241A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap241_arena_matrix_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap241.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-requirement-matrix-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,11,12,1,11,8,11,70,7,1' "$vm_log"
rg -F -q 'seen=1,1,1,1,1,1,1,1,1,1,1' "$vm_log"
rg -F -q 'owner=2,1,1,1,1' "$vm_log"
rg -F -q 'empty=0,1,0' "$vm_log"
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
    "Main.makeReadiness/5",
    "HakoAllocSegmentArenaBackingRequirementMatrixInventory.recordRequirementMatrix/10",
    "HakoAllocSegmentArenaBackingRequirementMatrixDiagnostic.observeRequirementMatrixDiagnostics/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingRequirementMatrixDiagnosticReport")
if report is None:
    raise SystemExit("missing arena backing requirement matrix diagnostic report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "observed",
    "reason",
    "diagnostic_present",
    "inventory_count",
    "accepted_count",
    "reject_count",
    "blocked_requirement_reject_count",
    "readiness_reject_seen",
    "matrix_diagnostic_reject_seen",
    "geometry_reject_seen",
    "arena_requirement_reject_seen",
    "raw_pointer_requirement_reject_seen",
    "segment_map_requirement_reject_seen",
    "atomic_bitmap_requirement_reject_seen",
    "osvm_requirement_reject_seen",
    "worker_requirement_reject_seen",
    "provider_requirement_reject_seen",
    "backend_matcher_requirement_reject_seen",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing arena backing requirement matrix diagnostic field: {name}")

print("[mimap241a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

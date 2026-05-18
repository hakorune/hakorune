#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-no-escape-address-capability"
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
    echo "[$TAG] ERROR: MIMAP-244A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-no-escape-address-capability-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-no-escape-address-capability-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-no-escape-address-capability-proof/test.sh"
CARD_242A="docs/development/current/main/phases/phase-293x/293x-765-MIMAP-242A-SEGMENT-ARENA-BACKING-REQUIREMENT-MATRIX-CLOSEOUT-PACK.md"
CARD_243A="docs/development/current/main/phases/phase-293x/293x-766-MIMAP-243A-POST-SEGMENT-ARENA-BACKING-REQUIREMENT-MATRIX-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-767-MIMAP-244A-SEGMENT-ARENA-BACKING-NO-ESCAPE-RAW-POINTER-CAPABILITY-INVENTORY.md"
CARD_245A="docs/development/current/main/phases/phase-293x/293x-768-MIMAP-245A-SEGMENT-ARENA-BACKING-NO-ESCAPE-ADDRESS-CAPABILITY-DIAGNOSTICS.md"
DESIGN_MATRIX="docs/development/current/main/design/hako-alloc-segment-arena-backing-requirement-matrix-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-no-escape-address-capability-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MATRIX_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_requirement_matrix_box.hako"
OWNER="lang/src/hako_alloc/memory/segment_arena_backing_no_escape_address_capability_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_no_escape_address_capability_guard.sh"

printf '[%s] checking MIMAP-244A segment arena backing no-escape address capability\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_242A" \
  "$CARD_243A" \
  "$CARD" \
  "$CARD_245A" \
  "$DESIGN_MATRIX" \
  "$DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$MATRIX_OWNER" \
  "$OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_242A" "MIMAP-242A closeout must be landed before address capability"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_243A" "MIMAP-243A selection must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-244A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$CARD_245A" "MIMAP-245A diagnostics must be selected after MIMAP-244A"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_MATRIX" "MIMAP-240A matrix design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-244A design must be accepted"
guard_expect_in_file "$TAG" 'Reason Vocabulary' "$DESIGN" "MIMAP-244A design must define reason vocabulary"
guard_expect_in_file "$TAG" 'MIMAP-244A' "$PLAN" "granularity SSOT must describe MIMAP-244A"
guard_expect_in_file "$TAG" 'MIMAP-245A' "$PLAN" "granularity SSOT must describe MIMAP-245A"
guard_expect_in_file "$TAG" 'MIMAP-244A segment arena backing no-escape raw pointer capability inventory' "$JOINT" "joint order must name MIMAP-244A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-244A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-244A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-244A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-244A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-244A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_no_escape_address_capability_box' "$MODULE" "module must export owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_no_escape_address_capability_box.hako' "$MEMORY_README" "memory README must name owner"
guard_expect_in_file "$TAG" 'recordCapability' "$OWNER" "owner must expose capability recorder"
guard_expect_in_file "$TAG" 'no_escape_address_capability_present: i64 = 1' "$OWNER" "report must publish capability presence bit"
guard_expect_in_file "$TAG" 'check "mimap244a segment arena backing no escape address capability"' "$APP" "proof must use labelled check block"

if rg -n 'pointer_member|lookupByPointer|lookupPointer|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-244A must keep pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-no-escape-address-capability-proof|NoEscapeAddressCapability|noEscapeAddressCapability' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-244A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap244_no_escape_addr.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap244.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-no-escape-address-capability-proof' "$vm_log"
rg -F -q 'capability=1,0,70,7,3,70007004002,1,0' "$vm_log"
rg -F -q 'rejects=1,2,3,4,5,6,7,8,9,10,11,12,13,14,15' "$vm_log"
rg -F -q 'counts=16,1,15,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1' "$vm_log"
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
    "Main.makeMatrix/2",
    "HakoAllocSegmentArenaBackingNoEscapeAddressCapabilityInventory.recordCapability/15",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingNoEscapeAddressCapabilityReport")
if report is None:
    raise SystemExit("missing no-escape address capability report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "accepted",
    "reason",
    "capability_present",
    "no_escape_address_capability_present",
    "matrix_present",
    "lifetime_generation",
    "address_carrier",
    "address_carrier_valid",
    "escape_blocker_count",
    "escape_to_return",
    "escape_to_storage",
    "escape_to_alias",
    "requires_pointer_residence",
    "requires_pointer_lookup",
    "would_create_pointer_residence",
    "would_lookup_by_pointer",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing no-escape address capability report field: {name}")

print("[mimap244a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

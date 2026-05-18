#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-no-escape-address-residence"
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
    echo "[$TAG] ERROR: MIMAP-248A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-proof/test.sh"
CARD_246A="docs/development/current/main/phases/phase-293x/293x-769-MIMAP-246A-SEGMENT-ARENA-BACKING-NO-ESCAPE-ADDRESS-CAPABILITY-CLOSEOUT-PACK.md"
CARD_247A="docs/development/current/main/phases/phase-293x/293x-770-MIMAP-247A-POST-SEGMENT-ARENA-BACKING-NO-ESCAPE-ADDRESS-CAPABILITY-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-771-MIMAP-248A-SEGMENT-ARENA-BACKING-MODELED-NO-ESCAPE-ADDRESS-RESIDENCE-INVENTORY.md"
CARD_249A="docs/development/current/main/phases/phase-293x/293x-772-MIMAP-249A-SEGMENT-ARENA-BACKING-MODELED-NO-ESCAPE-ADDRESS-RESIDENCE-DIAGNOSTICS.md"
CAPABILITY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-no-escape-address-capability-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CAPABILITY_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_no_escape_address_capability_box.hako"
OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_no_escape_address_residence_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_no_escape_address_residence_guard.sh"

printf '[%s] checking MIMAP-248A segment arena backing modeled no-escape address residence\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_246A" \
  "$CARD_247A" \
  "$CARD" \
  "$CARD_249A" \
  "$CAPABILITY_SSOT" \
  "$DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$CADENCE" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CAPABILITY_OWNER" \
  "$OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_246A" "MIMAP-246A closeout must be landed before residence inventory"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_247A" "MIMAP-247A selection must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-248A card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_249A" "MIMAP-249A diagnostics must be landed after MIMAP-248A"
guard_expect_in_file "$TAG" 'Decision: accepted' "$CAPABILITY_SSOT" "MIMAP-244A capability design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-248A residence design must be accepted"
guard_expect_in_file "$TAG" 'modeled residence row' "$CARD" "MIMAP-248A card must call out modeled residence"
guard_expect_in_file "$TAG" 'MIMAP-248A' "$PLAN" "granularity SSOT must describe MIMAP-248A"
guard_expect_in_file "$TAG" 'MIMAP-249A' "$PLAN" "granularity SSOT must describe MIMAP-249A"
guard_expect_in_file "$TAG" 'MIMAP-248A segment arena backing modeled no-escape address residence inventory' "$JOINT" "joint order must name MIMAP-248A"
guard_expect_in_file "$TAG" 'modeled no-escape address residence rows' "$CADENCE" "cadence SSOT must define residence family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-248A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-248A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-248A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-248A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-248A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_no_escape_address_residence_box' "$MODULE" "module must export residence owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_no_escape_address_residence_box.hako' "$MEMORY_README" "memory README must name residence owner"
guard_expect_in_file "$TAG" 'recordResidence' "$OWNER" "residence owner must expose record route"
guard_expect_in_file "$TAG" 'non_dereferenceable: i64 = 1' "$OWNER" "residence report must publish non-dereferenceable bit"
guard_expect_in_file "$TAG" 'check "mimap248a segment arena backing modeled no escape address residence"' "$APP" "proof must use labelled check block"

if rg -n 'lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-248A must keep pointer lookup/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-proof|ModeledNoEscapeAddressResidence|modeledNoEscapeAddressResidence' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-248A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap248_modeled_residence.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap248.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-proof' "$vm_log"
rg -F -q 'residence=1,0,1,70,7,3,70007004002,70007004002,1' "$vm_log"
rg -F -q 'rejects=1,2,4,5,6,7,8,9,10,11,12,13,14' "$vm_log"
rg -F -q 'counts=14,1,13,1,1,1,1,9,14,0' "$vm_log"
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
    "HakoAllocSegmentArenaBackingModeledNoEscapeAddressResidenceInventory.recordResidence/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingModeledNoEscapeAddressResidenceReport")
if report is None:
    raise SystemExit("missing modeled no-escape address residence report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "accepted",
    "reason",
    "residence_present",
    "modeled_no_escape_address_residence_present",
    "row_index",
    "segment_id",
    "arena_id",
    "address_carrier",
    "residence_token",
    "non_dereferenceable",
    "closed_substrate_blocker_count",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing modeled residence field: {name}")

print("[mimap248a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

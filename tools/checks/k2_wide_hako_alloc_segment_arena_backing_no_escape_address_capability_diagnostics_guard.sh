#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-no-escape-address-capability-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-245A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-no-escape-address-capability-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-no-escape-address-capability-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-no-escape-address-capability-diagnostics-proof/test.sh"
CARD_244A="docs/development/current/main/phases/phase-293x/293x-767-MIMAP-244A-SEGMENT-ARENA-BACKING-NO-ESCAPE-RAW-POINTER-CAPABILITY-INVENTORY.md"
CARD="docs/development/current/main/phases/phase-293x/293x-768-MIMAP-245A-SEGMENT-ARENA-BACKING-NO-ESCAPE-ADDRESS-CAPABILITY-DIAGNOSTICS.md"
CARD_246A="docs/development/current/main/phases/phase-293x/293x-769-MIMAP-246A-SEGMENT-ARENA-BACKING-NO-ESCAPE-ADDRESS-CAPABILITY-CLOSEOUT-PACK.md"
DESIGN_244A="docs/development/current/main/design/hako-alloc-segment-arena-backing-no-escape-address-capability-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-no-escape-address-capability-diagnostics-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
INVENTORY_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_no_escape_address_capability_box.hako"
DIAGNOSTIC_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_no_escape_address_capability_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_no_escape_address_capability_diagnostics_guard.sh"

printf '[%s] checking MIMAP-245A segment arena backing no-escape address capability diagnostics\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_244A" \
  "$CARD" \
  "$CARD_246A" \
  "$DESIGN_244A" \
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

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_244A" "MIMAP-244A inventory must be landed before diagnostics"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-245A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$CARD_246A" "MIMAP-246A closeout must be selected after MIMAP-245A"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_244A" "MIMAP-244A design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-245A diagnostics design must be accepted"
guard_expect_in_file "$TAG" 'observer-only diagnostics' "$CARD" "MIMAP-245A card must call out observer-only diagnostics"
guard_expect_in_file "$TAG" 'MIMAP-245A' "$PLAN" "granularity SSOT must describe MIMAP-245A"
guard_expect_in_file "$TAG" 'MIMAP-246A' "$PLAN" "granularity SSOT must describe MIMAP-246A"
guard_expect_in_file "$TAG" 'MIMAP-245A segment arena backing no-escape address capability diagnostics' "$JOINT" "joint order must name MIMAP-245A"
guard_expect_in_file "$TAG" 'segment arena backing no-escape address capability rows' "$CADENCE" "cadence SSOT must define capability family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-245A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-245A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-245A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-245A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-245A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_no_escape_address_capability_diagnostic_box' "$MODULE" "module must export diagnostic owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_no_escape_address_capability_diagnostic_box.hako' "$MEMORY_README" "memory README must name diagnostic owner"
guard_expect_in_file "$TAG" 'observeCapabilityDiagnostics' "$DIAGNOSTIC_OWNER" "diagnostic owner must expose observer route"
guard_expect_in_file "$TAG" 'diagnostic_present: i64 = 1' "$DIAGNOSTIC_OWNER" "diagnostic report must publish presence bit"
guard_expect_in_file "$TAG" 'check "mimap245a segment arena backing no escape address capability diagnostics"' "$APP" "proof must use labelled check block"

if rg -n 'recordCapability|me\.(inventory_count|accepted_count|reject_count|matrix_reject_count|lifetime_reject_count|address_carrier_reject_count|return_escape_reject_count|storage_escape_reject_count|alias_escape_reject_count|pointer_residence_reject_count|pointer_lookup_reject_count|arena_backing_reject_count|segment_map_reject_count|atomic_bitmap_reject_count|osvm_reject_count|worker_reject_count|provider_reject_count|backend_matcher_reject_count)[[:space:]]*\+=' \
  "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".mutation_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-245A diagnostic owner must not record capability rows or mutate inventory counters" >&2
  cat /tmp/"$TAG".mutation_leak >&2
  rm -f /tmp/"$TAG".mutation_leak
  exit 1
fi
rm -f /tmp/"$TAG".mutation_leak

if rg -n 'pointer_member|lookupByPointer|lookupPointer|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-245A must keep pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-no-escape-address-capability-diagnostics-proof|NoEscapeAddressCapabilityDiagnostic|noEscapeAddressCapabilityDiagnostic' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-245A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap245_no_escape_addr_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap245.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-no-escape-address-capability-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,15,16,1,15,3,9,15,70,7,1' "$vm_log"
rg -F -q 'seen=1,1,1,1,1,1,1,1,1,1,1,1,1,1,1' "$vm_log"
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
    "Main.makeMatrix/2",
    "HakoAllocSegmentArenaBackingNoEscapeAddressCapabilityInventory.recordCapability/15",
    "HakoAllocSegmentArenaBackingNoEscapeAddressCapabilityDiagnostic.observeCapabilityDiagnostics/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingNoEscapeAddressCapabilityDiagnosticReport")
if report is None:
    raise SystemExit("missing no-escape address capability diagnostic report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "observed",
    "reason",
    "diagnostic_present",
    "inventory_count",
    "accepted_count",
    "reject_count",
    "escape_reject_count",
    "closed_substrate_reject_count",
    "matrix_reject_seen",
    "lifetime_reject_seen",
    "address_carrier_reject_seen",
    "return_escape_reject_seen",
    "pointer_residence_reject_seen",
    "pointer_lookup_reject_seen",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing no-escape address capability diagnostic field: {name}")

print("[mimap245a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

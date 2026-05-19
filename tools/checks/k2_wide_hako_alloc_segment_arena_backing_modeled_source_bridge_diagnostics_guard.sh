#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-source-bridge-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-261A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-source-bridge-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-source-bridge-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-source-bridge-diagnostics-proof/test.sh"
CARD_260A="docs/development/current/main/phases/phase-293x/293x-783-MIMAP-260A-SEGMENT-ARENA-BACKING-MODELED-SOURCE-BRIDGE-INVENTORY.md"
CARD="docs/development/current/main/phases/phase-293x/293x-784-MIMAP-261A-SEGMENT-ARENA-BACKING-MODELED-SOURCE-BRIDGE-DIAGNOSTICS.md"
DESIGN_260A="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-source-bridge-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-source-bridge-diagnostics-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
INVENTORY_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_source_bridge_box.hako"
DIAGNOSTIC_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_source_bridge_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_diagnostics_guard.sh"

printf '[%s] checking MIMAP-261A segment arena backing modeled source bridge diagnostics\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_260A" \
  "$CARD" \
  "$DESIGN_260A" \
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

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_260A" "MIMAP-260A inventory must be landed before diagnostics"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-261A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_260A" "MIMAP-260A source bridge design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-261A diagnostics design must be accepted"
guard_expect_in_file "$TAG" 'observer-only diagnostics' "$CARD" "MIMAP-261A card must call out observer-only diagnostics"
guard_expect_in_file "$TAG" 'MIMAP-261A granularity' "$PLAN" "granularity SSOT must describe MIMAP-261A"
guard_expect_in_file "$TAG" 'MIMAP-261A segment arena backing modeled source bridge diagnostics' "$JOINT" "joint order must name MIMAP-261A"
guard_expect_in_file "$TAG" 'segment-arena-backing-modeled-source-bridge' "$CADENCE" "cadence SSOT must define source-bridge family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-261A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-261A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-261A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-261A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-261A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_source_bridge_diagnostic_box' "$MODULE" "module must export diagnostic owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_source_bridge_diagnostic_box.hako' "$MEMORY_README" "memory README must name diagnostic owner"
guard_expect_in_file "$TAG" 'observeSourceBridgeDiagnostics' "$DIAGNOSTIC_OWNER" "diagnostic owner must expose observer route"
guard_expect_in_file "$TAG" 'diagnostic_present: i64 = 1' "$DIAGNOSTIC_OWNER" "diagnostic report must publish presence bit"
guard_expect_in_file "$TAG" 'last_report_source_capacity: usize = 0' "$DIAGNOSTIC_OWNER" "source bridge diagnostic capacity mirror must be usize after HAKO-ALLOC-USIZE-FIELD-GROUP-024"
guard_expect_in_file "$TAG" 'last_report_source_committed_bytes: usize = 0' "$DIAGNOSTIC_OWNER" "source bridge diagnostic committed mirror must be usize after HAKO-ALLOC-USIZE-FIELD-GROUP-024"
guard_expect_in_file "$TAG" 'check "mimap261a segment arena backing modeled source bridge diagnostics"' "$APP" "proof must use labelled check block"

if rg -n 'recordSourceBridge|me\.(inventory_count|accepted_count|reject_count|missing_slot_reject_count|rejected_slot_reject_count|invalid_arena_slot_token_reject_count|invalid_source_shape_reject_count|invalid_geometry_reject_count|closed_substrate_reject_count)[[:space:]]*\+=' \
  "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".mutation_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-261A diagnostic owner must not record source bridge rows or mutate inventory counters" >&2
  cat /tmp/"$TAG".mutation_leak >&2
  rm -f /tmp/"$TAG".mutation_leak
  exit 1
fi
rm -f /tmp/"$TAG".mutation_leak

if rg -n 'lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-261A must keep pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-source-bridge-diagnostics-proof|ModeledSourceBridgeDiagnostic|modeledSourceBridgeDiagnostic' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-261A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap261_source_bridge_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap261.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-source-bridge-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,6,7,1,6,1,6,70,7,1' "$vm_log"
rg -F -q 'seen=1,1,1,1,1,1' "$vm_log"
rg -F -q 'source=70007004005,1,0,16384,4096,16' "$vm_log"
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
    "Main.makeSlot/2",
    "HakoAllocSegmentArenaBackingModeledSourceBridgeInventory.recordSourceBridge/6",
    "HakoAllocSegmentArenaBackingModeledSourceBridgeDiagnostic.observeSourceBridgeDiagnostics/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingModeledSourceBridgeDiagnosticReport")
if report is None:
    raise SystemExit("missing modeled source bridge diagnostic report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "observed",
    "reason",
    "diagnostic_present",
    "inventory_count",
    "accepted_count",
    "reject_count",
    "closed_substrate_reject_seen",
    "source_bridge_present",
    "modeled_source_bridge_present",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing modeled source bridge diagnostic field: {name}")

for name in (
    "last_report_source_capacity",
    "last_report_source_committed_bytes",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"source bridge diagnostic mirror {name} must be usize storage: {field}")

print("[mimap261a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

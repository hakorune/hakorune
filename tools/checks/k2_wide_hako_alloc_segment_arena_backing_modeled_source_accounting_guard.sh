#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-source-accounting"
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
    echo "[$TAG] ERROR: MIMAP-264A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-source-accounting-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-source-accounting-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-source-accounting-proof/test.sh"
CARD_260A="docs/development/current/main/phases/phase-293x/293x-783-MIMAP-260A-SEGMENT-ARENA-BACKING-MODELED-SOURCE-BRIDGE-INVENTORY.md"
CARD_262A="docs/development/current/main/phases/phase-293x/293x-785-MIMAP-262A-SEGMENT-ARENA-BACKING-MODELED-SOURCE-BRIDGE-CLOSEOUT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-787-MIMAP-264A-SEGMENT-ARENA-BACKING-MODELED-SOURCE-ACCOUNTING-INVENTORY.md"
DESIGN_260A="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-source-bridge-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-source-accounting-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
SOURCE_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_source_bridge_box.hako"
ACCOUNTING_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_source_accounting_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_accounting_guard.sh"

printf '[%s] checking MIMAP-264A segment arena backing modeled source accounting\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_260A" \
  "$CARD_262A" \
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
  "$SOURCE_OWNER" \
  "$ACCOUNTING_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_260A" "MIMAP-260A source bridge must be landed before accounting"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_262A" "MIMAP-262A source bridge closeout must be landed before accounting"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-264A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_260A" "MIMAP-260A source bridge design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-264A accounting design must be accepted"
guard_expect_in_file "$TAG" 'source-backed arena accounting' "$CARD" "MIMAP-264A card must call out source-backed arena accounting"
guard_expect_in_file "$TAG" 'MIMAP-264A segment arena backing modeled source accounting inventory' "$PLAN" "granularity SSOT must describe MIMAP-264A"
guard_expect_in_file "$TAG" 'MIMAP-264A segment arena backing modeled source accounting inventory' "$JOINT" "joint order must name MIMAP-264A"
guard_expect_in_file "$TAG" 'MIMAP-264A segment arena backing modeled source accounting inventory' "$CADENCE" "cadence SSOT must name MIMAP-264A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-264A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-264A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-264A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-264A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-264A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_source_accounting_box' "$MODULE" "module must export source accounting owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_source_accounting_box.hako' "$MEMORY_README" "memory README must name source accounting owner"
guard_expect_in_file "$TAG" 'recordSourceAccounting' "$ACCOUNTING_OWNER" "source accounting owner must expose record route"
guard_expect_in_file "$TAG" 'source_uncommitted_bytes' "$ACCOUNTING_OWNER" "source accounting report must publish uncommitted bytes"
guard_expect_in_file "$TAG" 'check "mimap264a segment arena backing modeled source accounting"' "$APP" "proof must use labelled check block"

if rg -n 'lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$ACCOUNTING_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-264A must keep pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-source-accounting-proof|ModeledSourceAccounting|modeledSourceAccounting' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-264A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap264_source_accounting.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap264.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-source-accounting-proof' "$vm_log"
rg -F -q 'account=1,0,1,70,7,3,70007004005,1,90007004005,16384,4096,12288,4096,8192,4096' "$vm_log"
rg -F -q 'rejects=1,2,3,4,5' "$vm_log"
rg -F -q 'counts=6,1,5,1,1,1,1,1,5,0' "$vm_log"
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
    "HakoAllocSegmentArenaBackingModeledSourceBridgeInventory.recordSourceBridge/6",
    "HakoAllocSegmentArenaBackingModeledSourceAccountingInventory.recordSourceAccounting/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingModeledSourceAccountingReport")
if report is None:
    raise SystemExit("missing modeled source accounting report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "accepted",
    "reason",
    "source_accounting_present",
    "modeled_source_accounting_present",
    "source_capacity",
    "source_committed_bytes",
    "source_uncommitted_bytes",
    "accounted_padded_bytes",
    "available_after_padded_bytes",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing modeled source accounting field: {name}")

print("[mimap264a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

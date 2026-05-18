#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-modeled-consume-ledger-released-token-recycle"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

VALIDATION_LEVEL="L2"
while [ "$#" -gt 0 ]; do
  case "$1" in
    --level)
      if [ "$#" -lt 2 ]; then
        echo "[$TAG] ERROR: --level requires a value" >&2
        exit 2
      fi
      VALIDATION_LEVEL="$2"
      shift 2
      ;;
    --level=*)
      VALIDATION_LEVEL="${1#--level=}"
      shift
      ;;
    *)
      echo "[$TAG] ERROR: unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

case "$VALIDATION_LEVEL" in
  L0|L1|L2) ;;
  L3|L4)
    echo "[$TAG] ERROR: MIMAP-164A defers L3/L4 EXE evidence to a future closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-modeled-consume-ledger-released-token-recycle-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-modeled-consume-ledger-released-token-recycle-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-modeled-consume-ledger-released-token-recycle-proof/test.sh"
CARD_163A="docs/development/current/main/phases/phase-293x/293x-685-MIMAP-163A-POST-SEGMENT-MAP-MODELED-CONSUME-LEDGER-RELEASE-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-686-MIMAP-164A-SEGMENT-MAP-MODELED-CONSUME-LEDGER-RELEASED-TOKEN-RECYCLE-ROUTE.md"
CARD_165A="docs/development/current/main/phases/phase-293x/293x-687-MIMAP-165A-POST-SEGMENT-MAP-MODELED-CONSUME-LEDGER-RELEASED-TOKEN-RECYCLE-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-released-token-recycle-ssot.md"
RELEASE_SSOT="docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-release-ssot.md"
RECYCLE_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-released-token-recycle-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
PURPOSE="docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_token_recycle_guard.sh"

printf '[%s] checking MIMAP-164A segment-map modeled consume-ledger released-token recycle\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_163A" \
  "$CARD" \
  "$CARD_165A" \
  "$DESIGN" \
  "$RELEASE_SSOT" \
  "$RECYCLE_SSOT" \
  "$PLAN" \
  "$JOINT" \
  "$PURPOSE" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$MODULE" \
  "$OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_163A" "MIMAP-163A must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-164A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$CARD_165A" "MIMAP-165A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-164A design must be accepted"
guard_expect_in_file "$TAG" 'C mimalloc' "$PURPOSE" "purpose SSOT must name C mimalloc comparison target"
guard_expect_in_file "$TAG" 'performance and memory-usage comparison' "$PURPOSE" "purpose SSOT must make comparison objective explicit"
guard_expect_in_file "$TAG" 'MIMAP-161A' "$RELEASE_SSOT" "recycle route must stay downstream of release route"
guard_expect_in_file "$TAG" 'MIMAP-100A' "$RECYCLE_SSOT" "recycle route must reuse modeled ledger released-token recycle contract"
guard_expect_in_file "$TAG" 'MIMAP-164A' "$PLAN" "granularity SSOT must describe MIMAP-164A"
guard_expect_in_file "$TAG" 'MIMAP-164A segment-map modeled consume ledger released-token recycle route' "$JOINT" "joint order must name MIMAP-164A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-164A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-164A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-164A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-164A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-164A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'memory.segment_map_accepted_readiness_modeled_consume_ledger_box = "memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"' "$MODULE" "hako module must export owner"
guard_expect_in_file "$TAG" 'MIMAP-164A' "$MEMORY_README" "memory README must define MIMAP-164A owner boundary"
guard_expect_in_file "$TAG" 'consumeAcceptedReadiness' "$OWNER" "MIMAP-164A owner must expose consume route"
guard_expect_in_file "$TAG" 'releaseConsumedToken' "$OWNER" "MIMAP-164A owner must expose release route"
guard_expect_in_file "$TAG" 'check "mimap164a segment map modeled consume ledger released token recycle route"' "$APP" "MIMAP-164A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-164A must keep real allocation/free, raw pointer, concurrency, segment-map, atomics, page-source/OS release seams, and backend-visible page release inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-164A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-modeled-consume-ledger-released-token-recycle-proof|HakoAllocSegmentMapModeledConsumeLedgerReleasedTokenRecycle|releasedTokenRecycle' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-164A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_token_recycle_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-164A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

pure_first_guard_build_hakorune_debug

tmp_dir="$(mktemp -d /tmp/hakorune_mimap164a_segment_map_consume_ledger_recycle.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap164a.mir.json"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-modeled-consume-ledger-released-token-recycle-proof' "$vm_log"
rg -F -q 'first=1,0,0,-1,70007002,1,1' "$vm_log"
rg -F -q 'duplicate_live=0,3,4,0,2' "$vm_log"
rg -F -q 'release_first=1,0,0,1,0,0' "$vm_log"
rg -F -q 'after_release=-1,0,-1' "$vm_log"
rg -F -q 'recycled=1,0,1,-1,70007002,2,1' "$vm_log"
rg -F -q 'after_recycle=1,0,1,70007002' "$vm_log"
rg -F -q 'duplicate_after_recycle=0,3,4,1' "$vm_log"
rg -F -q 'release_recycled=1,0,1,1,0,0' "$vm_log"
rg -F -q 'counts=4,2,2,2,2,2,0,2,2,0,70007002,0' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$vm_log"
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
    "HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedger.consumeAcceptedReadiness/2",
    "HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedger.releaseConsumedToken/2",
    "HakoAllocSegmentAllocationModeledLedger.recordModeledConsume/12",
    "HakoAllocSegmentAllocationModeledLedger.releaseModeledToken/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedgerReport")
if report is None:
    raise SystemExit("missing segment-map consume-ledger report typed object plan")
release_report = plans.get("HakoAllocSegmentMapModeledConsumeLedgerReleaseReport")
if release_report is None:
    raise SystemExit("missing segment-map release report typed object plan")

for name in ("accepted", "reason", "ledger_reason", "row_index", "existing_index", "modeled_allocation_token"):
    fields = {field.get("name"): field for field in report.get("fields", [])}
    if fields.get(name) is None:
        raise SystemExit(f"missing consume-ledger report field: {name}")

for name in ("did_release", "reason", "row_index", "modeled_allocation_token", "live_before", "live_after"):
    fields = {field.get("name"): field for field in release_report.get("fields", [])}
    if fields.get(name) is None:
        raise SystemExit(f"missing release report field: {name}")

print("[mimap164a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

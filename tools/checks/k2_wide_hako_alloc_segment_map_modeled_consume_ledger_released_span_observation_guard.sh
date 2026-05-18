#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-modeled-consume-ledger-released-span-observation"
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
    echo "[$TAG] ERROR: MIMAP-168A defers L3/L4 EXE evidence to a future closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-proof/test.sh"
CARD_167A="docs/development/current/main/phases/phase-293x/293x-689-MIMAP-167A-POST-SEGMENT-MAP-MODELED-CONSUME-LEDGER-RELEASED-TOKEN-RECYCLE-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-690-MIMAP-168A-SEGMENT-MAP-MODELED-CONSUME-LEDGER-RELEASED-SPAN-OBSERVATION-ROUTE.md"
CARD_169A="docs/development/current/main/phases/phase-293x/293x-691-MIMAP-169A-POST-SEGMENT-MAP-MODELED-CONSUME-LEDGER-RELEASED-SPAN-OBSERVATION-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-ssot.md"
RECYCLE_CLOSEOUT="docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-released-token-recycle-closeout-ssot.md"
RELEASED_SPAN_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-released-span-ledger-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
SPAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_span_observation_guard.sh"

printf '[%s] checking MIMAP-168A segment-map consume-ledger released-span observation\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_167A" \
  "$CARD" \
  "$CARD_169A" \
  "$DESIGN" \
  "$RECYCLE_CLOSEOUT" \
  "$RELEASED_SPAN_SSOT" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$MODULE" \
  "$OWNER" \
  "$SPAN_OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_167A" "MIMAP-167A must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-168A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$CARD_169A" "MIMAP-169A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-168A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-166A' "$RECYCLE_CLOSEOUT" "observation row must stay downstream of recycle closeout"
guard_expect_in_file "$TAG" 'MIMAP-107A' "$RELEASED_SPAN_SSOT" "observation row must reuse released-span ledger"
guard_expect_in_file "$TAG" 'MIMAP-168A' "$PLAN" "granularity SSOT must describe MIMAP-168A"
guard_expect_in_file "$TAG" 'MIMAP-168A segment-map modeled consume ledger released-span observation route' "$JOINT" "joint order must name MIMAP-168A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-168A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-168A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-168A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-168A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-168A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'memory.segment_map_accepted_readiness_modeled_consume_ledger_box = "memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"' "$MODULE" "hako module must export segment-map owner"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_released_span_ledger_box = "memory/segment_allocation_modeled_released_span_ledger_box.hako"' "$MODULE" "hako module must export released-span owner"
guard_expect_in_file "$TAG" 'MIMAP-168A' "$MEMORY_README" "memory README must define MIMAP-168A owner boundary"
guard_expect_in_file "$TAG" 'modeled_block_end' "$OWNER" "segment-map release report must expose block end"
guard_expect_in_file "$TAG" 'recordReleasedSpan' "$SPAN_OWNER" "released-span ledger must expose record route"
guard_expect_in_file "$TAG" 'check "mimap168a segment map modeled consume ledger released span observation route"' "$APP" "MIMAP-168A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$SPAN_OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-168A must keep real allocation/free, free-list mutation, raw pointer, concurrency, segment-map, atomics, page-source/OS release seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$SPAN_OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-168A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-proof|ReleasedSpanObservation|recordReleasedSpan' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-168A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_span_observation_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-168A guard must stay local-run/index-listed by default" >&2
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

tmp_dir="$(mktemp -d /tmp/hakorune_mimap168a_segment_map_released_span_observation.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap168a.mir.json"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-proof' "$vm_log"
rg -F -q 'release_first=1,0,0,70007002,70,7,2,5,3,1' "$vm_log"
rg -F -q 'first=1,0,0,-1,70007002,70,7,2,5,3,1,1' "$vm_log"
rg -F -q 'missing=0,2,-1,1' "$vm_log"
rg -F -q 'duplicate=0,3,0,1' "$vm_log"
rg -F -q 'recycled=1,0,1,-1,70007002,70,7,2,5,3,2,2' "$vm_log"
rg -F -q 'unsupported=0,4,1' "$vm_log"
rg -F -q 'counts=5,2,2,3,0,1,1,1' "$vm_log"
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
    "HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedger.releaseConsumedToken/2",
    "HakoAllocSegmentAllocationModeledReleasedSpanLedger.recordReleasedSpan/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
release_report = plans.get("HakoAllocSegmentMapModeledConsumeLedgerReleaseReport")
if release_report is None:
    raise SystemExit("missing segment-map release report typed object plan")
release_fields = {field.get("name"): field for field in release_report.get("fields", [])}
if "modeled_block_end" not in release_fields:
    raise SystemExit("missing segment-map release report modeled_block_end field")

span_report = plans.get("HakoAllocSegmentAllocationModeledReleasedSpanLedgerReport")
if span_report is None:
    raise SystemExit("missing released-span ledger report typed object plan")
span_fields = {field.get("name"): field for field in span_report.get("fields", [])}
for name in ("did_record_release_span", "modeled_block_end", "released_blocks", "would_mutate_free_list"):
    if name not in span_fields:
        raise SystemExit(f"missing released-span report field: {name}")

print("[mimap168a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

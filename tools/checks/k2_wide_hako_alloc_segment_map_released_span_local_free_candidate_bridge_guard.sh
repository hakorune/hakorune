#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-released-span-local-free-candidate-bridge"
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
    echo "[$TAG] ERROR: MIMAP-172A defers L3/L4 EXE evidence to a future closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-released-span-local-free-candidate-bridge-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-released-span-local-free-candidate-bridge-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-released-span-local-free-candidate-bridge-proof/test.sh"
CARD_171A="docs/development/current/main/phases/phase-293x/293x-693-MIMAP-171A-POST-SEGMENT-MAP-MODELED-CONSUME-LEDGER-RELEASED-SPAN-OBSERVATION-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-694-MIMAP-172A-SEGMENT-MAP-RELEASED-SPAN-LOCAL-FREE-CANDIDATE-BRIDGE.md"
CARD_173A="docs/development/current/main/phases/phase-293x/293x-695-MIMAP-173A-POST-SEGMENT-MAP-RELEASED-SPAN-LOCAL-FREE-CANDIDATE-BRIDGE-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-released-span-local-free-candidate-bridge-ssot.md"
OBS_CLOSEOUT="docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-closeout-ssot.md"
LOCAL_FREE_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
SPAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako"
CANDIDATE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_candidate_ledger_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_released_span_local_free_candidate_bridge_guard.sh"

printf '[%s] checking MIMAP-172A segment-map released-span local-free candidate bridge\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_171A" \
  "$CARD" \
  "$CARD_173A" \
  "$DESIGN" \
  "$OBS_CLOSEOUT" \
  "$LOCAL_FREE_SSOT" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$OWNER" \
  "$SPAN_OWNER" \
  "$CANDIDATE_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_171A" "MIMAP-171A must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-172A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$CARD_173A" "MIMAP-173A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-172A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-170A' "$OBS_CLOSEOUT" "bridge row must stay after released-span observation closeout"
guard_expect_in_file "$TAG" 'MIMAP-109A' "$LOCAL_FREE_SSOT" "bridge row must reuse local-free candidate ledger"
guard_expect_in_file "$TAG" 'MIMAP-172A' "$PLAN" "granularity SSOT must describe MIMAP-172A"
guard_expect_in_file "$TAG" 'MIMAP-172A segment-map released-span local-free candidate bridge' "$JOINT" "joint order must name MIMAP-172A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-172A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-172A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-172A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-172A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-172A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'MIMAP-172A' "$MEMORY_README" "memory README must define MIMAP-172A owner boundary"
guard_expect_in_file "$TAG" 'recordLocalFreeCandidate' "$CANDIDATE_OWNER" "local-free candidate ledger must expose record route"
guard_expect_in_file "$TAG" 'check "mimap172a segment map released span local free candidate bridge"' "$APP" "MIMAP-172A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$SPAN_OWNER" "$CANDIDATE_OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-172A must keep real free, free-list mutation, raw pointer, concurrency, segment-map, atomics, page-source/OS release seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$SPAN_OWNER" "$CANDIDATE_OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-172A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-released-span-local-free-candidate-bridge-proof|SegmentMapReleasedSpanLocalFreeCandidate|recordLocalFreeCandidate' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-172A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

pure_first_guard_build_hakorune_debug

tmp_dir="$(mktemp -d /tmp/hakorune_mimap172a_local_free_candidate_bridge.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap172a.mir.json"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-released-span-local-free-candidate-bridge-proof' "$vm_log"
rg -F -q 'span_first=1,0,0,-1,70007002,70,7,2,5,3' "$vm_log"
rg -F -q 'candidate_first=1,0,0,-1,70007002,70,7,2,5,3,1,1' "$vm_log"
rg -F -q 'candidate_missing=0,2,-1,1' "$vm_log"
rg -F -q 'candidate_duplicate=0,3,0,1' "$vm_log"
rg -F -q 'candidate_recycled=1,0,1,-1,70007002,70,7,2,5,3,2,2' "$vm_log"
rg -F -q 'candidate_unsupported=0,4,1' "$vm_log"
rg -F -q 'candidate_counts=5,2,2,3,0,1,1,1' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledReleasedSpanLedger.recordReleasedSpan/2",
    "HakoAllocSegmentAllocationModeledLocalFreeCandidateLedger.recordLocalFreeCandidate/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
candidate_report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeCandidateLedgerReport")
if candidate_report is None:
    raise SystemExit("missing local-free candidate report typed object plan")
fields = {field.get("name"): field for field in candidate_report.get("fields", [])}
for name in ("did_record_local_free_candidate", "modeled_block_end", "candidate_blocks", "would_mutate_free_list"):
    if name not in fields:
        raise SystemExit(f"missing local-free candidate report field: {name}")

print("[mimap172a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

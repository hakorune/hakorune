#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-modeled-consume-ledger-release"
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
    echo "[$TAG] ERROR: MIMAP-161A defers L3/L4 EXE evidence to the release closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-modeled-consume-ledger-release-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-modeled-consume-ledger-release-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-modeled-consume-ledger-release-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-683-MIMAP-161A-SEGMENT-MAP-MODELED-CONSUME-LEDGER-RELEASE-ROUTE.md"
CARD_162A="docs/development/current/main/phases/phase-293x/293x-684-MIMAP-162A-SEGMENT-MAP-MODELED-CONSUME-LEDGER-RELEASE-CLOSEOUT-PACK.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-release-ssot.md"
CONSUME_LEDGER_SSOT="docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-closeout-ssot.md"
RELEASE_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-release-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_release_guard.sh"

printf '[%s] checking MIMAP-161A segment-map modeled consume ledger release route\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$CARD_162A" \
  "$DESIGN" \
  "$CONSUME_LEDGER_SSOT" \
  "$RELEASE_SSOT" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$MODULE" \
  "$OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-161A card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_162A" "MIMAP-162A must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-161A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-157A' "$CONSUME_LEDGER_SSOT" "release route must stay downstream of consume ledger pack"
guard_expect_in_file "$TAG" 'MIMAP-097A' "$RELEASE_SSOT" "release route must reuse modeled ledger release substrate"
guard_expect_in_file "$TAG" 'MIMAP-161A' "$PLAN" "granularity SSOT must describe MIMAP-161A"
guard_expect_in_file "$TAG" 'MIMAP-161A segment-map modeled consume ledger release route' "$JOINT" "joint order must name MIMAP-161A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-161A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-161A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-161A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-161A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-161A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'memory.segment_map_accepted_readiness_modeled_consume_ledger_box = "memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"' "$MODULE" "hako module must export owner"
guard_expect_in_file "$TAG" 'MIMAP-161A' "$MEMORY_README" "memory README must define MIMAP-161A owner"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentMapModeledConsumeLedgerReleaseReport' "$OWNER" "MIMAP-161A release report box must exist"
guard_expect_in_file "$TAG" 'releaseConsumedToken' "$OWNER" "MIMAP-161A owner must expose release route"
guard_expect_in_file "$TAG" 'HakoAllocSegmentAllocationModeledLedger' "$OWNER" "MIMAP-161A must reuse modeled ledger owner"
guard_expect_in_file "$TAG" 'check "mimap161a segment map modeled consume ledger release route"' "$APP" "MIMAP-161A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-161A must keep real free, raw pointer, concurrency, segment-map, atomics, page-source/OS release seams, and backend-visible page release inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-161A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-modeled-consume-ledger-release-proof|HakoAllocSegmentMapModeledConsumeLedgerRelease|releaseConsumedToken' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-161A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_map_modeled_consume_ledger_release_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-161A guard must stay local-run/index-listed by default" >&2
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

tmp_dir="$(mktemp -d /tmp/hakorune_mimap161a_segment_map_consume_ledger_release.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap161a.mir.json"
build_log="$tmp_dir/build.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-modeled-consume-ledger-release-proof' "$vm_log"
rg -F -q 'release_first=1,0,0,0,70007002,70,7,2,1,0,1,0,3,1' "$vm_log"
rg -F -q 'blocked=0,4,4,-1,70007002' "$vm_log"
rg -F -q 'rejects=1,2,3,4,1,2,3,4' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'release_counts=5,1,4,1,1,1,1,70007002,4,0' "$vm_log"
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
    "HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedger.releaseReport/12",
    "HakoAllocSegmentAllocationModeledLedger.releaseModeledToken/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentMapModeledConsumeLedgerReleaseReport")
if report is None:
    raise SystemExit("missing segment-map consume-ledger release report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "did_release",
    "reason",
    "release_reason",
    "row_index",
    "modeled_allocation_token",
    "segment_id",
    "page_id",
    "modeled_block_start",
    "live_before",
    "live_after",
    "released_blocks",
    "release_span_present",
    "would_execute_real_segment_free",
    "would_use_raw_pointer",
    "would_use_segment_map",
):
    field = fields.get(name)
    if field is None:
        raise SystemExit(f"missing release report field: {name}")
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad release report field {name}: {field}")

print("[mimap161a-mir-json] ok")
PY

pure_first_guard_route_preflight "$TAG" "$ROOT_DIR" "$mir_json" "$build_log"
printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

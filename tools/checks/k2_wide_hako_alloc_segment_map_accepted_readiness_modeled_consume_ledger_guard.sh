#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger"
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
    echo "[$TAG] ERROR: MIMAP-157A defers L3/L4 EXE evidence to the consume-ledger closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-679-MIMAP-157A-SEGMENT-MAP-ACCEPTED-READINESS-MODELED-CONSUME-LEDGER-ROUTE.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger-ssot.md"
DIAGNOSTICS_DESIGN="docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-diagnostics-ssot.md"
CARD_158A="docs/development/current/main/phases/phase-293x/293x-680-MIMAP-158A-SEGMENT-MAP-MODELED-CONSUME-LEDGER-DIAGNOSTICS.md"
CARD_159A="docs/development/current/main/phases/phase-293x/293x-681-MIMAP-159A-SEGMENT-MAP-MODELED-CONSUME-LEDGER-CLOSEOUT-PACK.md"
COMPOSITION_SSOT="docs/development/current/main/design/hako-alloc-segment-map-lookup-guarded-readiness-composition-ssot.md"
CONSUME_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-consume-ssot.md"
LEDGER_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
COMPOSITION_OWNER="lang/src/hako_alloc/memory/segment_map_lookup_guarded_readiness_composition_box.hako"
CONSUME_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_consume_box.hako"
LEDGER_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_accepted_readiness_modeled_consume_ledger_guard.sh"

printf '[%s] checking MIMAP-157A segment-map accepted readiness modeled consume ledger route\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$DIAGNOSTICS_DESIGN" \
  "$CARD_158A" \
  "$CARD_159A" \
  "$COMPOSITION_SSOT" \
  "$CONSUME_SSOT" \
  "$LEDGER_SSOT" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$MODULE" \
  "$OWNER" \
  "$COMPOSITION_OWNER" \
  "$CONSUME_OWNER" \
  "$LEDGER_OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-157A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-157A design must be accepted"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_158A" "MIMAP-158A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$CARD_159A" "MIMAP-159A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DIAGNOSTICS_DESIGN" "MIMAP-158A diagnostics design must be accepted"
guard_expect_in_file "$TAG" 'blocked' "$DIAGNOSTICS_DESIGN" "MIMAP-158A diagnostics must define blocked"
guard_expect_in_file "$TAG" 'duplicate' "$DIAGNOSTICS_DESIGN" "MIMAP-158A diagnostics must define duplicate"
guard_expect_in_file "$TAG" 'stale' "$DIAGNOSTICS_DESIGN" "MIMAP-158A diagnostics must define stale"
guard_expect_in_file "$TAG" 'Decision: accepted' "$COMPOSITION_SSOT" "MIMAP-153A composition SSOT must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$CONSUME_SSOT" "MIMAP-091A consume SSOT must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$LEDGER_SSOT" "MIMAP-094A ledger SSOT must stay accepted"
guard_expect_in_file "$TAG" 'MIMAP-157A granularity' "$PLAN" "granularity SSOT must describe MIMAP-157A"
guard_expect_in_file "$TAG" 'MIMAP-157A segment-map accepted readiness modeled consume ledger route' "$JOINT" "joint order must name MIMAP-157A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-157A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-157A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-157A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-157A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-157A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'memory.segment_map_accepted_readiness_modeled_consume_ledger_box = "memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"' "$MODULE" "hako module must export MIMAP-157A owner"
guard_expect_in_file "$TAG" 'MIMAP-157A. It may compose an accepted MIMAP-153A readiness report' "$MEMORY_README" "memory README must define MIMAP-157A owner"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedgerReport' "$OWNER" "MIMAP-157A report box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedger' "$OWNER" "MIMAP-157A owner must exist"
guard_expect_in_file "$TAG" 'consumeAcceptedReadiness' "$OWNER" "MIMAP-157A owner must expose consumeAcceptedReadiness"
guard_expect_in_file "$TAG" 'HakoAllocSegmentAllocationModeledConsume' "$OWNER" "MIMAP-157A must compose modeled consume owner"
guard_expect_in_file "$TAG" 'HakoAllocSegmentAllocationModeledLedger' "$OWNER" "MIMAP-157A must compose modeled ledger owner"
guard_expect_in_file "$TAG" 'diagnosticBlocked' "$OWNER" "MIMAP-158A owner must expose blocked diagnostic"
guard_expect_in_file "$TAG" 'diagnosticDuplicate' "$OWNER" "MIMAP-158A owner must expose duplicate diagnostic"
guard_expect_in_file "$TAG" 'diagnosticStale' "$OWNER" "MIMAP-158A owner must expose stale diagnostic"
guard_expect_in_file "$TAG" 'check "mimap157a segment map accepted readiness modeled consume ledger route"' "$APP" "MIMAP-157A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-157A must not add real execution, raw pointer lookup, atomics, page-source/OS release seams, or backend-visible release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-157A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger-proof|HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedger|segment_map_accepted_readiness_modeled_consume_ledger' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-157A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_map_accepted_readiness_modeled_consume_ledger_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-157A guard must stay local-run/index-listed by default" >&2
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

tmp_dir="$(mktemp -d /tmp/hakorune_mimap157a_segment_map_consume_ledger.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap157a.mir.json"
build_log="$tmp_dir/build.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger-proof' "$vm_log"
rg -F -q 'consumed=1,0,0,0,0,0,-1,70,7,2,3,5,3,2,70007002,1,1' "$vm_log"
rg -F -q 'rejected=0,1,3,-1,-1,70,7' "$vm_log"
rg -F -q 'diagnostics=1,2,3,4,4,5' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'counts=5,1,4,2,1,1,1,1,1,1,1,70007002,3' "$vm_log"
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
    "HakoAllocSegmentMapLookupGuardedReadinessComposition.composeLookupReadiness/10",
    "HakoAllocSegmentAllocationModeledConsume.consumeReadiness/8",
    "HakoAllocSegmentAllocationModeledLedger.recordModeledConsume/12",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedger",
    "HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedgerReport",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

fields = {
    field.get("name"): field
    for field in plans["HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedgerReport"].get("fields", [])
}
required_fields = {
    "accepted",
    "reason",
    "upstream_reason",
    "lookup_reason",
    "membership_reason",
    "readiness_reason",
    "consume_reason",
    "ledger_reason",
    "diagnostic_kind",
    "row_index",
    "existing_index",
    "segment_id",
    "page_id",
    "modeled_allocation_token",
    "ledger_count_after",
    "ledger_live_count_after",
    "modeled_consume_present",
    "modeled_ledger_present",
    "would_use_raw_pointer",
    "would_use_real_segment_map",
    "would_add_backend_matcher",
}
missing_fields = sorted(name for name in required_fields if name not in fields)
if missing_fields:
    raise SystemExit(f"missing report fields: {missing_fields}")

for name in required_fields:
    field = fields[name]
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad report field {name}: {field}")

print("[mimap157a-mir-json] ok")
PY

pure_first_guard_route_preflight "$TAG" "$ROOT_DIR" "$mir_json" "$build_log"
printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic"
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
    echo "[$TAG] ERROR: MIMAP-208A defers L3/L4 EXE evidence to a future closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-proof/test.sh"
CARD_206A="docs/development/current/main/phases/phase-293x/293x-729-MIMAP-206A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLIED-RECYCLE-BRIDGE-CLOSEOUT-PACK.md"
CARD_207A="docs/development/current/main/phases/phase-293x/293x-730-MIMAP-207A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLIED-RECYCLE-BRIDGE-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-731-MIMAP-208A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLIED-RECYCLE-SECOND-RELEASE-DIAGNOSTIC.md"
CARD_209A="docs/development/current/main/phases/phase-293x/293x-732-MIMAP-209A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLIED-RECYCLE-SECOND-RELEASE-DIAGNOSTIC-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-ssot.md"
CLOSEOUT="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge-closeout-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
SOURCE_LEDGER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"
RELEASE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_release_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_applied_recycle_second_release_diagnostic_guard.sh"

printf '[%s] checking MIMAP-208A release-applied recycle second-release diagnostic\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_206A" \
  "$CARD_207A" \
  "$CARD" \
  "$CARD_209A" \
  "$DESIGN" \
  "$CLOSEOUT" \
  "$PLAN" \
  "$JOINT" \
  "$CADENCE" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$SOURCE_LEDGER" \
  "$RELEASE_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_206A" "MIMAP-206A must be landed before second-release diagnostic"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_207A" "MIMAP-207A selection must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-208A card must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$CARD_209A" "MIMAP-209A must be selected current after diagnostic"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-208A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-206A' "$CLOSEOUT" "second-release diagnostic must follow closeout"
guard_expect_in_file "$TAG" 'one-release-per-modeled-reuse-token' "$DESIGN" "design must name one-release boundary"
guard_expect_in_file "$TAG" 'generation/lifecycle token' "$DESIGN" "design must defer generation/lifecycle token"
guard_expect_in_file "$TAG" 'MIMAP-208A' "$PLAN" "granularity SSOT must describe MIMAP-208A"
guard_expect_in_file "$TAG" 'MIMAP-208A segment-map local-free reuse ledger release-applied recycle second-release diagnostic' "$JOINT" "joint order must name MIMAP-208A"
guard_expect_in_file "$TAG" 'MIMAP-209A' "$JOINT" "joint order must name MIMAP-209A"
guard_expect_in_file "$TAG" 'MIMAP-209A' "$CADENCE" "cadence SSOT must name next diagnostic closeout boundary"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-208A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-208A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-208A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-208A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-208A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'recordLocalFreeReuse' "$SOURCE_LEDGER" "source ledger must keep recycle route"
guard_expect_in_file "$TAG" 'applyReuseLedgerRelease' "$SOURCE_LEDGER" "source ledger must keep apply route"
guard_expect_in_file "$TAG" 'recordReuseLedgerRelease' "$RELEASE_OWNER" "release owner must keep release route"
guard_expect_in_file "$TAG" 'duplicate_reject_count' "$RELEASE_OWNER" "release owner must keep duplicate diagnostic counter"
guard_expect_in_file "$TAG" 'check "mimap208a segment map local free reuse ledger release-applied recycle second-release diagnostic"' "$APP" "MIMAP-208A proof must use labelled check block"

if rg -n 'generation_id|lifecycle_id|generationToken|lifecycleToken|recordModeledConsume|releaseModeledToken|segment_allocation_modeled_ledger_box' \
  "$APP" "$SOURCE_LEDGER" "$RELEASE_OWNER" >/tmp/"$TAG".generation_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-208A must not introduce generation/lifecycle or bump-ledger dependency" >&2
  cat /tmp/"$TAG".generation_leak >&2
  rm -f /tmp/"$TAG".generation_leak
  exit 1
fi
rm -f /tmp/"$TAG".generation_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$APP" "$SOURCE_LEDGER" "$RELEASE_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-208A must keep real execution/raw pointer/concurrency/segment-map/atomics/page-source/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-proof|SecondReleaseDiagnostic|secondReleaseDiagnostic' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-208A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap208_second_release_diagnostic.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap208.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-proof' "$vm_log"
rg -F -q 'first=1,0,0,70007004,1' "$vm_log"
rg -F -q 'apply=1,0,0,70007004,0' "$vm_log"
rg -F -q 'recycle=1,0,1,70007004,1' "$vm_log"
rg -F -q 'second_release=0,3,0,70007004' "$vm_log"
rg -F -q 'reads=-1,70007004,4' "$vm_log"
rg -F -q 'reuse_counts=2,2,0,2,1,1,1,0' "$vm_log"
rg -F -q 'release_counts=2,1,1,1,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
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
    "Main.nextReuseReport/4",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedger.recordLocalFreeReuse/2",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedger.applyReuseLedgerRelease/2",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerRelease.recordReuseLedgerRelease/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
release_report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReleaseReport")
if release_report is None:
    raise SystemExit("missing local-free reuse ledger release report typed object plan")

fields = {field.get("name"): field for field in release_report.get("fields", [])}
for name in ("did_release", "reason", "existing_index", "modeled_reuse_token"):
    if name not in fields:
        raise SystemExit(f"missing release report field: {name}")

print("[mimap208a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

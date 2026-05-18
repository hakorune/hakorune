#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-229A defers L3/L4 EXE evidence to the MIMAP-230A closeout pack" >&2
    exit 2
    ;;
  *)
    echo "[$TAG] ERROR: unsupported validation level: $VALIDATION_LEVEL" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-diagnostics-proof/test.sh"
CARD_228A="docs/development/current/main/phases/phase-293x/293x-751-MIMAP-228A-SOURCE-RELEASE-LEDGER-LIFECYCLE-KEY-MIGRATION-PILOT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-752-MIMAP-229A-SOURCE-LIFECYCLE-KEYED-RELEASE-LEDGER-DIAGNOSTICS.md"
CARD_230A="docs/development/current/main/phases/phase-293x/293x-753-MIMAP-230A-SOURCE-RELEASE-LEDGER-LIFECYCLE-KEY-MIGRATION-CLOSEOUT-PACK.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-diagnostics-ssot.md"
LEDGER_DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
LEDGER_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_ledger_box.hako"
DIAGNOSTIC_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_ledger_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_ledger_diagnostics_guard.sh"

printf '[%s] checking MIMAP-229A source lifecycle-keyed release ledger diagnostics\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_228A" \
  "$CARD" \
  "$CARD_230A" \
  "$DESIGN" \
  "$LEDGER_DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$CADENCE" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$LEDGER_OWNER" \
  "$DIAGNOSTIC_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_228A" "MIMAP-228A must be landed before diagnostics"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-229A diagnostics card must be landed"
guard_expect_in_file "$TAG" 'Status: (landed|selected current)' "$CARD_230A" "MIMAP-230A must be selected current or landed after MIMAP-229A"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-229A diagnostics design must be accepted"
guard_expect_in_file "$TAG" 'does not mutate either the old modeled-reuse-token' "$DESIGN" "diagnostics design must stay observer-only"
guard_expect_in_file "$TAG" 'Decision: accepted' "$LEDGER_DESIGN" "MIMAP-228A ledger design must stay accepted"
guard_expect_in_file "$TAG" 'MIMAP-229A' "$PLAN" "granularity SSOT must describe MIMAP-229A"
guard_expect_in_file "$TAG" 'MIMAP-230A' "$PLAN" "granularity SSOT must describe MIMAP-230A"
guard_expect_in_file "$TAG" 'MIMAP-229A source lifecycle-keyed release ledger diagnostics' "$JOINT" "joint order must name MIMAP-229A"
guard_expect_in_file "$TAG" 'MIMAP-230A source release-ledger lifecycle-key migration closeout pack' "$JOINT" "joint order must name MIMAP-230A"
guard_expect_in_file "$TAG" 'source release-ledger lifecycle-key migration family' "$CADENCE" "cadence SSOT must define migration family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-229A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-229A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-229A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-229A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-229A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_ledger_diagnostic_box' "$MODULE" "module must export diagnostics owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_ledger_diagnostic_box.hako' "$MEMORY_README" "memory README must name diagnostics owner"
guard_expect_in_file "$TAG" 'observeReleaseLedgerDiagnostics' "$DIAGNOSTIC_OWNER" "diagnostic owner must expose observer route"
guard_expect_in_file "$TAG" 'reject_summary_present: i64 = 1' "$DIAGNOSTIC_OWNER" "diagnostic report must publish reject summary presence"
guard_expect_in_file "$TAG" 'check "mimap229a source lifecycle-keyed release ledger diagnostics"' "$APP" "MIMAP-229A proof must use labelled check block"

if rg -n 'recordLifecycleKeyedRelease|reuse_lifecycle_tokens\.push|modeled_reuse_tokens\.push|lifecycle_ids\.push' \
  "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".mutation_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-229A diagnostic owner must not mutate release ledgers" >&2
  cat /tmp/"$TAG".mutation_leak >&2
  rm -f /tmp/"$TAG".mutation_leak
  exit 1
fi
rm -f /tmp/"$TAG".mutation_leak

if rg -n 'realLifecycle|generationToken|migrateReleaseLedger|releaseLedgerKeyMigration' \
  "$APP" "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".real_lifecycle_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-229A must not define real lifecycle/generation or broad migration machinery" >&2
  cat /tmp/"$TAG".real_lifecycle_leak >&2
  rm -f /tmp/"$TAG".real_lifecycle_leak
  exit 1
fi
rm -f /tmp/"$TAG".real_lifecycle_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$APP" "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-229A must keep real execution/raw pointer/concurrency/segment-map/atomics/page-source/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-diagnostics-proof|LifecycleKeyedReleaseLedgerDiagnostic|lifecycleKeyedReleaseLedgerDiagnostic' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-229A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap229_lifecycle_release_ledger_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap229.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-diagnostics-proof' "$vm_log"
rg -F -q 'base=70007004,70007004002,6' "$vm_log"
rg -F -q 'summary=1,0,6,1,1,5,1,1,1,1,1,5,70007004002' "$vm_log"
rg -F -q 'rejects=0,1,0,2' "$vm_log"
rg -F -q 'counts=3,1,2,1,1' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleKeyedReleaseLedger.recordLifecycleKeyedRelease/3",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleKeyedReleaseLedgerDiagnostic.observeReleaseLedgerDiagnostics/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleKeyedReleaseLedgerDiagnosticReport")
if report is None:
    raise SystemExit("missing lifecycle-keyed release ledger diagnostic report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "observed",
    "reason",
    "attempt_count",
    "ledger_count",
    "accepted_count",
    "reject_count",
    "duplicate_lifecycle_key_seen",
    "precondition_reject_seen",
    "lifecycle_report_reject_seen",
    "token_mismatch_reject_seen",
    "unsupported_requirement_reject_seen",
    "reject_summary_present",
    "would_use_raw_pointer",
    "would_use_segment_map",
    "would_activate_provider",
):
    if name not in fields:
        raise SystemExit(f"missing lifecycle-keyed release ledger diagnostic report field: {name}")

print("[mimap229a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

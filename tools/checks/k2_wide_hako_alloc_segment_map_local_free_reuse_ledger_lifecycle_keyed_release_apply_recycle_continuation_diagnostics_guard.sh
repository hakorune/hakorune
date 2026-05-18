#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-233A defers L3/L4 EXE evidence to the MIMAP-234A closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-diagnostics-proof/test.sh"
CARD_232A="docs/development/current/main/phases/phase-293x/293x-755-MIMAP-232A-SOURCE-LIFECYCLE-KEYED-RELEASE-APPLY-RECYCLE-CONTINUATION-BRIDGE.md"
CARD="docs/development/current/main/phases/phase-293x/293x-756-MIMAP-233A-SOURCE-LIFECYCLE-KEYED-RELEASE-APPLY-RECYCLE-CONTINUATION-DIAGNOSTICS.md"
CARD_234A="docs/development/current/main/phases/phase-293x/293x-757-MIMAP-234A-SOURCE-LIFECYCLE-KEYED-RELEASE-APPLY-RECYCLE-CONTINUATION-CLOSEOUT-PACK.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-diagnostics-ssot.md"
BRIDGE_DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
REUSE_LEDGER_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"
DIAGNOSTIC_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_apply_recycle_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_apply_recycle_continuation_diagnostics_guard.sh"

printf '[%s] checking MIMAP-233A lifecycle-keyed release apply/recycle continuation diagnostics\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_232A" \
  "$CARD" \
  "$CARD_234A" \
  "$DESIGN" \
  "$BRIDGE_DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$CADENCE" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$REUSE_LEDGER_OWNER" \
  "$DIAGNOSTIC_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_232A" "MIMAP-232A bridge must be landed before diagnostics"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-233A diagnostics card must be landed"
guard_expect_in_file "$TAG" 'Status: (landed|selected current)' "$CARD_234A" "MIMAP-234A must be selected current or landed after MIMAP-233A"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-233A diagnostics design must be accepted"
guard_expect_in_file "$TAG" 'observer-only diagnostic owner' "$CARD" "MIMAP-233A card must call out observer-only diagnostics"
guard_expect_in_file "$TAG" 'Decision: accepted' "$BRIDGE_DESIGN" "MIMAP-232A bridge design must stay accepted"
guard_expect_in_file "$TAG" 'MIMAP-233A' "$PLAN" "granularity SSOT must describe MIMAP-233A"
guard_expect_in_file "$TAG" 'MIMAP-234A' "$PLAN" "granularity SSOT must describe MIMAP-234A"
guard_expect_in_file "$TAG" 'MIMAP-233A source lifecycle-keyed release apply/recycle continuation diagnostics' "$JOINT" "joint order must name MIMAP-233A"
guard_expect_in_file "$TAG" 'source lifecycle-keyed release apply/recycle continuation family' "$CADENCE" "cadence SSOT must define continuation family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-233A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-233A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-233A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-233A must stay on scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-233A EXE evidence must be deferred to closeout"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_apply_recycle_diagnostic_box' "$MODULE" "module must export diagnostics owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_apply_recycle_diagnostic_box.hako' "$MEMORY_README" "memory README must name diagnostics owner"
guard_expect_in_file "$TAG" 'observeApplyRecycleDiagnostics' "$DIAGNOSTIC_OWNER" "diagnostic owner must expose observer route"
guard_expect_in_file "$TAG" 'continuation_diagnostic_present: i64 = 1' "$DIAGNOSTIC_OWNER" "diagnostic report must publish presence bit"
guard_expect_in_file "$TAG" 'check "mimap233a source lifecycle-keyed release apply recycle continuation diagnostics"' "$APP" "MIMAP-233A proof must use labelled check block"

if rg -n 'recordLocalFreeReuse|applyReuseLedger|applyReuseLedgerLifecycleKeyedRelease|recordLifecycleKeyedRelease|tokens\.push|live_flags\.set' \
  "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".mutation_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-233A diagnostic owner must not mutate reuse/release ledgers" >&2
  cat /tmp/"$TAG".mutation_leak >&2
  rm -f /tmp/"$TAG".mutation_leak
  exit 1
fi
rm -f /tmp/"$TAG".mutation_leak

if rg -n 'realLifecycle|generationToken|migrateReleaseLedger|releaseLedgerKeyMigration' \
  "$APP" "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".real_lifecycle_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-233A must not define real lifecycle/generation or broad migration machinery" >&2
  cat /tmp/"$TAG".real_lifecycle_leak >&2
  rm -f /tmp/"$TAG".real_lifecycle_leak
  exit 1
fi
rm -f /tmp/"$TAG".real_lifecycle_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$APP" "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-233A must keep real execution/raw pointer/concurrency/segment-map/atomics/page-source/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-diagnostics-proof|LifecycleKeyedReleaseApplyRecycleDiagnostic|applyRecycleDiagnostic' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-233A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap233_lifecycle_apply_recycle_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap233.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-diagnostics-proof' "$vm_log"
rg -F -q 'base=70007004,70007004002,4,5' "$vm_log"
rg -F -q 'diag=1,0,4,2,2,1,1,1,1,3,1,5,70007004' "$vm_log"
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
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLedger.applyReuseLedgerLifecycleKeyedRelease/2",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleKeyedReleaseApplyRecycleDiagnostic.observeApplyRecycleDiagnostics/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleKeyedReleaseApplyRecycleDiagnosticReport")
if report is None:
    raise SystemExit("missing lifecycle-keyed release apply/recycle diagnostic report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "observed",
    "reason",
    "release_apply_attempt_count",
    "release_apply_missing_reject_count",
    "release_apply_unsupported_seen",
    "post_continuation_duplicate_seen",
    "lifecycle_keyed_apply_seen",
    "continuation_diagnostic_present",
    "would_use_raw_pointer",
    "would_use_segment_map",
    "would_activate_provider",
):
    if name not in fields:
        raise SystemExit(f"missing lifecycle-keyed release apply/recycle diagnostic report field: {name}")

print("[mimap233a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

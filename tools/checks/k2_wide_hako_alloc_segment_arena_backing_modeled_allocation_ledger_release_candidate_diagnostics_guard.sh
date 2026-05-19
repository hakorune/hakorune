#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-diagnostics"
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
    echo "[$TAG] ERROR: MIMAP-281A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-diagnostics-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-diagnostics-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-diagnostics-proof/test.sh"
CARD_280A="docs/development/current/main/phases/phase-293x/293x-805-MIMAP-280A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-CANDIDATE-INVENTORY.md"
CARD="docs/development/current/main/phases/phase-293x/293x-806-MIMAP-281A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-CANDIDATE-DIAGNOSTICS.md"
DESIGN_280A="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-diagnostics-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
INVENTORY_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_candidate_box.hako"
DIAGNOSTIC_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_candidate_diagnostic_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_diagnostics_guard.sh"

printf '[%s] checking MIMAP-281A segment arena backing modeled allocation-ledger release candidate diagnostics\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_280A" \
  "$CARD" \
  "$DESIGN_280A" \
  "$DESIGN" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$INVENTORY_OWNER" \
  "$DIAGNOSTIC_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_280A" "MIMAP-280A release candidate inventory must be landed before diagnostics"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-281A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN_280A" "MIMAP-280A release candidate design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-281A diagnostics design must be accepted"
guard_expect_in_file "$TAG" 'observer-only' "$CARD" "MIMAP-281A card must call out observer-only diagnostics"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-281A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-281A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-281A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-281A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-281A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_allocation_ledger_release_candidate_diagnostic_box' "$MODULE" "module must export diagnostic owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_ledger_release_candidate_diagnostic_box.hako' "$MEMORY_README" "memory README must name diagnostic owner"
guard_expect_in_file "$TAG" 'record HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnosticReportFields' "$DIAGNOSTIC_OWNER" "diagnostic owner must use local ReportFields record payload"
guard_expect_in_file "$TAG" 'observeReleaseCandidateDiagnostics' "$DIAGNOSTIC_OWNER" "diagnostic owner must expose observer route"
guard_expect_in_file "$TAG" 'diagnostic_present: i64 = 1' "$DIAGNOSTIC_OWNER" "diagnostic report must publish presence bit"
guard_expect_in_file "$TAG" 'check "mimap281a segment arena backing modeled allocation ledger release candidate diagnostics"' "$APP" "proof must use labelled check block"

if rg -n 'recordReleaseCandidate|me\.(inventory_count|accepted_count|reject_count|missing_ledger_reject_count|rejected_ledger_reject_count|invalid_release_candidate_token_reject_count|duplicate_release_candidate_token_reject_count|closed_substrate_reject_count)[[:space:]]*\+=' \
  "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".mutation_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-281A diagnostic owner must not record release-candidate rows or mutate inventory counters" >&2
  cat /tmp/"$TAG".mutation_leak >&2
  rm -f /tmp/"$TAG".mutation_leak
  exit 1
fi
rm -f /tmp/"$TAG".mutation_leak

if rg -n 'lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$DIAGNOSTIC_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-281A must keep pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-diagnostics-proof|ModeledAllocationLedgerReleaseCandidateDiagnostic|modeledAllocationLedgerReleaseCandidateDiagnostic' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-281A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap281_release_candidate_diag.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap281.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-diagnostics-proof' "$vm_log"
rg -F -q 'diag=1,0,6,1,5,150,15,94015005005' "$vm_log"
rg -F -q 'seen=1,1,1,1,1' "$vm_log"
rg -F -q 'candidate=93015005005,94015005005,4096,4096,8192' "$vm_log"
rg -F -q 'present=1,1,1' "$vm_log"
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
    "Main.makeLedger/2",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateInventory.recordReleaseCandidate/2",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnostic.observeReleaseCandidateDiagnostics/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
record_names = set()
for decl in data.get("record_decls", []):
    if isinstance(decl, str):
        record_names.add(decl)
    elif isinstance(decl, dict):
        record_names.add(decl.get("name"))
if "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnosticReportFields" not in record_names:
    raise SystemExit("missing modeled release-candidate diagnostic ReportFields record declaration")

report = plans.get("HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnosticReport")
if report is None:
    raise SystemExit("missing modeled release-candidate diagnostic report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "observed",
    "reason",
    "diagnostic_present",
    "inventory_count",
    "accepted_count",
    "reject_count",
    "duplicate_release_candidate_token_reject_seen",
    "release_candidate_present",
    "modeled_release_candidate_present",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing modeled release-candidate diagnostic field: {name}")

print("[mimap281a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

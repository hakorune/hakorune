#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent"
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
    echo "[$TAG] ERROR: MIMAP-284A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent-proof/test.sh"
CARD_282A="docs/development/current/main/phases/phase-293x/293x-808-MIMAP-282A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-CANDIDATE-CLOSEOUT.md"
CARD_283A="docs/development/current/main/phases/phase-293x/293x-886-MIMAP-283A-POST-REPORTFIELDS-CLEANUP-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-887-MIMAP-284A-SEGMENT-ARENA-BACKING-MODELED-ALLOCATION-LEDGER-RELEASE-INTENT-INVENTORY.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CANDIDATE_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_candidate_box.hako"
INTENT_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_intent_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_intent_guard.sh"

printf '[%s] checking MIMAP-284A segment arena backing modeled allocation-ledger release intent\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_282A" \
  "$CARD_283A" \
  "$CARD" \
  "$DESIGN" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CANDIDATE_OWNER" \
  "$INTENT_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_282A" "MIMAP-282A release-candidate closeout must be landed before release intent"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_283A" "MIMAP-283A selection must be landed before release intent"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-284A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-284A release intent design must be accepted"
guard_expect_in_file "$TAG" 'release-intent' "$CARD" "MIMAP-284A card must call out release intent"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-284A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-284A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-284A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-284A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-284A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_allocation_ledger_release_intent_box' "$MODULE" "module must export release intent owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_ledger_release_intent_box.hako' "$MEMORY_README" "memory README must name release intent owner"
guard_expect_in_file "$TAG" 'record HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseIntentReportFields' "$INTENT_OWNER" "release intent owner must use local ReportFields record payload"
guard_expect_in_file "$TAG" 'local fields = HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseIntentReportFields' "$INTENT_OWNER" "release intent owner must construct report field records locally"
guard_expect_in_file "$TAG" 'makeReleaseIntentReport' "$INTENT_OWNER" "release intent owner must expose ReportFields helper-argument scalarization helper"
guard_expect_in_file "$TAG" 'return me.makeReleaseIntentReport' "$INTENT_OWNER" "release intent makeReport must delegate report copy through ReportFields helper"
guard_expect_in_file "$TAG" 'source_capacity: usize' "$INTENT_OWNER" "release intent byte/capacity group must use usize source_capacity"
guard_expect_in_file "$TAG" 'applied_backing_bytes: usize' "$INTENT_OWNER" "release intent byte/capacity group must use usize applied_backing_bytes"
guard_expect_in_file "$TAG" 'remaining_source_bytes: usize' "$INTENT_OWNER" "release intent byte/capacity group must use usize remaining_source_bytes"
guard_expect_in_file "$TAG" 'row_index: i64' "$INTENT_OWNER" "release intent row_index sentinel must stay i64"
guard_expect_in_file "$TAG" 'release_intent_token: i64' "$INTENT_OWNER" "release intent token must stay i64"
guard_expect_in_file "$TAG" 'recordReleaseIntent' "$INTENT_OWNER" "release intent owner must expose record route"
guard_expect_in_file "$TAG" 'duplicate_release_intent_token_reject_count' "$INTENT_OWNER" "release intent owner must reject duplicate token"
guard_expect_in_file "$TAG" 'check "mimap284a segment arena backing modeled allocation ledger release intent"' "$APP" "proof must use labelled check block"

if rg -n 'lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$INTENT_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-284A must keep pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent-proof|ReleaseIntent|releaseIntent' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-284A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap284_release_intent.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap284.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-intent-proof' "$vm_log"
rg -F -q 'intent=1,0,1,140,14,3,70014005005,1,90014005005,91014005005,92014005005,93014005005,94014005005,95014005005,4096,4096,12288,8192' "$vm_log"
rg -F -q 'rejects=1,2,3,4,5' "$vm_log"
rg -F -q 'counts=6,1,5,1,1,1,1,1,5,95014005005' "$vm_log"
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
    "Main.makeCandidate/3",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateInventory.recordReleaseCandidate/2",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseIntentInventory.makeReleaseIntentReport/1",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseIntentInventory.recordReleaseIntent/2",
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
if "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseIntentReportFields" not in record_names:
    raise SystemExit("missing modeled release intent ReportFields record declaration")

report = plans.get("HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseIntentReport")
if report is None:
    raise SystemExit("missing modeled release intent report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
required_fields = (
    "accepted",
    "reason",
    "release_intent_present",
    "modeled_release_intent_present",
    "release_candidate_token",
    "release_intent_token",
    "applied_backing_bytes",
    "applied_committed_bytes",
    "remaining_source_bytes",
    "would_add_backend_matcher",
)
for name in required_fields:
    if name not in fields:
        raise SystemExit(f"missing modeled release intent field: {name}")

usize_fields = {
    "source_capacity",
    "source_committed_bytes",
    "source_uncommitted_bytes",
    "padded_bytes",
    "slot_capacity",
    "planned_backing_bytes",
    "planned_committed_bytes",
    "applied_backing_bytes",
    "applied_committed_bytes",
    "remaining_source_bytes",
}
for name in usize_fields:
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"release intent {name} must be exact usize storage: {field}")

for name in ("reason", "row_index", "release_candidate_token", "release_intent_token"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"release intent {name} must remain i64 storage: {field}")

record_decl = None
for decl in data.get("record_decls", []):
    if isinstance(decl, dict) and decl.get("name") == "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseIntentReportFields":
        record_decl = decl
        break
if record_decl is None:
    raise SystemExit("missing release intent ReportFields record details")

record_fields = {
    field.get("name"): field
    for field in record_decl.get("field_decls", [])
}
for name in usize_fields:
    field = record_fields.get(name)
    if field is None or field.get("declared_type") != "usize":
        raise SystemExit(f"release intent ReportFields {name} must be declared usize: {field}")
for name in ("reason", "row_index", "release_candidate_token", "release_intent_token"):
    field = record_fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"release intent ReportFields {name} must remain declared i64: {field}")

def walk(value):
    if isinstance(value, dict):
        yield value
        for child in value.values():
            yield from walk(child)
    elif isinstance(value, list):
        for child in value:
            yield from walk(child)

nodes = list(walk(data))
if any(
    node.get("op") == "newbox"
    and node.get("type") == "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseIntentReportFields"
    for node in nodes
):
    raise SystemExit("release intent ReportFields record materialized as NewBox")

print("[mimap284a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"

#!/usr/bin/env bash
set -euo pipefail

arena_backing_release_candidate_check_docs() {
  guard_expect_in_file "$TAG" 'Status: landed' "$CARD_276A" "MIMAP-276A allocation ledger must be landed before release candidate"
  guard_expect_in_file "$TAG" 'Status: landed' "$CARD_278A" "MIMAP-278A allocation ledger closeout must be landed before release candidate"
  guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-280A card must be landed"
  guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-280A release candidate design must be accepted"
  guard_expect_in_file "$TAG" 'release candidate' "$CARD" "MIMAP-280A card must call out release candidate"
  guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-280A guard"
  guard_expect_in_file "$TAG" 'tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml' "$PROOF_MANIFEST" "proof manifest must include arena-backing release lifecycle manifest"
  guard_expect_in_file "$TAG" 'id = "MIMAP-280A"' "$PROOF_MANIFEST_ROWS" "proof manifest row file must list MIMAP-280A"
  guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST_ROWS" "MIMAP-280A must use scalar-mir validation"
  guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST_ROWS" "MIMAP-280A EXE evidence must be deferred"
  guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_allocation_ledger_release_candidate_box' "$MODULE" "module must export release candidate owner"
  guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_allocation_ledger_release_candidate_box.hako' "$MEMORY_README" "memory README must name release candidate owner"
  guard_expect_in_file "$TAG" 'record HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReportFields' "$CANDIDATE_OWNER" "release candidate owner must use local ReportFields record payload"
  guard_expect_in_file "$TAG" 'local fields = HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReportFields' "$CANDIDATE_OWNER" "release candidate owner must construct report field records locally"
  guard_expect_in_file "$TAG" 'makeReleaseCandidateReport' "$CANDIDATE_OWNER" "release candidate owner must expose ReportFields helper-argument scalarization helper"
  guard_expect_in_file "$TAG" 'return me.makeReleaseCandidateReport' "$CANDIDATE_OWNER" "release candidate makeReport must delegate report copy through ReportFields helper"
  guard_expect_in_file "$TAG" 'source_capacity: usize' "$CANDIDATE_OWNER" "release candidate byte/capacity group must migrate source_capacity to usize"
  guard_expect_in_file "$TAG" 'applied_backing_bytes: usize' "$CANDIDATE_OWNER" "release candidate byte/capacity group must migrate applied_backing_bytes to usize"
  guard_expect_in_file "$TAG" 'remaining_source_bytes: usize' "$CANDIDATE_OWNER" "release candidate byte/capacity group must migrate remaining_source_bytes to usize"
  guard_expect_in_file "$TAG" 'row_index: i64' "$CANDIDATE_OWNER" "release candidate row_index sentinel must stay i64"
  guard_expect_in_file "$TAG" 'release_candidate_token: i64' "$CANDIDATE_OWNER" "release candidate token must stay i64"
  guard_expect_in_file "$TAG" 'recordReleaseCandidate' "$CANDIDATE_OWNER" "release candidate owner must expose record route"
  guard_expect_in_file "$TAG" 'duplicate_release_candidate_token_reject_count' "$CANDIDATE_OWNER" "release candidate owner must reject duplicate token"
  guard_expect_in_file "$TAG" 'check "mimap280a segment arena backing modeled allocation ledger release candidate"' "$APP" "proof must use labelled check block"
}

arena_backing_release_candidate_check_forbidden() {
  if rg -n 'lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
    "$APP" "$CANDIDATE_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
    echo "[$TAG] ERROR: MIMAP-280A must keep pointer/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
    cat /tmp/"$TAG".execution_leak >&2
    rm -f /tmp/"$TAG".execution_leak
    exit 1
  fi
  rm -f /tmp/"$TAG".execution_leak

  if rg -n 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-proof|ReleaseCandidate|releaseCandidate' \
    lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
    echo "[$TAG] ERROR: MIMAP-280A app/owner matcher leaked into .inc" >&2
    cat /tmp/"$TAG".inc_leak >&2
    rm -f /tmp/"$TAG".inc_leak
    exit 1
  fi
  rm -f /tmp/"$TAG".inc_leak
}

arena_backing_release_candidate_check_vm() {
  if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
    return 0
  fi

  tmp_dir="$(mktemp -d /tmp/hakorune_mimap280_release_candidate.XXXXXX)"
  trap 'rm -rf "$tmp_dir"' EXIT

  local vm_log="$tmp_dir/vm.log"
  if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
    exit 1
  fi

  rg -F -q 'hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-proof' "$vm_log"
  rg -F -q 'candidate=1,0,1,140,14,3,70014005005,1,90014005005,91014005005,92014005005,93014005005,94014005005,4096,4096,12288,8192' "$vm_log"
  rg -F -q 'rejects=1,2,3,4,5' "$vm_log"
  rg -F -q 'counts=6,1,5,1,1,1,1,1,5,94014005005' "$vm_log"
  rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$vm_log"
  rg -F -q 'check=1' "$vm_log"
  rg -F -q 'summary=ok' "$vm_log"
}

arena_backing_release_candidate_check_mir() {
  if ! pure_first_guard_level_allows_mir "$VALIDATION_LEVEL"; then
    return 0
  fi

  tmp_dir="$(mktemp -d /tmp/hakorune_mimap280_release_candidate.XXXXXX)"
  trap 'rm -rf "$tmp_dir"' EXIT

  local mir_json="$tmp_dir/mimap280.mir.json"
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
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerInventory.recordAllocationLedger/2",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateInventory.makeReleaseCandidateReport/1",
    "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateInventory.recordReleaseCandidate/2",
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
if "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReportFields" not in record_names:
    raise SystemExit("missing modeled release candidate ReportFields record declaration")

report = plans.get("HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReport")
if report is None:
    raise SystemExit("missing modeled release candidate report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
required_fields = (
    "accepted",
    "reason",
    "release_candidate_present",
    "modeled_release_candidate_present",
    "release_candidate_token",
    "ledger_token",
    "apply_token",
    "applied_backing_bytes",
    "applied_committed_bytes",
    "remaining_source_bytes",
    "would_add_backend_matcher",
)
for name in required_fields:
    if name not in fields:
        raise SystemExit(f"missing modeled release candidate field: {name}")

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
        raise SystemExit(f"release candidate {name} must be exact usize storage: {field}")

for name in ("reason", "row_index", "release_candidate_token"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"release candidate {name} must remain i64 storage: {field}")

record_decl = None
for decl in data.get("record_decls", []):
    if isinstance(decl, dict) and decl.get("name") == "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReportFields":
        record_decl = decl
        break
if record_decl is None:
    raise SystemExit("missing release candidate ReportFields record details")

record_fields = {
    field.get("name"): field
    for field in record_decl.get("field_decls", [])
}
for name in usize_fields:
    field = record_fields.get(name)
    if field is None or field.get("declared_type") != "usize":
        raise SystemExit(f"release candidate ReportFields {name} must be declared usize: {field}")
for name in ("reason", "row_index", "release_candidate_token"):
    field = record_fields.get(name)
    if field is None or field.get("declared_type") != "i64":
        raise SystemExit(f"release candidate ReportFields {name} must remain declared i64: {field}")

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
    and node.get("type") == "HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReportFields"
    for node in nodes
):
    raise SystemExit("release candidate ReportFields record materialized as NewBox")

print("[mimap280a-mir-json] ok")
PY
}

arena_backing_release_candidate_check_all() {
  arena_backing_release_candidate_check_docs
  arena_backing_release_candidate_check_forbidden
  arena_backing_release_candidate_check_vm
  arena_backing_release_candidate_check_mir
}

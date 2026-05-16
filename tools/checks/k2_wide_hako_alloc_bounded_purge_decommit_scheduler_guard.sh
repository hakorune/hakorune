#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-bounded-purge-decommit-scheduler"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-bounded-purge-decommit-scheduler-proof/main.hako"
APP_README="apps/hako-alloc-bounded-purge-decommit-scheduler-proof/README.md"
APP_TEST="apps/hako-alloc-bounded-purge-decommit-scheduler-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-259-M212-BOUNDED-PURGE-DECOMMIT-SCHEDULER-SMALL-PATH.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/purge_bounded_scheduler_box.hako"
LIFECYCLE="lang/src/hako_alloc/memory/page_lifecycle_invariant_box.hako"
CANDIDATE="lang/src/hako_alloc/memory/purge_candidate_policy_box.hako"
DUPLICATE_GUARD="lang/src/hako_alloc/memory/purge_state_aware_decommit_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_bounded_purge_decommit_scheduler_guard.sh"

echo "[$TAG] checking M212 bounded purge/decommit scheduler small path"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$PLAN" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$OWNER" \
  "$LIFECYCLE" \
  "$CANDIDATE" \
  "$DUPLICATE_GUARD" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M212 card must be complete"
guard_expect_in_file "$TAG" 'M212 status:' "$PLAN" "mimalloc plan must record M212 status"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M212 guard"
guard_expect_in_file "$TAG" 'id = "M212"' "$PROOF_MANIFEST" "proof app manifest must list M212"
guard_expect_in_file "$TAG" 'memory.purge_bounded_scheduler_box = "memory/purge_bounded_scheduler_box.hako"' "$MODULE" "hako_alloc module must export M212 owner"
guard_expect_in_file "$TAG" 'record HakoAllocBoundedPurgeDecommitSchedulerReportFields' "$OWNER" "MIMAP-041A report payload record must exist"
guard_expect_in_file "$TAG" 'box HakoAllocBoundedPurgeDecommitSchedulerReport' "$OWNER" "M212 report box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocBoundedPurgeDecommitScheduler' "$OWNER" "M212 scheduler box must exist"
guard_expect_in_file "$TAG" 'run\(heap, decommit_guard, max_scan_pages\)' "$OWNER" "M212 scheduler must expose bounded run"
guard_expect_in_file "$TAG" 'local fields = HakoAllocBoundedPurgeDecommitSchedulerReportFields' "$OWNER" "MIMAP-041A must construct local record report payload"
guard_expect_in_file "$TAG" 'result.status = fields.status' "$OWNER" "MIMAP-041A must materialize report box from record field reads"
guard_expect_in_file "$TAG" 'observeHeapPage' "$OWNER" "M212 must consume M207 observation"
guard_expect_in_file "$TAG" 'classifyLifecycleReport' "$OWNER" "M212 must consume M211 decisions"
guard_expect_in_file "$TAG" 'decommit_guard.attemptHeapPage' "$OWNER" "M212 must call M199 state-aware guard seam"
guard_expect_in_file "$TAG" 'scanned_pages' "$OWNER" "M212 must report bounded scan count"
guard_expect_in_file "$TAG" 'attempted' "$OWNER" "M212 must report one candidate attempt"
guard_expect_in_file "$TAG" 'purge_bounded_scheduler_box.hako` owns M212 bounded purge/decommit scheduler' "$MEMORY_README" "memory README must define M212 owner"

if rg -n 'HakoAllocPurgeHeapDecommitIntegration|HakoAllocBoundedDecommitPolicy|HakoAllocPageSourceDecommitAdapter|HakoAllocPageSourcePolicy|OsVmCoreBox|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(' \
  "$OWNER" >/tmp/"$TAG".direct_execution_leak 2>&1; then
  echo "[$TAG] ERROR: M212 scheduler must not call direct M197/M195/M196/page-source/OS release seams" >&2
  cat /tmp/"$TAG".direct_execution_leak >&2
  rm -f /tmp/"$TAG".direct_execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".direct_execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|provider_|install_hook|global_allocator|InlineRecord|ArrayStorage|PlanProbe|AutoUse|compiler' \
  "$OWNER" "$APP_README" >/tmp/"$TAG".forbidden_vocab 2>&1; then
  echo "[$TAG] ERROR: M212 scheduler/readme must stay options/provider/backend vocabulary free" >&2
  cat /tmp/"$TAG".forbidden_vocab >&2
  rm -f /tmp/"$TAG".forbidden_vocab
  exit 1
fi
rm -f /tmp/"$TAG".forbidden_vocab

if rg -n 'report[[:space:]]*\(status,[[:space:]]*stop_reason|me\.report[[:space:]]*\(' \
  "$OWNER" >/tmp/"$TAG".report16 2>&1; then
  echo "[$TAG] ERROR: MIMAP-041A must not keep the old 16-argument report helper/calls" >&2
  cat /tmp/"$TAG".report16 >&2
  rm -f /tmp/"$TAG".report16
  exit 1
fi
rm -f /tmp/"$TAG".report16

if rg -n 'hako-alloc-bounded-purge-decommit-scheduler-proof|HakoAllocBoundedPurgeDecommitScheduler|purge_bounded_scheduler' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M212 app/scheduler matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_bounded_purge_decommit_scheduler_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M212 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m212_hako_alloc_purge.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m212.mir.json"
exe_out="$tmp_dir/m212.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

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
    "HakoAllocBoundedPurgeDecommitScheduler.run/3",
    "HakoAllocPageLifecycleInvariantObserver.observeHeapPage/3",
    "HakoAllocPurgeCandidatePolicyInventory.classifyLifecycleReport/1",
    "HakoAllocPurgeStateAwareDecommitGuard.attemptHeapPage/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

if functions.get("HakoAllocBoundedPurgeDecommitScheduler.report/16") is not None:
    raise SystemExit("old report/16 helper must be removed")

record_decls = {
    decl.get("name"): decl
    for decl in data.get("record_decls", [])
}
payload_decl = record_decls.get("HakoAllocBoundedPurgeDecommitSchedulerReportFields")
if payload_decl is None:
    raise SystemExit("missing scheduler report payload record decl")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
for name in ("HakoAllocBoundedPurgeDecommitSchedulerReport", "HakoAllocBoundedPurgeDecommitScheduler"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

report_fields = {
    field.get("name"): field
    for field in plans["HakoAllocBoundedPurgeDecommitSchedulerReport"].get("fields", [])
}
required_fields = {
    "status",
    "stop_reason",
    "max_scan_pages",
    "scan_limit",
    "scanned_pages",
    "missing_pages",
    "rejected_pages",
    "candidate_pages",
    "attempted",
    "source_executed",
    "marker_marked",
    "selected_page_id",
    "selected_lifecycle_state",
    "selected_reason",
    "decommit_status",
    "mark_status",
}
missing_fields = sorted(name for name in required_fields if name not in report_fields)
if missing_fields:
    raise SystemExit(f"missing scheduler report fields: {missing_fields}")

for name in required_fields:
    field = report_fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad scheduler report field {name}: {field}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

run_fn = functions["HakoAllocBoundedPurgeDecommitScheduler.run/3"]
seen = set()
for callee in iter_calls(run_fn):
    seen.add((callee.get("box_name"), callee.get("name")))
    if (callee.get("box_name"), callee.get("name")) == ("HakoAllocBoundedPurgeDecommitScheduler", "report"):
        raise SystemExit("run must not call old report helper")
    if callee.get("name") in {"decommitPage", "commitPage", "reservePage", "unreserve", "releasePage"}:
        raise SystemExit(f"scheduler must not call direct page-source/OS method: {callee}")

required_calls = {
    ("HakoAllocPageLifecycleInvariantObserver", "observeHeapPage"),
    ("HakoAllocPurgeCandidatePolicyInventory", "classifyLifecycleReport"),
    ("HakoAllocPurgeStateAwareDecommitGuard", "attemptHeapPage"),
}
missing_calls = sorted(required_calls - seen)
if missing_calls:
    raise SystemExit(f"scheduler missing required seam calls: {missing_calls}")

print("[m212-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-bounded-purge-decommit-scheduler-proof' "$run_log"
rg -F -q 'zero=1,1,0,0,0,0' "$run_log"
rg -F -q 'active=1,2,1,1,0,0' "$run_log"
rg -F -q 'ready=0,0,1,1,1,1,1,0' "$run_log"
rg -F -q 'duplicate=1,2,1,1,0,1,1' "$run_log"
rg -F -q 'bound=1,2,1,1,0,0' "$run_log"
rg -F -q 'two=0,0,2,1,1,1,1' "$run_log"
rg -F -q 'over=2,3,1,1,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-modeled-consume-ledger-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

SSOT="docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-closeout-ssot.md"
ROUTE_SSOT="docs/development/current/main/design/hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger-ssot.md"
DIAGNOSTICS_SSOT="docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-diagnostics-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
CARD_157A="docs/development/current/main/phases/phase-293x/293x-679-MIMAP-157A-SEGMENT-MAP-ACCEPTED-READINESS-MODELED-CONSUME-LEDGER-ROUTE.md"
CARD_158A="docs/development/current/main/phases/phase-293x/293x-680-MIMAP-158A-SEGMENT-MAP-MODELED-CONSUME-LEDGER-DIAGNOSTICS.md"
CARD_159A="docs/development/current/main/phases/phase-293x/293x-681-MIMAP-159A-SEGMENT-MAP-MODELED-CONSUME-LEDGER-CLOSEOUT-PACK.md"
CARD_160A="docs/development/current/main/phases/phase-293x/293x-682-MIMAP-160A-POST-SEGMENT-MAP-MODELED-CONSUME-LEDGER-CLOSEOUT-ROW-SELECTION.md"
APP="apps/hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger-proof/test.sh"
OWNER="lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako"
GUARD_157A="tools/checks/k2_wide_hako_alloc_segment_map_accepted_readiness_modeled_consume_ledger_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_closeout_guard.sh"
RUN_PROOF="tools/checks/run_proof_app.sh"

echo "[$TAG] checking MIMAP-159A segment-map modeled consume ledger closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$ROUTE_SSOT" \
  "$DIAGNOSTICS_SSOT" \
  "$CADENCE" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$CARD_157A" \
  "$CARD_158A" \
  "$CARD_159A" \
  "$CARD_160A" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$OWNER" \
  "$GUARD_157A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT" \
  "$RUN_PROOF"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_157A" "$SELF_SCRIPT" "$IMPL_SCRIPT" "$RUN_PROOF"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_157A" "MIMAP-157A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_158A" "MIMAP-158A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_159A" "MIMAP-159A closeout card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_160A" "MIMAP-160A must be selected current after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-159A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$ROUTE_SSOT" "MIMAP-157A route SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$DIAGNOSTICS_SSOT" "MIMAP-158A diagnostics SSOT must stay accepted"
guard_expect_in_file "$TAG" "MIMAP-157A" "$SSOT" "closeout SSOT must include route row"
guard_expect_in_file "$TAG" "MIMAP-158A" "$SSOT" "closeout SSOT must include diagnostics row"
guard_expect_in_file "$TAG" "segment-map-consume-ledger" "$SSOT" "closeout SSOT must name pack"
guard_expect_in_file "$TAG" "--level L2" "$SSOT" "closeout SSOT must freeze L2 daily command"
guard_expect_in_file "$TAG" "representative L3 EXE evidence" "$SSOT" "closeout SSOT must require L3 evidence"
guard_expect_in_file "$TAG" "MIMAP-160A post-segment-map-modeled-consume-ledger-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "segment-map-consume-ledger" "$CADENCE" "cadence SSOT must name consume ledger pack"
guard_expect_in_file "$TAG" "MIMAP-157A" "$GRANULARITY" "granularity SSOT must describe MIMAP-157A"
guard_expect_in_file "$TAG" "MIMAP-158A" "$GRANULARITY" "granularity SSOT must describe MIMAP-158A"
guard_expect_in_file "$TAG" "MIMAP-159A" "$GRANULARITY" "granularity SSOT must describe MIMAP-159A"
guard_expect_in_file "$TAG" "MIMAP-160A" "$GRANULARITY" "granularity SSOT must describe MIMAP-160A"
guard_expect_in_file "$TAG" "MIMAP-159A segment-map modeled consume ledger closeout pack" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-160A post-segment-map-modeled-consume-ledger-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-160A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-157A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-157A"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-consume-ledger\"" "$PROOF_MANIFEST" "proof manifest must assign consume-ledger pack"
guard_expect_in_file "$TAG" "exe = \"deferred-to-closeout\"" "$PROOF_MANIFEST" "MIMAP-157A EXE evidence must stay deferred to closeout"
guard_expect_in_file "$TAG" "cmd_l2" "$PROOF_MANIFEST" "proof manifest must keep L2 command"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-map-modeled-consume-ledger-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-159A closeout row"
guard_expect_in_file "$TAG" "closeout_pack = \"segment-map-consume-ledger\"" "$GUARD_MANIFEST" "guard manifest must assign consume-ledger pack"
guard_expect_in_file "$TAG" "$GUARD_157A" "$INDEX" "check index must list MIMAP-157A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-159A closeout guard"

guard_expect_in_file "$TAG" "diagnosticBlocked" "$OWNER" "owner must retain blocked diagnostic"
guard_expect_in_file "$TAG" "diagnosticDuplicate" "$OWNER" "owner must retain duplicate diagnostic"
guard_expect_in_file "$TAG" "diagnosticStale" "$OWNER" "owner must retain stale diagnostic"
guard_expect_in_file "$TAG" "consumeAcceptedReadiness" "$OWNER" "owner must retain consume route"
guard_expect_in_file "$TAG" "diagnostics=" "$APP" "proof must print diagnostic summary"
guard_expect_in_file "$TAG" "counts=" "$APP" "proof must print closeout counters"

bash "$RUN_PROOF" --closeout-pack segment-map-consume-ledger --level L2 --dry-run >/tmp/"$TAG".proof_dry_run 2>&1 || {
  cat /tmp/"$TAG".proof_dry_run >&2
  rm -f /tmp/"$TAG".proof_dry_run
  guard_fail "$TAG" "segment-map consume-ledger L2 dry-run selection failed"
}
guard_expect_in_file "$TAG" "MIMAP-157A" /tmp/"$TAG".proof_dry_run "L2 dry-run must include MIMAP-157A"
rm -f /tmp/"$TAG".proof_dry_run

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "consume-ledger closeout must keep raw pointer/concurrency/segment-map/atomics/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "consume-ledger closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger-proof|HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedger|segment_map_accepted_readiness_modeled_consume_ledger' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "consume-ledger app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap159a_segment_map_consume_ledger_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap159a.mir.json"
exe_out="$tmp_dir/mimap159a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger-proof' "$vm_log"
rg -F -q 'consumed=1,0,0,0,0,0,-1,70,7,2,3,5,3,2,70007002,1,1' "$vm_log"
rg -F -q 'diagnostics=1,2,3,4,4,5' "$vm_log"
rg -F -q 'counts=5,1,4,2,1,1,1,1,1,1,1,70007002,3' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

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
    "HakoAllocSegmentAllocationModeledConsume.consumeReadiness/8",
    "HakoAllocSegmentAllocationModeledLedger.recordModeledConsume/12",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedgerReport")
if report is None:
    raise SystemExit("missing consume ledger closeout report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "lookup_reason",
    "membership_reason",
    "readiness_reason",
    "consume_reason",
    "ledger_reason",
    "diagnostic_kind",
    "modeled_allocation_token",
    "ledger_count_after",
    "ledger_live_count_after",
):
    field = fields.get(name)
    if field is None:
        raise SystemExit(f"missing closeout report field: {name}")
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad closeout report field {name}: {field}")

print("[mimap159a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger-proof' "$run_log"
rg -F -q 'consumed=1,0,0,0,0,0,-1,70,7,2,3,5,3,2,70007002,1,1' "$run_log"
rg -F -q 'diagnostics=1,2,3,4,4,5' "$run_log"
rg -F -q 'counts=5,1,4,2,1,1,1,1,1,1,1,70007002,3' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

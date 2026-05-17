#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-lifecycle-scalar-state"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-segment-lifecycle-scalar-state-proof/main.hako"
APP_README="apps/hako-alloc-segment-lifecycle-scalar-state-proof/README.md"
APP_TEST="apps/hako-alloc-segment-lifecycle-scalar-state-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-569-MIMAP-082A-SEGMENT-LIFECYCLE-SCALAR-STATE-CONTRACT.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-lifecycle-scalar-state-ssot.md"
LIFECYCLE_BLUEPRINT="docs/development/current/main/design/mimalloc-lifecycle-rewrite-blueprint-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
OWNER="lang/src/hako_alloc/memory/segment_lifecycle_scalar_state_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_lifecycle_scalar_state_guard.sh"

printf '[%s] checking MIMAP-082A segment lifecycle scalar state contract\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$LIFECYCLE_BLUEPRINT" \
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

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-082A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-082A design must be accepted"
guard_expect_in_file "$TAG" 'Segment Lifecycle' "$LIFECYCLE_BLUEPRINT" "lifecycle blueprint must name segment lifecycle"
guard_expect_in_file "$TAG" 'Reserved' "$DESIGN" "MIMAP-082A design must name Reserved state"
guard_expect_in_file "$TAG" 'PurgeScheduled' "$DESIGN" "MIMAP-082A design must name PurgeScheduled state"
guard_expect_in_file "$TAG" 'Abandoned -> Reclaimed' "$DESIGN" "MIMAP-082A design must name reclaim transition"
guard_expect_in_file "$TAG" 'MIMAP-082A granularity' "$PLAN" "granularity SSOT must describe MIMAP-082A"
guard_expect_in_file "$TAG" 'MIMAP-082A segment lifecycle scalar state contract' "$JOINT" "joint order must name MIMAP-082A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-082A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-082A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-082A"
guard_expect_in_file "$TAG" 'memory.segment_lifecycle_scalar_state_box = "memory/segment_lifecycle_scalar_state_box.hako"' "$MODULE" "hako module must export MIMAP-082A owner"
guard_expect_in_file "$TAG" 'segment_lifecycle_scalar_state_box.hako` owns MIMAP-082A' "$MEMORY_README" "memory README must define MIMAP-082A owner"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentLifecycleScalarStateReport' "$OWNER" "MIMAP-082A report box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocSegmentLifecycleScalarState' "$OWNER" "MIMAP-082A owner must exist"
guard_expect_in_file "$TAG" 'classifyTransition' "$OWNER" "MIMAP-082A owner must expose classifyTransition"
guard_expect_in_file "$TAG" 'transitionId' "$OWNER" "MIMAP-082A owner must centralize transition ids"
guard_expect_in_file "$TAG" 'HakoAllocSegmentLifecycleScalarState' "$APP" "MIMAP-082A proof must construct lifecycle owner"
guard_expect_in_file "$TAG" 'check "mimap082a segment lifecycle scalar state contract"' "$APP" "MIMAP-082A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-082A must not add source concurrency, atomics, page-source/OS release seams, or page release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-082A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-lifecycle-scalar-state-proof|HakoAllocSegmentLifecycleScalarState|segment_lifecycle_scalar_state' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-082A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_segment_lifecycle_scalar_state_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-082A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap082a_segment_lifecycle.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap082a.mir.json"
exe_out="$tmp_dir/mimap082a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-segment-lifecycle-scalar-state-proof' "$vm_log"
rg -F -q 'transitions=10,11,12,13,14,15,16,17' "$vm_log"
rg -F -q 'rejects=1,2,3,4,5,6,7,8' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'counts=16,8,8,1,1,1,1,1,1,1,1,34,8,0' "$vm_log"
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
    "HakoAllocSegmentLifecycleScalarState.classifyTransition/8",
    "HakoAllocSegmentLifecycleScalarState.transitionId/2",
    "HakoAllocSegmentLifecycleScalarState.rejectUnsupportedRequirement/9",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
if plans.get("HakoAllocSegmentLifecycleScalarStateReport") is None:
    raise SystemExit("missing typed object plan: HakoAllocSegmentLifecycleScalarStateReport")
if plans.get("HakoAllocSegmentLifecycleScalarState") is None:
    raise SystemExit("missing typed object plan: HakoAllocSegmentLifecycleScalarState")

fields = {field.get("name"): field for field in plans["HakoAllocSegmentLifecycleScalarStateReport"].get("fields", [])}
required_fields = {
    "accepted",
    "reason",
    "segment_id",
    "from_state",
    "to_state",
    "transition_id",
    "used_slices",
    "abandoned_slices",
    "committed_slices",
    "purge_scheduled_slices",
    "unsupported_requirement",
    "lifecycle_contract_present",
    "would_use_raw_pointer",
    "would_execute_atomic_bitmap",
    "would_call_osvm",
    "would_run_thread",
    "would_activate_provider",
    "would_replace_process_allocator",
    "would_add_backend_matcher",
}
missing_fields = sorted(name for name in required_fields if name not in fields)
if missing_fields:
    raise SystemExit(f"missing report fields: {missing_fields}")

for name in required_fields:
    field = fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad report field {name}: {field}")

print("[mimap082a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-lifecycle-scalar-state-proof' "$run_log"
rg -F -q 'transitions=10,11,12,13,14,15,16,17' "$run_log"
rg -F -q 'rejects=1,2,3,4,5,6,7,8' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0' "$run_log"
rg -F -q 'counts=16,8,8,1,1,1,1,1,1,1,1,34,8,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"


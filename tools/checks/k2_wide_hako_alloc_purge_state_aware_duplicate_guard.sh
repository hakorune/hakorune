#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-purge-state-aware-duplicate"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-purge-state-aware-duplicate-guard-proof/main.hako"
APP_README="apps/hako-alloc-purge-state-aware-duplicate-guard-proof/README.md"
APP_TEST="apps/hako-alloc-purge-state-aware-duplicate-guard-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-239-M199-PURGE-STATE-AWARE-DUPLICATE-GUARD.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
GUARD_OWNER="lang/src/hako_alloc/memory/purge_state_aware_decommit_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_purge_state_aware_duplicate_guard.sh"

echo "[$TAG] checking M199 purge state-aware duplicate guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$PLAN" \
  "$PHASE_README" \
  "$TASKBOARD" \
  "$INDEX" \
  "$GUARD_OWNER" \
  "$MODULE" \
  "$MEMORY_README" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M199 card must be complete"
guard_expect_in_file "$TAG" 'M199 status:' "$PLAN" "mimalloc plan must record M199 status"
guard_expect_in_file "$TAG" '`293x-239`' "$PHASE_README" "phase README must list M199 row"
guard_expect_in_file "$TAG" '\[x\] `293x-239`' "$TASKBOARD" "taskboard must mark M199 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M199 guard"

guard_expect_in_file "$TAG" 'memory.purge_state_aware_decommit_box = "memory/purge_state_aware_decommit_box.hako"' "$MODULE" "hako_alloc module must export state-aware guard"
guard_expect_in_file "$TAG" 'box HakoAllocPurgeStateAwareDecommitGuard' "$GUARD_OWNER" "state-aware guard box must exist"
guard_expect_in_file "$TAG" 'isMarked' "$GUARD_OWNER" "state-aware guard must consult marker first"
guard_expect_in_file "$TAG" 'attemptHeapPage' "$GUARD_OWNER" "state-aware guard entry must exist"
guard_expect_in_file "$TAG" 'owns M199 purge state-aware duplicate' "$MEMORY_README" "memory README must define M199 owner"

if rg -n 'HakoAllocPageSourcePolicy\.|OsVmCoreBox\.|decommitPage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|heap\.decommitPage' \
  "$GUARD_OWNER" >/tmp/"$TAG".direct_leak 2>&1; then
  echo "[$TAG] ERROR: M199 guard must use M197/M198 owners, not direct page-source calls" >&2
  cat /tmp/"$TAG".direct_leak >&2
  rm -f /tmp/"$TAG".direct_leak
  exit 1
fi
rm -f /tmp/"$TAG".direct_leak

if rg -n 'hako-alloc-purge-state-aware-duplicate-guard-proof|HakoAllocPurgeStateAwareDecommitGuard' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M199 app/guard matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_purge_state_aware_duplicate_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M199 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m199_hako_alloc_duplicate.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m199.mir.json"
exe_out="$tmp_dir/m199.exe"
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
    "HakoAllocPurgeStateAwareDecommitGuard.attemptHeapPage/2",
    "HakoAllocPurgeHeapDecommitIntegration.attemptHeapPage/2",
    "HakoAllocPurgeDecommitStateMarker.markIfDecommitted/2",
    "HakoAllocPurgeDecommitStateMarker.isMarked/1",
    "HakoAllocPageSourceDecommitAdapter.decommitPage/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
guard = plans.get("HakoAllocPurgeStateAwareDecommitGuard")
if guard is None:
    raise SystemExit("missing typed object plan: HakoAllocPurgeStateAwareDecommitGuard")
fields = {
    field.get("name"): field
    for field in guard.get("fields", [])
}
for name, declared in (
    ("integration", "HakoAllocPurgeHeapDecommitIntegration"),
    ("marker", "HakoAllocPurgeDecommitStateMarker"),
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != declared or field.get("storage") != "handle":
        raise SystemExit(f"bad guard field {name}: {field}")
for name in ("attempt_count", "success_count", "blocked_count", "duplicate_block_count"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad guard counter field {name}: {field}")

print("[m199-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-purge-state-aware-duplicate-guard-proof' "$run_log"
rg -F -q 'live=2,1,0,2,3' "$run_log"
rg -F -q 'release=1' "$run_log"
rg -F -q 'first=0,0,1,1,0,0' "$run_log"
rg -F -q 'duplicate=1,1,1,0' "$run_log"
rg -F -q 'guard=3,1,2,1,1' "$run_log"
rg -F -q 'integration=2,1,1' "$run_log"
rg -F -q 'adapter=1,1,16,0' "$run_log"
rg -F -q 'marker=2,1,1,0,1' "$run_log"
rg -F -q 'marked=1,1' "$run_log"
rg -F -q 'heap=0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

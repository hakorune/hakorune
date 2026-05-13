#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-purge-decommit-state-marker"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-purge-decommit-state-marker-proof/main.hako"
APP_README="apps/hako-alloc-purge-decommit-state-marker-proof/README.md"
APP_TEST="apps/hako-alloc-purge-decommit-state-marker-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-238-M198-PURGE-DECOMMIT-STATE-MARKER.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
MARKER="lang/src/hako_alloc/memory/purge_decommit_state_marker_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_purge_decommit_state_marker_guard.sh"

echo "[$TAG] checking M198 purge decommit state marker"

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
  "$MARKER" \
  "$MODULE" \
  "$MEMORY_README" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M198 card must be complete"
guard_expect_in_file "$TAG" 'M198 status:' "$PLAN" "mimalloc plan must record M198 status"
guard_expect_in_file "$TAG" '`293x-238`' "$PHASE_README" "phase README must list M198 row"
guard_expect_in_file "$TAG" '\[x\] `293x-238`' "$TASKBOARD" "taskboard must mark M198 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M198 guard"

guard_expect_in_file "$TAG" 'memory.purge_decommit_state_marker_box = "memory/purge_decommit_state_marker_box.hako"' "$MODULE" "hako_alloc module must export decommit state marker"
guard_expect_in_file "$TAG" 'box HakoAllocPurgeDecommitStateMarker' "$MARKER" "state marker box must exist"
guard_expect_in_file "$TAG" 'markIfDecommitted' "$MARKER" "state marker entry must exist"
guard_expect_in_file "$TAG" 'isMarked' "$MARKER" "state marker observer must exist"
guard_expect_in_file "$TAG" 'purge_decommit_state_marker_box.hako` owns M198 purge decommit state marker' "$MEMORY_README" "memory README must define M198 owner"

if rg -n 'HakoAllocPageSourcePolicy\.|OsVmCoreBox\.|decommitPage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|heap\.|page\.' \
  "$MARKER" >/tmp/"$TAG".direct_leak 2>&1; then
  echo "[$TAG] ERROR: M198 marker must not call page-source APIs or mutate heap/page state" >&2
  cat /tmp/"$TAG".direct_leak >&2
  rm -f /tmp/"$TAG".direct_leak
  exit 1
fi
rm -f /tmp/"$TAG".direct_leak

if rg -n 'hako-alloc-purge-decommit-state-marker-proof|HakoAllocPurgeDecommitStateMarker' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M198 app/marker matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_purge_decommit_state_marker_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M198 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m198_hako_alloc_marker.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m198.mir.json"
exe_out="$tmp_dir/m198.exe"
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
    "HakoAllocPurgeDecommitStateMarker.markIfDecommitted/2",
    "HakoAllocPurgeDecommitStateMarker.isMarked/1",
    "HakoAllocPurgeHeapDecommitIntegration.attemptHeapPage/2",
    "HakoAllocBoundedDecommitPolicy.attemptDecommit/4",
    "HakoAllocPageSourceDecommitAdapter.decommitPage/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
marker = plans.get("HakoAllocPurgeDecommitStateMarker")
if marker is None:
    raise SystemExit("missing typed object plan: HakoAllocPurgeDecommitStateMarker")
fields = {
    field.get("name"): field
    for field in marker.get("fields", [])
}
page_ids = fields.get("marked_page_ids")
if page_ids is None or page_ids.get("declared_type") != "ArrayBox" or page_ids.get("storage") != "handle":
    raise SystemExit(f"bad marked_page_ids field: {page_ids}")
for name in ("attempt_count", "marked_count", "reject_count", "duplicate_count"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad marker counter field {name}: {field}")

print("[m198-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-purge-decommit-state-marker-proof' "$run_log"
rg -F -q 'live=2,1,0' "$run_log"
rg -F -q 'live_mark=3,0,1' "$run_log"
rg -F -q 'release=1' "$run_log"
rg -F -q 'decommit=0,1,1,0' "$run_log"
rg -F -q 'mark=0,1,0,0' "$run_log"
rg -F -q 'duplicate=5,0,1,1' "$run_log"
rg -F -q 'marker=3,1,2,1,1,0,0' "$run_log"
rg -F -q 'marked=1,1' "$run_log"
rg -F -q 'integration=2,1,1' "$run_log"
rg -F -q 'adapter=1,1,16,0' "$run_log"
rg -F -q 'release_fields=0,0' "$run_log"
rg -F -q 'heap=0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

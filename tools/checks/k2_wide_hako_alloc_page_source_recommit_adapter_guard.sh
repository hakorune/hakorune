#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-page-source-recommit-adapter"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-page-source-recommit-adapter-proof/main.hako"
APP_README="apps/hako-alloc-page-source-recommit-adapter-proof/README.md"
APP_TEST="apps/hako-alloc-page-source-recommit-adapter-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-247-M203-PAGE-SOURCE-RECOMMIT-ADAPTER.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
OWNER="lang/src/hako_alloc/memory/purge_page_source_recommit_adapter_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_page_source_recommit_adapter_guard.sh"

echo "[$TAG] checking M203 page-source recommit adapter"

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
  "$OWNER" \
  "$MODULE" \
  "$MEMORY_README" \
  "$PROOF_MANIFEST" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M203 card must be complete"
guard_expect_in_file "$TAG" 'M203 status:' "$PLAN" "mimalloc plan must record M203 status"
guard_expect_in_file "$TAG" '`293x-247`' "$PHASE_README" "phase README must list M203 row"
guard_expect_in_file "$TAG" '\[x\] `293x-247`' "$TASKBOARD" "taskboard must mark M203 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M203 guard"
guard_expect_in_file "$TAG" 'id = "M203"' "$PROOF_MANIFEST" "proof app manifest must list M203"

guard_expect_in_file "$TAG" 'memory.purge_page_source_recommit_adapter_box = "memory/purge_page_source_recommit_adapter_box.hako"' "$MODULE" "hako_alloc module must export recommit adapter"
guard_expect_in_file "$TAG" 'box HakoAllocPageSourceRecommitAdapter' "$OWNER" "recommit adapter box must exist"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy.commitPage' "$OWNER" "M203 adapter must delegate to commitPage"
guard_expect_in_file "$TAG" 'M203 page-source recommit' "$MEMORY_README" "memory README must define M203 owner"

if rg -n 'reservePage[[:space:]]*\(|decommitPage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|heap\.|addBackedPage[[:space:]]*\(|addFreshPage[[:space:]]*\(' \
  "$OWNER" >/tmp/"$TAG".direct_leak 2>&1; then
  echo "[$TAG] ERROR: M203 recommit adapter must expose only commitPage and must not mutate heap/page state" >&2
  cat /tmp/"$TAG".direct_leak >&2
  rm -f /tmp/"$TAG".direct_leak
  exit 1
fi
rm -f /tmp/"$TAG".direct_leak

if rg -n 'marked_page_ids\.|\.push\(|marker\.marked_page_ids|clearMarked|clearMarker|remove' "$OWNER" >/tmp/"$TAG".marker_mutation 2>&1; then
  echo "[$TAG] ERROR: M203 recommit adapter must not mutate or clear decommit marker state" >&2
  cat /tmp/"$TAG".marker_mutation >&2
  rm -f /tmp/"$TAG".marker_mutation
  exit 1
fi
rm -f /tmp/"$TAG".marker_mutation

if rg -n 'hako-alloc-page-source-recommit-adapter-proof|HakoAllocPageSourceRecommitAdapter' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M203 app/adapter matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_page_source_recommit_adapter_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M203 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m203_hako_alloc_recommit.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m203.mir.json"
exe_out="$tmp_dir/m203.exe"
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
    "HakoAllocPageSourceRecommitAdapter.commitPage/2",
    "HakoAllocBoundedRecommitPolicy.attemptRecommit/4",
    "HakoAllocDecommittedPageReusePrecondition.classifyHeapPage/3",
    "HakoAllocPurgeStateAwareDecommitGuard.attemptHeapPage/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
adapter = plans.get("HakoAllocPageSourceRecommitAdapter")
if adapter is None:
    raise SystemExit("missing typed object plan: HakoAllocPageSourceRecommitAdapter")
fields = {
    field.get("name"): field
    for field in adapter.get("fields", [])
}
for name in ("call_count", "success_count", "reject_count", "last_base", "last_bytes", "last_rc"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad recommit adapter field {name}: {field}")

print("[m203-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-page-source-recommit-adapter-proof' "$run_log"
rg -F -q 'before=0,1,0,16' "$run_log"
rg -F -q 'release=1' "$run_log"
rg -F -q 'decommit=0,1,1' "$run_log"
rg -F -q 'decision=1,1,1,16' "$run_log"
rg -F -q 'recommit=0,0,1,1,0,0,0,0' "$run_log"
rg -F -q 'adapter=1,1,0,0,16' "$run_log"
rg -F -q 'policy=1,0,1,1,0' "$run_log"
rg -F -q 'marker=1,1,0' "$run_log"
rg -F -q 'after=1,1,1' "$run_log"
rg -F -q 'guard=1,1,1' "$run_log"
rg -F -q 'heap=0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

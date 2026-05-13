#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-reuse-proof-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-reuse-proof-closeout-proof/main.hako"
APP_README="apps/hako-alloc-reuse-proof-closeout-proof/README.md"
APP_TEST="apps/hako-alloc-reuse-proof-closeout-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-250-M206-REUSE-PROOF-CLOSEOUT.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
RECOMMIT_OWNER="lang/src/hako_alloc/memory/purge_recommit_heap_integration_box.hako"
DECOMMIT_GUARD="lang/src/hako_alloc/memory/purge_state_aware_decommit_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_reuse_proof_closeout_guard.sh"

echo "[$TAG] checking M206 reuse proof closeout"

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
  "$PROOF_MANIFEST" \
  "$RECOMMIT_OWNER" \
  "$DECOMMIT_GUARD" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M206 card must be complete"
guard_expect_in_file "$TAG" 'M206 status:' "$PLAN" "mimalloc plan must record M206 status"
guard_expect_in_file "$TAG" '`293x-250`' "$PHASE_README" "phase README must list M206 row"
guard_expect_in_file "$TAG" '\[x\] `293x-250`' "$TASKBOARD" "taskboard must mark M206 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M206 guard"
guard_expect_in_file "$TAG" 'id = "M206"' "$PROOF_MANIFEST" "proof app manifest must list M206"
guard_expect_in_file "$TAG" 'HakoAllocPurgeStateAwareDecommitGuard' "$APP" "M206 proof must compose M199 duplicate guard"
guard_expect_in_file "$TAG" 'HakoAllocRecommitHeapIntegration' "$APP" "M206 proof must compose M205 recommit integration"

if rg -n 'reuse_proof_closeout|ReuseProofCloseout|hako-alloc-reuse-proof-closeout-proof' \
  lang/src/hako_alloc/memory lang/c-abi/shims >/tmp/"$TAG".leak 2>&1; then
  echo "[$TAG] ERROR: M206 closeout proof must not add allocator owners or .inc matchers" >&2
  cat /tmp/"$TAG".leak >&2
  rm -f /tmp/"$TAG".leak
  exit 1
fi
rm -f /tmp/"$TAG".leak

if rg -n 'k2_wide_hako_alloc_reuse_proof_closeout_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M206 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m206_hako_alloc_reuse.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m206.mir.json"
exe_out="$tmp_dir/m206.exe"
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
    "HakoAllocRecommitHeapIntegration.attemptHeapPage/3",
    "HakoAllocPageModel.reactivate/0",
    "HakoAllocPageQueue.directPageId/0",
    "HakoAllocPageModel.acquire/1",
    "HakoAllocPageModel.releaseLocal/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

print("[m206-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-reuse-proof-closeout-proof' "$run_log"
rg -F -q 'first=0,0,1,0,1,1' "$run_log"
rg -F -q 'blocked=1,1,1,0,-1' "$run_log"
rg -F -q 'recommit0=0,1,1,1,1' "$run_log"
rg -F -q 'reuse1=0,0,1,0,1,1' "$run_log"
rg -F -q 'recommit1=0,1,1,1,1' "$run_log"
rg -F -q 'reuse2=0,0' "$run_log"
rg -F -q 'marker=2,2,0' "$run_log"
rg -F -q 'decommit_guard=3,2,1,1,2' "$run_log"
rg -F -q 'recommit_integration=2,2,0,2,2' "$run_log"
rg -F -q 'page=1,0,0,0,2,2,1' "$run_log"
rg -F -q 'heap=0,0,1,1,1,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

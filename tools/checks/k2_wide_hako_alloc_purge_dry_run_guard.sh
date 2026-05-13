#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-purge-dry-run"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-purge-dry-run-proof/main.hako"
APP_README="apps/hako-alloc-purge-dry-run-proof/README.md"
APP_TEST="apps/hako-alloc-purge-dry-run-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-233-M193-PURGE-DECOMMIT-DRY-RUN-OBSERVER.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
DRY_RUN="lang/src/hako_alloc/memory/purge_dry_run_box.hako"
POLICY="lang/src/hako_alloc/memory/purge_policy_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_purge_dry_run_guard.sh"

echo "[$TAG] checking M193 purge/decommit dry-run observer"

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
  "$DRY_RUN" \
  "$POLICY" \
  "$MODULE" \
  "$MEMORY_README" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M193 card must be complete"
guard_expect_in_file "$TAG" 'M193 status:' "$PLAN" "mimalloc plan must record M193 status"
guard_expect_in_file "$TAG" '`293x-233`' "$PHASE_README" "phase README must list M193 row"
guard_expect_in_file "$TAG" '\[x\] `293x-233`' "$TASKBOARD" "taskboard must mark M193 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M193 guard"

guard_expect_in_file "$TAG" 'memory.purge_dry_run_box = "memory/purge_dry_run_box.hako"' "$MODULE" "hako_alloc module must export dry-run owner"
guard_expect_in_file "$TAG" 'box HakoAllocPurgeDryRunObserver' "$DRY_RUN" "dry-run observer box must exist"
guard_expect_in_file "$TAG" 'observeHeapPage' "$DRY_RUN" "dry-run observer must expose observeHeapPage"
guard_expect_in_file "$TAG" 'HakoAllocPurgePolicyInventory' "$DRY_RUN" "dry-run observer must delegate to M192 policy"
guard_expect_in_file "$TAG" 'purge_dry_run_box.hako` owns M193 purge/decommit dry-run observation' "$MEMORY_README" "memory README must define dry-run owner"

if rg -n 'using selfhost\.hako_alloc\.memory\.page_source_policy_box|HakoAllocPageSourcePolicy\.|OsVmCoreBox\.|decommitPage[[:space:]]*\(|pageSourceDecommit|reservePage[[:space:]]*\(|commitPage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(' \
  "$DRY_RUN" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: M193 dry-run observer must not call page source or OS release surfaces" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-purge-dry-run-proof|HakoAllocPurgeDryRunObserver|observeHeapPage' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M193 app/dry-run matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_purge_dry_run_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M193 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m193_hako_alloc_purge.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m193.mir.json"
exe_out="$tmp_dir/m193.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-purge-dry-run-proof' "$vm_log"
rg -F -q 'live=0,2,0,0' "$vm_log"
rg -F -q 'release=1' "$vm_log"
rg -F -q 'ready=1,0,1,1,1,16' "$vm_log"
rg -F -q 'missing=0,1,0' "$vm_log"
rg -F -q 'would=0,0,0' "$vm_log"
rg -F -q 'observer=2,1,2,1,99,0' "$vm_log"
rg -F -q 'policy=1,2,1,1,0,1' "$vm_log"
rg -F -q 'page=0,1,0' "$vm_log"
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
    "HakoAllocPurgeDryRunObserver.observeHeapPage/2",
    "HakoAllocPurgePolicyInventory.classifyLocalPage/3",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
observer = plans.get("HakoAllocPurgeDryRunObserver")
if observer is None:
    raise SystemExit("missing typed object plan: HakoAllocPurgeDryRunObserver")
fields = {
    field.get("name"): field
    for field in observer.get("fields", [])
}
policy = fields.get("policy")
if (
    policy is None
    or policy.get("declared_type") != "HakoAllocPurgePolicyInventory"
    or policy.get("storage") != "handle"
):
    raise SystemExit(f"dry-run policy field must be HakoAllocPurgePolicyInventory handle: {policy}")

print("[m193-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-purge-dry-run-proof' "$run_log"
rg -F -q 'live=0,2,0,0' "$run_log"
rg -F -q 'release=1' "$run_log"
rg -F -q 'ready=1,0,1,1,1,16' "$run_log"
rg -F -q 'missing=0,1,0' "$run_log"
rg -F -q 'would=0,0,0' "$run_log"
rg -F -q 'observer=2,1,2,1,99,0' "$run_log"
rg -F -q 'policy=1,2,1,1,0,1' "$run_log"
rg -F -q 'page=0,1,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

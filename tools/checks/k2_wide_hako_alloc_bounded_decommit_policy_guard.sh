#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-bounded-decommit-policy"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-bounded-decommit-policy-proof/main.hako"
APP_README="apps/hako-alloc-bounded-decommit-policy-proof/README.md"
APP_TEST="apps/hako-alloc-bounded-decommit-policy-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-235-M195-BOUNDED-DECOMMIT-EXECUTION-POLICY.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
BOUNDED="lang/src/hako_alloc/memory/purge_bounded_decommit_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_bounded_decommit_policy_guard.sh"

echo "[$TAG] checking M195 bounded decommit execution policy"

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
  "$BOUNDED" \
  "$MODULE" \
  "$MEMORY_README" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M195 card must be complete"
guard_expect_in_file "$TAG" 'M195 status:' "$PLAN" "mimalloc plan must record M195 status"
guard_expect_in_file "$TAG" '`293x-235`' "$PHASE_README" "phase README must list M195 row"
guard_expect_in_file "$TAG" '\[x\] `293x-235`' "$TASKBOARD" "taskboard must mark M195 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M195 guard"

guard_expect_in_file "$TAG" 'memory.purge_bounded_decommit_box = "memory/purge_bounded_decommit_box.hako"' "$MODULE" "hako_alloc module must export bounded decommit owner"
guard_expect_in_file "$TAG" 'box HakoAllocBoundedDecommitReport' "$BOUNDED" "bounded decommit report must exist"
guard_expect_in_file "$TAG" 'box HakoAllocBoundedDecommitPolicy' "$BOUNDED" "bounded decommit policy must exist"
guard_expect_in_file "$TAG" 'attemptDecommit' "$BOUNDED" "bounded decommit policy must expose attemptDecommit"
guard_expect_in_file "$TAG" 'source.decommitPage\(base, bytes\)' "$BOUNDED" "bounded decommit must execute through caller-provided source"
guard_expect_in_file "$TAG" 'unreserve_executed: i64 = 0' "$BOUNDED" "M195 must keep unreserve inactive"
guard_expect_in_file "$TAG" 'os_release_executed: i64 = 0' "$BOUNDED" "M195 must keep OS release inactive"
guard_expect_in_file "$TAG" 'purge_bounded_decommit_box.hako` owns M195 bounded decommit execution' "$MEMORY_README" "memory README must define bounded decommit owner"

if rg -n 'using selfhost\.hako_alloc\.memory\.page_source_policy_box|HakoAllocPageSourcePolicy\.|OsVmCoreBox\.|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(' \
  "$BOUNDED" >/tmp/"$TAG".direct_osvm_leak 2>&1; then
  echo "[$TAG] ERROR: M195 bounded policy must not directly call page source or OS release surfaces" >&2
  cat /tmp/"$TAG".direct_osvm_leak >&2
  rm -f /tmp/"$TAG".direct_osvm_leak
  exit 1
fi
rm -f /tmp/"$TAG".direct_osvm_leak

if rg -n 'hako-alloc-bounded-decommit-policy-proof|HakoAllocBoundedDecommitPolicy|HakoAllocBoundedDecommitReport|HakoAllocFakeDecommitSource' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M195 app/policy matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_bounded_decommit_policy_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M195 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m195_hako_alloc_decommit.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m195.mir.json"
exe_out="$tmp_dir/m195.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,180p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-bounded-decommit-policy-proof' "$vm_log"
rg -F -q 'missing=1,1,0' "$vm_log"
rg -F -q 'ineligible=2,2,1' "$vm_log"
rg -F -q 'bad_base=3,1' "$vm_log"
rg -F -q 'too_large=5,8192,4096' "$vm_log"
rg -F -q 'source_fail=6,1,0,7' "$vm_log"
rg -F -q 'success=0,0,1,1,0' "$vm_log"
rg -F -q 'source=2,1000,16' "$vm_log"
rg -F -q 'policy=6,5,2,1,1' "$vm_log"
rg -F -q 'release=0,0' "$vm_log"
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
    "HakoAllocBoundedDecommitPolicy.attemptDecommit/4",
    "HakoAllocBoundedDecommitPolicy.report/6",
    "HakoAllocFakeDecommitSource.decommitPage/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
for name in ("HakoAllocBoundedDecommitReport", "HakoAllocBoundedDecommitPolicy"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

report_fields = {
    field.get("name"): field
    for field in plans["HakoAllocBoundedDecommitReport"].get("fields", [])
}
for name in ("decommit_attempted", "decommit_executed", "unreserve_executed", "os_release_executed"):
    if name not in report_fields:
        raise SystemExit(f"missing bounded decommit report field: {name}")

print("[m195-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-bounded-decommit-policy-proof' "$run_log"
rg -F -q 'missing=1,1,0' "$run_log"
rg -F -q 'ineligible=2,2,1' "$run_log"
rg -F -q 'bad_base=3,1' "$run_log"
rg -F -q 'too_large=5,8192,4096' "$run_log"
rg -F -q 'source_fail=6,1,0,7' "$run_log"
rg -F -q 'success=0,0,1,1,0' "$run_log"
rg -F -q 'source=2,1000,16' "$run_log"
rg -F -q 'policy=6,5,2,1,1' "$run_log"
rg -F -q 'release=0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-reclaim-post-drain-owner-transfer"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-reclaim-post-drain-owner-transfer-proof/main.hako"
APP_README="apps/hako-alloc-reclaim-post-drain-owner-transfer-proof/README.md"
APP_TEST="apps/hako-alloc-reclaim-post-drain-owner-transfer-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-545-MIMAP-058A-RECLAIM-POST-DRAIN-OWNER-TRANSFER-INTEGRATION-ROUTE.md"
DESIGN="docs/development/current/main/design/hako-alloc-reclaim-post-drain-owner-transfer-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/reclaim_post_drain_owner_transfer_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_reclaim_post_drain_owner_transfer_guard.sh"

printf '[%s] checking MIMAP-058A post-drain owner-transfer integration\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-058A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-058A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-058A granularity' "$PLAN" "granularity SSOT must describe MIMAP-058A"
guard_expect_in_file "$TAG" 'MIMAP-058A reclaim post-drain owner-transfer integration route' "$JOINT" "joint order must name current row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-058A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-058A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-058A"
guard_expect_in_file "$TAG" 'memory.reclaim_post_drain_owner_transfer_box = "memory/reclaim_post_drain_owner_transfer_box.hako"' "$MODULE" "hako_alloc module must export MIMAP-058A owner"
guard_expect_in_file "$TAG" 'reclaim_post_drain_owner_transfer_box.hako` owns MIMAP-058A' "$MEMORY_README" "memory README must define MIMAP-058A owner"
guard_expect_in_file "$TAG" 'box HakoAllocReclaimPostDrainOwnerTransferReport' "$OWNER" "MIMAP-058A report box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocReclaimPostDrainOwnerTransfer' "$OWNER" "MIMAP-058A integration owner must exist"
guard_expect_in_file "$TAG" 'executePostDrainTransfer' "$OWNER" "MIMAP-058A owner must expose integration method"
guard_expect_in_file "$TAG" 'HakoAllocReclaimRemoteFreeDrainExecution' "$OWNER" "MIMAP-058A owner must compose drain execution"
guard_expect_in_file "$TAG" 'HakoAllocReclaimOwnerTransferExecution' "$OWNER" "MIMAP-058A owner must compose owner transfer"
guard_expect_in_file "$TAG" 'would_execute_full_reclaim: i64 = 0' "$OWNER" "full reclaim must stay inactive"
guard_expect_in_file "$TAG" 'would_schedule_thread: i64 = 0' "$OWNER" "thread scheduling must stay inactive"
guard_expect_in_file "$TAG" 'would_call_page_source: i64 = 0' "$OWNER" "page-source calls must stay inactive"
guard_expect_in_file "$TAG" 'would_unreserve: i64 = 0' "$OWNER" "unreserve must stay inactive"
guard_expect_in_file "$TAG" 'would_release_osvm: i64 = 0' "$OWNER" "OS release must stay inactive"
guard_expect_in_file "$TAG" 'would_activate_provider: i64 = 0' "$OWNER" "provider activation must stay inactive"
guard_expect_in_file "$TAG" 'HakoAllocReclaimPostDrainOwnerTransfer' "$APP" "MIMAP-058A proof must construct owner"
guard_expect_in_file "$TAG" 'check "mimap058a reclaim post drain owner transfer"' "$APP" "MIMAP-058A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|drainRemote[[:space:]]*\(|drain_remote[[:space:]]*\(|collectOne[[:space:]]*\(|pushRetry[[:space:]]*\(|peekHead[[:space:]]*\(|peekNext[[:space:]]*\(|releaseLocal[[:space:]]*\(|executeReclaim[[:space:]]*\(|reclaimPage[[:space:]]*\(|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-058A must not add real atomics, pointer drain, page release, full reclaim, or page-source/OS release seams" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-058A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-reclaim-post-drain-owner-transfer-proof|HakoAllocReclaimPostDrainOwnerTransfer|reclaim_post_drain_owner_transfer' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-058A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_reclaim_post_drain_owner_transfer_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-058A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap058a_reclaim_post_drain.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap058a.mir.json"
exe_out="$tmp_dir/mimap058a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-reclaim-post-drain-owner-transfer-proof' "$vm_log"
rg -F -q 'drained_transfer=1,1,0,0,2' "$vm_log"
rg -F -q 'direct_transfer=0,1,0,4' "$vm_log"
rg -F -q 'pending_remains=1,0,2,1' "$vm_log"
rg -F -q 'drain_block=0,0,1,3' "$vm_log"
rg -F -q 'transfer_block=0,0,3,2,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0' "$vm_log"
rg -F -q 'counts=5,2,1,1,1,74,3,1,0' "$vm_log"
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
    "HakoAllocReclaimPostDrainOwnerTransfer.executePostDrainTransfer/8",
    "HakoAllocReclaimPostDrainOwnerTransfer.report/11",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in ("HakoAllocReclaimPostDrainOwnerTransferReport", "HakoAllocReclaimPostDrainOwnerTransfer"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

fields = {field.get("name"): field for field in plans["HakoAllocReclaimPostDrainOwnerTransferReport"].get("fields", [])}
required_fields = {
    "did_drain",
    "did_transfer",
    "reason",
    "page_id",
    "pending_before",
    "pending_after",
    "owner_after",
    "drain_reason",
    "transfer_reason",
    "did_change_modeled_pending",
    "did_change_modeled_owner",
    "would_execute_full_reclaim",
    "would_schedule_thread",
    "would_call_page_source",
    "would_unreserve",
    "would_release_osvm",
    "would_activate_provider",
}
missing_fields = sorted(name for name in required_fields if name not in fields)
if missing_fields:
    raise SystemExit(f"missing report fields: {missing_fields}")

for name in required_fields:
    field = fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad report field {name}: {field}")

print("[mimap058a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-reclaim-post-drain-owner-transfer-proof' "$run_log"
rg -F -q 'drained_transfer=1,1,0,0,2' "$run_log"
rg -F -q 'direct_transfer=0,1,0,4' "$run_log"
rg -F -q 'pending_remains=1,0,2,1' "$run_log"
rg -F -q 'drain_block=0,0,1,3' "$run_log"
rg -F -q 'transfer_block=0,0,3,2,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0' "$run_log"
rg -F -q 'counts=5,2,1,1,1,74,3,1,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

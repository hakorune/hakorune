#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-result-contract"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

PAGE_HEAP="lang/src/hako_alloc/memory/page_heap_box.hako"
APP="apps/mimalloc-result-contract-proof/main.hako"
APP_TEST="apps/mimalloc-result-contract-proof/test.sh"
APP_README="apps/mimalloc-result-contract-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-201-M190-NULLABLE-FAILURE-HANDLE-CONTRACT.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_result_contract_guard.sh"

echo "[$TAG] checking M190 nullable/failure handle contract"

guard_require_files \
  "$TAG" \
  "$PAGE_HEAP" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$PLAN" \
  "$INDEX"

guard_expect_in_file "$TAG" 'box HakoAllocHandleResult' "$PAGE_HEAP" "M190 must add a typed handle result wrapper"
guard_expect_in_file "$TAG" 'allocateResult\(size\)' "$PAGE_HEAP" "M190 must add allocation result entry"
guard_expect_in_file "$TAG" 'reallocResult\(handle, requested_size\)' "$PAGE_HEAP" "M190 must add realloc result entry"
guard_expect_in_file "$TAG" 'isLiveHandle\(handle\)' "$PAGE_HEAP" "M190 must reject stale handles before replacement allocation"
guard_expect_in_file "$TAG" 'if me\.isLiveHandle\(handle\) == 0' "$PAGE_HEAP" "M190 realloc must reject stale handles before replacement allocation"
guard_expect_in_file "$TAG" 'me\.release\(replacement\)' "$PAGE_HEAP" "M190 realloc must rollback replacement on old-release failure"
guard_expect_in_file "$TAG" 'new HakoAllocHandleResult\(0, 1, null\)' "$PAGE_HEAP" "M190 must name null-handle failures"
guard_expect_in_file "$TAG" 'new HakoAllocHandleResult\(0, 2, null\)' "$PAGE_HEAP" "M190 must name invalid-size failures"
guard_expect_in_file "$TAG" 'new HakoAllocHandleResult\(0, 3, null\)' "$PAGE_HEAP" "M190 must name stale-handle failures"
guard_expect_in_file "$TAG" 'new HakoAllocHandleResult\(0, 4, null\)' "$PAGE_HEAP" "M190 must name allocation failures"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_heap_box as HakoAllocPageHeapBox' "$APP" "proof app must import page heap API"
guard_expect_in_file "$TAG" 'M190 nullable / failure handle contract` \| Complete' "$PLAN" "plan must mark M190 complete"
guard_expect_in_file "$TAG" '293x-201 M190 Nullable / Failure Handle Contract' "$CARD" "missing M190 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M190 guard"

if rg -n 'provider|install_hook|hook_install|global_allocator|hako_mem_|externcall|OSVM|OsVm|page_map|HugeRelease|huge_release|alignedResult|hugeResult' \
  "$PAGE_HEAP" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M190 leaked beyond nullable/failure handle contract scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-result-contract|HakoAllocHandleResult|allocateResult|reallocResult|result_contract' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M190 app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

tmp_dir="$(mktemp -d /tmp/hakorune_m190_result_contract.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
vm_out="$tmp_dir/vm.out"
vm_err="$tmp_dir/vm.err"

if [[ -n "${HAKORUNE_BIN:-}" ]]; then
  HAKO_CMD=("$HAKORUNE_BIN")
else
  HAKO_CMD=(cargo run -q --bin hakorune --)
fi

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "${HAKO_CMD[@]}" --backend vm "$APP" >"$vm_out" 2>"$vm_err"

rg -F -q 'mimalloc-result-contract-proof' "$vm_out"
rg -F -q 'alloc=1,0,0,7,16' "$vm_out"
rg -F -q 'same=1,0,0,7,16' "$vm_out"
rg -F -q 'moved=1,0,1,3,48' "$vm_out"
rg -F -q 'invalid_size=0,2' "$vm_out"
rg -F -q 'oversized=0,4' "$vm_out"
rg -F -q 'null_realloc=0,1' "$vm_out"
rg -F -q 'stale=0,3' "$vm_out"
rg -F -q 'heap=0,0,8,4,0,56' "$vm_out"
rg -F -q 'summary=ok' "$vm_out"

pure_first_guard_build_toolchain

mir_json="$tmp_dir/m190.mir.json"
exe_out="$tmp_dir/m190.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

vm_filtered="$tmp_dir/vm.filtered"
exe_filtered="$tmp_dir/exe.filtered"
rg -F \
  -e 'mimalloc-result-contract-proof' \
  -e 'alloc=' \
  -e 'same=' \
  -e 'moved=' \
  -e 'invalid_size=' \
  -e 'oversized=' \
  -e 'null_realloc=' \
  -e 'stale=' \
  -e 'heap=' \
  -e 'summary=' \
  "$vm_out" >"$vm_filtered"
rg -F \
  -e 'mimalloc-result-contract-proof' \
  -e 'alloc=' \
  -e 'same=' \
  -e 'moved=' \
  -e 'invalid_size=' \
  -e 'oversized=' \
  -e 'null_realloc=' \
  -e 'stale=' \
  -e 'heap=' \
  -e 'summary=' \
  "$run_log" >"$exe_filtered"

if ! diff -u "$vm_filtered" "$exe_filtered" >/tmp/"$TAG".diff 2>&1; then
  echo "[$TAG] ERROR: VM and pure-first EXE proof lines differ" >&2
  cat /tmp/"$TAG".diff >&2
  rm -f /tmp/"$TAG".diff
  exit 1
fi
rm -f /tmp/"$TAG".diff

cat "$vm_out"
echo "[$TAG] ok"

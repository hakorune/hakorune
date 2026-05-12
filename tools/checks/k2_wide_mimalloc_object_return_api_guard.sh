#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-object-return-api"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

PAGE_HEAP="lang/src/hako_alloc/memory/page_heap_box.hako"
APP="apps/mimalloc-object-return-api-proof/main.hako"
APP_TEST="apps/mimalloc-object-return-api-proof/test.sh"
APP_README="apps/mimalloc-object-return-api-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-200-M189-OBJECT-RETURN-ALLOCATOR-API.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_object_return_api_guard.sh"

echo "[$TAG] checking M189 object-return allocator API"

guard_require_files \
  "$TAG" \
  "$PAGE_HEAP" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$PLAN" \
  "$INDEX"

guard_expect_in_file "$TAG" 'return new HakoAllocHandle' "$PAGE_HEAP" "allocate must keep returning handle objects"
guard_expect_in_file "$TAG" 'resizeInPlace\(handle, requested_size\)' "$PAGE_HEAP" "M189 must add same-object resize support"
guard_expect_in_file "$TAG" 'realloc\(handle, requested_size\)' "$PAGE_HEAP" "M189 must add object-return realloc"
guard_expect_in_file "$TAG" 'return replacement' "$PAGE_HEAP" "M189 realloc grow path must return replacement object"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_heap_box as HakoAllocPageHeapBox' "$APP" "proof app must import page heap API"
guard_expect_in_file "$TAG" 'M189 object-return allocate/realloc EXE parity` \| Complete' "$PLAN" "plan must mark M189 complete"
guard_expect_in_file "$TAG" '293x-200 M189 Object-Return Allocator API' "$CARD" "missing M189 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M189 guard"

if rg -n 'provider|install_hook|hook_install|global_allocator|hako_mem_|externcall|OSVM|OsVm|page_map|HugeRelease|huge_release' \
  "$PAGE_HEAP" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M189 leaked beyond object-return allocator API scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-object-return-api|HakoAllocHandle|HakoAllocHeap|object_return_api' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M189 app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

tmp_dir="$(mktemp -d /tmp/hakorune_m189_object_return.XXXXXX)"
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

rg -F -q 'mimalloc-object-return-api-proof' "$vm_out"
rg -F -q 'alloc=0,7,16' "$vm_out"
rg -F -q 'same=0,7,16' "$vm_out"
rg -F -q 'moved=1,3,48' "$vm_out"
rg -F -q 'fail=true' "$vm_out"
rg -F -q 'release_count=1' "$vm_out"
rg -F -q 'heap=0,0,8,4,0,56' "$vm_out"
rg -F -q 'summary=ok' "$vm_out"

pure_first_guard_build_toolchain

mir_json="$tmp_dir/m189.mir.json"
exe_out="$tmp_dir/m189.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

vm_filtered="$tmp_dir/vm.filtered"
exe_filtered="$tmp_dir/exe.filtered"
rg -F \
  -e 'mimalloc-object-return-api-proof' \
  -e 'alloc=' \
  -e 'same=' \
  -e 'moved=' \
  -e 'fail=' \
  -e 'release_count=' \
  -e 'heap=' \
  -e 'summary=' \
  "$vm_out" >"$vm_filtered"
rg -F \
  -e 'mimalloc-object-return-api-proof' \
  -e 'alloc=' \
  -e 'same=' \
  -e 'moved=' \
  -e 'fail=' \
  -e 'release_count=' \
  -e 'heap=' \
  -e 'summary=' \
  "$run_log" >"$exe_filtered"

if ! diff -u "$vm_filtered" "$exe_filtered" >/tmp/"$TAG".diff 2>&1; then
  echo "[$TAG] ERROR: VM and pure-first EXE output differ" >&2
  cat /tmp/"$TAG".diff >&2
  rm -f /tmp/"$TAG".diff
  exit 1
fi
rm -f /tmp/"$TAG".diff

cat "$vm_out"
echo "[$TAG] ok"

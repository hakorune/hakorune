#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-mem-threadsafe-abi"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-397-MIMAP-THREADSAFE-ABI-001-THREAD-SAFE-HAKO-MEM-ABI.md"
SUBSTRATE="docs/reference/runtime/substrate-capabilities.md"
RETURN_PROOF="docs/development/current/main/design/return-proof-vocabulary-ssot.md"
ABI_SURFACE="docs/reference/abi/nyrt_host_surface_v0.md"
ABI_MATRIX="docs/reference/abi/ABI_BOUNDARY_MATRIX.md"
KERNEL_MEM="crates/nyash_kernel/src/exports/mem.rs"
C_MEM="lang/c-abi/shims/hako_diag_mem_shared_impl.inc"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_mem_threadsafe_abi_guard.sh"

echo "[$TAG] running MIMAP-THREADSAFE-ABI-001 hako_mem thread-safe ABI guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$SUBSTRATE" \
  "$RETURN_PROOF" \
  "$ABI_SURFACE" \
  "$ABI_MATRIX" \
  "$KERNEL_MEM" \
  "$C_MEM" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "tools/checks/k2_wide_hako_mem_runtime_decl_guard.sh" \
  "tools/checks/k2_wide_hako_mem_extern_pure_first_guard.sh"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

for path in "$CARD" "$SUBSTRATE" "$RETURN_PROOF" "$ABI_SURFACE" "$ABI_MATRIX"; do
  guard_expect_in_file "$TAG" 'thread-safe' "$path" "$path must mention the thread-safe hako_mem ABI contract"
  guard_expect_in_file "$TAG" 'distinct allocation' "$path" "$path must constrain hako_mem thread-safety to distinct allocations"
done

for symbol in hako_mem_alloc hako_mem_realloc hako_mem_free; do
  guard_expect_in_file "$TAG" "$symbol" "$SUBSTRATE" "$SUBSTRATE must document $symbol"
  guard_expect_in_file "$TAG" "$symbol" "$RETURN_PROOF" "$RETURN_PROOF must document $symbol"
  guard_expect_in_file "$TAG" "$symbol" "$C_MEM" "$C_MEM must define $symbol"
done

guard_expect_in_file "$TAG" 'pub extern "C" fn hako_mem_alloc' "$KERNEL_MEM" "kernel must export hako_mem_alloc"
guard_expect_in_file "$TAG" 'pub extern "C" fn hako_mem_realloc' "$KERNEL_MEM" "kernel must export hako_mem_realloc"
guard_expect_in_file "$TAG" 'pub extern "C" fn hako_mem_free' "$KERNEL_MEM" "kernel must export hako_mem_free"
guard_expect_in_file "$TAG" 'hako_mem_alloc_realloc_free_are_thread_safe_for_distinct_allocations' "$KERNEL_MEM" "kernel must carry concurrent distinct-allocation smoke"
guard_expect_in_file "$TAG" '_Thread_local const char\* hako_tls_last_error' "$C_MEM" "C shim diagnostics must remain thread-local"
guard_expect_in_file "$TAG" 'platform malloc/realloc/free' "$C_MEM" "C shim must document platform allocator ownership"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-THREADSAFE-ABI guard"

if rg -n '#\[global_allocator\]|install_hook|provider[A-Za-z0-9_]*[[:space:]]*\(|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$KERNEL_MEM" "$C_MEM" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: hako_mem ABI guard leaked provider/hook/global allocator activation" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

cargo test -q -p nyash_kernel hako_mem

bash tools/checks/k2_wide_hako_mem_runtime_decl_guard.sh
bash tools/checks/k2_wide_hako_mem_extern_pure_first_guard.sh

echo "[$TAG] ok"

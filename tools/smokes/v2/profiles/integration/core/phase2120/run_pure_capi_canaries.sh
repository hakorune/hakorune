#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

echo "[phase2120/compat] integration pure-lowering canaries"

export NYASH_LLVM_USE_CAPI=1
export HAKO_V1_EXTERN_PROVIDER_C_ABI=1
export HAKO_CAPI_PURE=${HAKO_CAPI_PURE:-1}

ffi_candidates=(
  "$ROOT/target/release/libhako_llvmc_ffi.so"
  "$ROOT/lib/libhako_llvmc_ffi.so"
)
ffi_found=0
for c in "${ffi_candidates[@]}"; do
  if [[ -f "$c" ]]; then ffi_found=1; break; fi
done

if [[ "$ffi_found" != "1" ]]; then
  echo "[phase2120] SKIP pure canaries (FFI .so not found). Hint: bash tools/build_hako_llvmc_ffi.sh" >&2
  exit 0
fi

ACTIVE_PURE_CANARIES=(
  'core/phase2120/s3_link_run_llvmcapi_pure_ternary_collect_canary_vm.sh'
  'core/phase2120/s3_link_run_llvmcapi_pure_map_set_size_canary_vm.sh'
  'core/phase2120/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh'
  'core/phase2120/s3_link_run_llvmcapi_pure_loop_count_canary_vm.sh'
)

ARCHIVE_PURE_CANARIES=(
  'core/phase2120/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh'
  'core/phase2120/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh'
  'core/phase2120/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh'
)

for filter in "${ACTIVE_PURE_CANARIES[@]}"; do
  bash "$ROOT/tools/smokes/v2/run.sh" --profile integration --filter "$filter"
done

echo "[phase2120/compat] archive-backed historical pure canaries"
for filter in "${ARCHIVE_PURE_CANARIES[@]}"; do
  bash "$ROOT/tools/smokes/v2/run.sh" --profile archive --filter "$filter"
done

echo "[phase2120] pure canaries done."

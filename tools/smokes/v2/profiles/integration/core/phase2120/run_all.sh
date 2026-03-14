#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

echo "[phase2120/compat] historical C-API pure (emit+link) reps"

# Flags for pure C-API path
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
  echo "[phase2120] SKIP (FFI .so not found). Hint: bash tools/build_hako_llvmc_ffi.sh" >&2
  exit 0
fi

bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/s3_link_run_llvmcapi_pure_ternary_collect_canary_vm.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/s3_link_run_llvmcapi_pure_map_set_size_canary_vm.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/s3_link_run_llvmcapi_pure_loop_count_canary_vm.sh'
# Unbox (map.get -> integer.get_h) reps
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh'
# VM Adapter reps (optional; skips if adapter disabled)
# Adapter tests (inline Hako): only run if inline using is supported
CHECK_FILE="/tmp/hako_inline_using_check_$$.hako"
cat > "$CHECK_FILE" <<'HCODE'
using "selfhost.vm.helpers.mir_call_v1_handler" as MirCallV1HandlerBox
static box Main { method main(args) { return 0 } }
HCODE
set +e
NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 NYASH_FEATURES=stage3 \
  NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 "$ROOT/target/release/hakorune" --backend vm "$CHECK_FILE" >/dev/null 2>&1
USING_OK=$?
rm -f "$CHECK_FILE" || true
set -e
if [ "$USING_OK" -eq 0 ]; then
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/s3_vm_adapter_array_len_canary_vm.sh'
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/s3_vm_adapter_array_length_alias_canary_vm.sh'
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/s3_vm_adapter_array_size_alias_canary_vm.sh'
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/s3_vm_adapter_array_len_per_recv_canary_vm.sh'
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/s3_vm_adapter_map_size_struct_canary_vm.sh'
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/s3_vm_adapter_register_userbox_length_canary_vm.sh'
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/s3_vm_adapter_map_len_alias_state_canary_vm.sh'
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2120/s3_vm_adapter_map_length_alias_state_canary_vm.sh'
else
  echo "[phase2120] SKIP adapter reps (inline using unsupported)" >&2
fi

echo "[phase2120] Done."

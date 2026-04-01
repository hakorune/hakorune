#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

echo "[phase2111] C-API (emit-only) reps — ternary/map (flags ON)"

# C‑API トグルを明示ON（llvmliteは保守と比較用途で残す）
export NYASH_LLVM_USE_CAPI=1
export HAKO_V1_EXTERN_PROVIDER_C_ABI=1
export HAKO_LLVM_OPT_LEVEL=${HAKO_LLVM_OPT_LEVEL:-0}

# 存在チェック（未ビルドなら SKIP 案内のみ）
ffi_candidates=(
  "$ROOT/target/release/libhako_llvmc_ffi.so"
  "$ROOT/lib/libhako_llvmc_ffi.so"
)
ffi_found=0
for c in "${ffi_candidates[@]}"; do
  if [[ -f "$c" ]]; then ffi_found=1; break; fi
done

if [[ "$ffi_found" != "1" ]]; then
  echo "[phase2111] SKIP (C-API FFI library not found). Build libhako_llvmc_ffi.so first." >&2
  echo "[phase2111] Tried: ${ffi_candidates[*]}" >&2
  echo "[phase2111] Hint: bash tools/build_hako_llvmc_ffi.sh" >&2
  exit 0
fi

bash "$ROOT/tools/smokes/v2/run.sh" --profile archive --filter 'core/phase2111/s3_link_run_llvmcapi_ternary_collect_canary_vm.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile archive --filter 'core/phase2111/s3_link_run_llvmcapi_map_set_size_canary_vm.sh'

echo "[phase2111] Done."

#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

suite="phase29x-legacy-emit-object-evidence"

echo "[archive/phase29x-legacy-emit-object-evidence] legacy emit_object replay bundle"

# C-API トグルを明示ON（llvmliteは保守と比較用途で残す）
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
  echo "[archive/phase29x-legacy-emit-object-evidence] SKIP (C-API FFI library not found). Build libhako_llvmc_ffi.so first." >&2
  echo "[archive/phase29x-legacy-emit-object-evidence] Tried: ${ffi_candidates[*]}" >&2
  echo "[archive/phase29x-legacy-emit-object-evidence] Hint: bash tools/build_hako_llvmc_ffi.sh" >&2
  exit 0
fi

bash "$ROOT/tools/smokes/v2/run.sh" --profile archive --suite "$suite"

echo "[archive/phase29x-legacy-emit-object-evidence] Done."

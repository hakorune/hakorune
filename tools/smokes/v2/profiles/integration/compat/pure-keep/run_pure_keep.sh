#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
SUITE="compat/pure-keep"

echo "[compat/pure-keep] integration pure-lowering keep canaries"

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

bash "$ROOT/tools/smokes/v2/run.sh" --profile integration --suite "$SUITE"

echo "[compat/pure-keep] pure canaries done."

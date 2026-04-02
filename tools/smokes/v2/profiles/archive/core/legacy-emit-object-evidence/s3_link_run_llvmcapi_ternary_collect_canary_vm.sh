#!/bin/bash
# S3 (C‑API): threeblock collect → rc=44（C‑API FFIが未整備なら SKIP）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Require C‑API toggle（未設定なら自分でON）
export NYASH_LLVM_USE_CAPI=${NYASH_LLVM_USE_CAPI:-1}
export HAKO_V1_EXTERN_PROVIDER_C_ABI=${HAKO_V1_EXTERN_PROVIDER_C_ABI:-1}
if [[ "${NYASH_LLVM_USE_CAPI}" != "1" ]]; then
  echo "[SKIP] s3_link_run_llvmcapi_ternary_collect_canary_vm (NYASH_LLVM_USE_CAPI!=1)" >&2
  exit 0
fi

# FFI library presence check
ffi_candidates=(
  "$ROOT/target/release/libhako_llvmc_ffi.so"
  "$ROOT/lib/libhako_llvmc_ffi.so"
)
ffi_found=0
for c in "${ffi_candidates[@]}"; do
  if [[ -f "$c" ]]; then ffi_found=1; break; fi
done
if [[ "$ffi_found" != "1" ]]; then
  echo "[SKIP] s3_link_run_llvmcapi_ternary_collect_canary_vm (FFI library not found)" >&2
  exit 0
fi

# Generate v1 JSON and compile via provider (C‑API)
json=$(bash "$ROOT/tools/selfhost/examples/gen_v1_threeblock_collect.sh")
export _MIR_JSON="$json"
exe="/tmp/s3_exe_ternary_capi_$$"

code=$(cat <<'HCODE'
static box Main { method main(args) {
  local j = env.get("_MIR_JSON")
  local a = new ArrayBox(); a.push(j)
  // compile (C‑API route)
  local obj = hostbridge.extern_invoke("env.codegen", "emit_object", a)
  if obj == null { print("NULL"); return 1 }
  // link (C‑API route)
  local b = new ArrayBox(); b.push(obj); b.push(env.get("_EXE_OUT"))
  local exe = hostbridge.extern_invoke("env.codegen", "link_object", b)
  if exe == null { print("NULL"); return 1 }
  print("" + exe)
  return 0
} }
HCODE
)

export _EXE_OUT="$exe"
out=$(run_nyash_vm -c "$code")
path=$(echo "$out" | tail -n1 | tr -d '\r')
if [[ ! -f "$path" ]]; then echo "[FAIL] exe not produced: $path" >&2; exit 1; fi
set +e
"$path" >/dev/null 2>&1
rc=$?
set -e
if [[ "$rc" -ne 44 ]]; then echo "[FAIL] rc=$rc (expect 44)" >&2; exit 1; fi
echo "[PASS] s3_link_run_llvmcapi_ternary_collect_canary_vm ($path)"
exit 0

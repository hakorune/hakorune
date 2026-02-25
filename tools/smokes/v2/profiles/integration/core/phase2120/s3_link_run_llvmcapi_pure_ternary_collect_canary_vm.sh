#!/bin/bash
# S3 (C‑API pure): threeblock collect → rc=44（pureフラグON）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

export NYASH_LLVM_USE_CAPI=${NYASH_LLVM_USE_CAPI:-1}
export HAKO_V1_EXTERN_PROVIDER_C_ABI=${HAKO_V1_EXTERN_PROVIDER_C_ABI:-1}
export HAKO_CAPI_PURE=${HAKO_CAPI_PURE:-1}
if [[ "${NYASH_LLVM_USE_CAPI}" != "1" || "${HAKO_V1_EXTERN_PROVIDER_C_ABI}" != "1" || "${HAKO_CAPI_PURE}" != "1" ]]; then
  echo "[SKIP] s3_link_run_llvmcapi_pure_ternary_collect_canary_vm (toggles off)" >&2
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
  echo "[SKIP] s3_link_run_llvmcapi_pure_ternary_collect_canary_vm (FFI library not found)" >&2
  exit 0
fi

json=$(bash "$ROOT/tools/selfhost/examples/gen_v1_threeblock_collect.sh")
export _MIR_JSON="$json"

code=$(cat <<'HCODE'
static box Main { method main(args) {
  local j = env.get("_MIR_JSON")
  local a = new ArrayBox(); a.push(j)
  local obj = hostbridge.extern_invoke("env.codegen", "emit_object", a)
  if obj == null { print("NULL"); return 1 }
  local b = new ArrayBox(); b.push(obj); b.push(env.get("_EXE_OUT"))
  local exe = hostbridge.extern_invoke("env.codegen", "link_object", b)
  if exe == null { print("NULL"); return 1 }
  print("" + exe)
  return 0
} }
HCODE
)

sha_cmd=""
if command -v sha1sum >/dev/null 2>&1; then sha_cmd="sha1sum"; elif command -v shasum >/dev/null 2>&1; then sha_cmd="shasum"; fi

last_rc=""
last_hash=""
last_size=""
get_size() {
  if stat -c %s "$1" >/dev/null 2>&1; then stat -c %s "$1"; elif stat -f %z "$1" >/dev/null 2>&1; then stat -f %z "$1"; else echo 0; fi
}
for i in 1 2 3; do
  exe="/tmp/s3_exe_ternary_capi_pure_${$}_${i}"
  export _EXE_OUT="$exe"
  out=$(run_nyash_vm -c "$code")
  path=$(echo "$out" | tail -n1 | tr -d '\r')
  if [[ ! -f "$path" ]]; then echo "[FAIL] exe not produced: $path" >&2; exit 1; fi
  set +e
  "$path" >/dev/null 2>&1
  rc=$?
  set -e
  if [[ "$rc" -ne 44 ]]; then echo "[FAIL] rc=$rc (expect 44)" >&2; exit 1; fi
  # Optional: print hash for inspection (determinism)
  if [[ -n "$sha_cmd" ]]; then "$sha_cmd" "$path" | awk '{print "[hash] "$1}'; fi
  cur_size=$(get_size "$path"); echo "[size] $cur_size"
  if [[ -z "$last_size" ]]; then last_size="$cur_size"; else
    if [[ "$cur_size" != "$last_size" ]]; then echo "[FAIL] size mismatch ($cur_size != $last_size)" >&2; exit 1; fi
  fi
  if [[ "${NYASH_HASH_STRICT:-0}" == "1" && -n "$sha_cmd" ]]; then
    cur_hash=$($sha_cmd "$path" | awk '{print $1}')
    if [[ -z "$last_hash" ]]; then last_hash="$cur_hash"; else
      if [[ "$cur_hash" != "$last_hash" ]]; then echo "[FAIL] hash mismatch ($cur_hash != $last_hash)" >&2; exit 1; fi
    fi
  fi
  last_rc="$rc"
done
echo "[PASS] s3_link_run_llvmcapi_pure_ternary_collect_canary_vm"
exit 0

#!/bin/bash
# S3 (C‑API pure/TM): map set→get→ret（自動アンボックス）→ rc=9
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

export NYASH_LLVM_USE_CAPI=${NYASH_LLVM_USE_CAPI:-1}
export HAKO_V1_EXTERN_PROVIDER_C_ABI=${HAKO_V1_EXTERN_PROVIDER_C_ABI:-1}
export HAKO_CAPI_PURE=${HAKO_CAPI_PURE:-1}
if [[ "${NYASH_LLVM_USE_CAPI}" != "1" || "${HAKO_V1_EXTERN_PROVIDER_C_ABI}" != "1" || "${HAKO_CAPI_PURE}" != "1" ]]; then
  echo "[SKIP] s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm (toggles off)" >&2
  exit 0
fi

ffi_candidates=(
  "$ROOT/target/release/libhako_llvmc_ffi.so"
  "$ROOT/lib/libhako_llvmc_ffi.so"
)
ffi_found=0
for c in "${ffi_candidates[@]}"; do
  if [[ -f "$c" ]]; then ffi_found=1; break; fi
done
if [[ "$ffi_found" != "1" ]]; then
  echo "[SKIP] s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm (FFI library not found)" >&2
  exit 0
fi

json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[
  {"op":"const","dst":2,"value":{"type":"i64","value":5}},
  {"op":"const","dst":3,"value":{"type":"i64","value":9}},
  {"op":"mir_call","dst":1,"mir_call":{"callee":{"type":"Constructor","box_name":"MapBox"},"args":[],"effects":[]}},
  {"op":"mir_call","mir_call":{"callee":{"type":"Method","box_name":"MapBox","method":"set","receiver":1},"args":[2,3],"effects":[]}},
  {"op":"mir_call","dst":4,"mir_call":{"callee":{"type":"Method","box_name":"MapBox","method":"get","receiver":1},"args":[2],"effects":[]}},
  {"op":"ret","value":4}
]}]}]}'
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

sha_cmd=""; if command -v sha1sum >/dev/null 2>&1; then sha_cmd="sha1sum"; elif command -v shasum >/dev/null 2>&1; then sha_cmd="shasum"; fi
last_size=""; last_hash=""
for i in 1 2 3; do
  exe="/tmp/s3_exe_map_get_unbox_pure_${$}_${i}"
  export _EXE_OUT="$exe"
  out=$(run_nyash_vm -c "$code")
  path=$(echo "$out" | tail -n1 | tr -d '\r')
  if [[ ! -f "$path" ]]; then echo "[FAIL] exe not produced: $path" >&2; exit 1; fi
  set +e; "$path" >/dev/null 2>&1; rc=$?; set -e
  if [[ "$rc" -ne 9 ]]; then echo "[FAIL] rc=$rc (expect 9)" >&2; exit 1; fi
  if [[ -n "$sha_cmd" ]]; then "$sha_cmd" "$path" | awk '{print "[hash] "$1}'; fi
  sz=$(stat -c %s "$path" 2>/dev/null || stat -f %z "$path" 2>/dev/null || echo 0); echo "[size] $sz"
  if [[ -z "$last_size" ]]; then last_size="$sz"; else if [[ "$sz" != "$last_size" ]]; then echo "[FAIL] size mismatch" >&2; exit 1; fi; fi
done
echo "[PASS] s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm"
exit 0


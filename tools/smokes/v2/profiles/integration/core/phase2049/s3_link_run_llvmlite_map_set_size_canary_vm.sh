#!/bin/bash
# S3: Map set→size → rc=1
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

if [ "${NYASH_LLVM_S3:-auto}" = "0" ]; then
  echo "[SKIP] s3_link_run_llvmlite_map_set_size_canary_vm (NYASH_LLVM_S3=0)" >&2
  exit 0
fi
if ! command -v llvm-config-18 >/dev/null 2>&1; then
  echo "[SKIP] s3_link_run_llvmlite_map_set_size_canary_vm (LLVM18 not available)" >&2
  exit 0
fi

json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[
  {"op":"mir_call","dst":0, "callee":{"type":"Constructor","box_type":"MapBox"}, "args":[], "effects":[]},
  {"op":"const","dst":1, "value": {"type": "i64", "value": 10}},
  {"op":"const","dst":3, "value": {"type": "i64", "value": 99}},
  {"op":"mir_call", "callee":{"type":"Method","box_name":"MapBox","method":"set","receiver":0}, "args":[1,3], "effects":[]},
  {"op":"mir_call","dst":2, "callee":{"type":"Method","box_name":"MapBox","method":"size","receiver":0}, "args":[], "effects":[]},
  {"op":"ret","value":2}
]}]}]}'

tmp_json="/tmp/s3_v1_map_$$.json"; printf '%s' "$json" > "$tmp_json"
exe="/tmp/s3_exe_map_$$"

set +e
out=$(NYASH_LLVM_SKIP_BUILD=${NYASH_LLVM_SKIP_BUILD:-1} \
      bash "$ROOT/tools/ny_mir_builder.sh" --in "$tmp_json" --emit exe -o "$exe" 2>&1)
rc=$?
set -e
if [ "$rc" -ne 0 ] || [ ! -x "$exe" ]; then
  # If link failed due to unresolved nyash.* runtime symbols (map API not part of kernel), SKIP.
  if printf '%s' "$out" | grep -q "undefined reference to \`nyash\.map\."; then
    echo "[SKIP] s3_link_run_llvmlite_map_set_size_canary_vm (missing nyash.map symbols in kernel)" >&2
    exit 0
  fi
  echo "[FAIL] s3_link_run_llvmlite_map_set_size_canary_vm (builder rc=$rc)" >&2
  printf '%s\n' "$out" | sed -n '1,160p' >&2
  exit 1
fi

set +e
"$exe" >/dev/null 2>&1
erc=$?
set -e
if [ "$erc" -ne 1 ]; then
  echo "[FAIL] s3_link_run_llvmlite_map_set_size_canary_vm (exit=$erc, expect 1)" >&2
  exit 1
fi
echo "[PASS] s3_link_run_llvmlite_map_set_size_canary_vm"
exit 0

#!/bin/bash
# S3: v1 → llvmlite obj → link NyRT → run → rc parity (compare→ret; expect rc=1)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

if [ "${NYASH_LLVM_S3:-auto}" = "0" ]; then
  echo "[SKIP] s3_link_run_llvmlite_compare_ret_canary_vm (NYASH_LLVM_S3=0)" >&2
  exit 0
fi
if ! command -v llvm-config-18 >/dev/null 2>&1; then
  echo "[SKIP] s3_link_run_llvmlite_compare_ret_canary_vm (LLVM 18 not available)" >&2
  exit 0
fi

json=$(bash "$ROOT/tools/selfhost/gen_v1_from_builder_compare_ret.sh")
tmp_json="/tmp/s3_v1_ret_$$.json"
printf '%s' "$json" > "$tmp_json"

exe="/tmp/s3_exe_ret_$$"
set +e
out=$(bash "$ROOT/tools/ny_mir_builder.sh" --in "$tmp_json" --emit exe -o "$exe" 2>&1)
rc=$?
set -e
if [ "$rc" -ne 0 ] || [ ! -x "$exe" ]; then
  echo "[FAIL] s3_link_run_llvmlite_compare_ret_canary_vm (builder rc=$rc)" >&2
  printf '%s\n' "$out" | sed -n '1,160p' >&2
  exit 1
fi

set +e
"$exe" >/dev/null 2>&1
erc=$?
set -e
if [ "$erc" -ne 1 ]; then
  echo "[FAIL] s3_link_run_llvmlite_compare_ret_canary_vm (exit=$erc, expect 1)" >&2
  exit 1
fi
echo "[PASS] s3_link_run_llvmlite_compare_ret_canary_vm"
exit 0

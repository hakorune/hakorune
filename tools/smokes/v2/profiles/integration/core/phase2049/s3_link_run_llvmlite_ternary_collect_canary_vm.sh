#!/bin/bash
# S3: threeblock collect → rc=44（ternary相当）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

if [ "${NYASH_LLVM_S3:-auto}" = "0" ]; then
  echo "[SKIP] s3_link_run_llvmlite_ternary_collect_canary_vm (NYASH_LLVM_S3=0)" >&2
  exit 0
fi
if ! command -v llvm-config-18 >/dev/null 2>&1; then
  echo "[SKIP] s3_link_run_llvmlite_ternary_collect_canary_vm (LLVM18 not available)" >&2
  exit 0
fi

json=$(bash "$ROOT/tools/selfhost/examples/gen_v1_threeblock_collect.sh")
tmp_json="/tmp/s3_v1_ternary_$$.json"; printf '%s' "$json" > "$tmp_json"
exe="/tmp/s3_exe_ternary_$$"

set +e
out=$(NYASH_LLVM_SKIP_BUILD=${NYASH_LLVM_SKIP_BUILD:-1} \
      bash "$ROOT/tools/ny_mir_builder.sh" --in "$tmp_json" --emit exe -o "$exe" 2>&1)
rc=$?
set -e
if [ "$rc" -ne 0 ] || [ ! -x "$exe" ]; then
  echo "[FAIL] s3_link_run_llvmlite_ternary_collect_canary_vm (builder rc=$rc)" >&2
  printf '%s\n' "$out" | sed -n '1,160p' >&2
  exit 1
fi

set +e
"$exe" >/dev/null 2>&1
erc=$?
set -e
if [ "$erc" -ne 44 ]; then
  echo "[FAIL] s3_link_run_llvmlite_ternary_collect_canary_vm (exit=$erc, expect 44)" >&2
  exit 1
fi
echo "[PASS] s3_link_run_llvmlite_ternary_collect_canary_vm"
exit 0

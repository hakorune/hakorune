#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

if [[ "${NYASH_LLVM_RUN_LLVMLITE:-0}" != "1" ]]; then
  echo "[phase2100/probe] llvmlite reps are deprecated by default (set NYASH_LLVM_RUN_LLVMLITE=1 to include)"
  exit 0
fi

if ! command -v llvm-config-18 >/dev/null 2>&1; then
  echo "[phase2100/probe] SKIP llvmlite reps (LLVM18 not available)" >&2
  exit 0
fi

echo "[phase2100/probe] S3 (llvmlite+NyRT) reps (opt-in)..."
(cd "$ROOT/crates/nyash-llvm-compiler" && timeout 180 cargo build --release -j 24 >/dev/null 2>&1) || true
(cd "$ROOT/crates/nyash_kernel" && timeout 180 cargo build --release -j 24 >/dev/null 2>&1) || true
NYASH_LLVM_SKIP_BUILD=${NYASH_LLVM_SKIP_BUILD:-1} \
  bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --timeout 120 --filter 'phase2049/s3_link_run_llvmlite_*'

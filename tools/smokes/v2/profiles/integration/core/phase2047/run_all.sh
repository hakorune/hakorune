#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

echo "[phase2047] Running PRIMARY no-fallback reps..."
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2047/primary_no_fallback_*'

echo "[phase2047] Checking provider v1 shape..."
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2047/provider_v1_shape_canary_vm.sh'

echo "[phase2047] Running S1/S2 (builder/provider) reps..."
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2047/*selfhost_s1_s2*'

if [[ -z "${NYASH_LLVM_S3:-}" ]]; then
  if command -v llvm-config-18 >/dev/null 2>&1; then
    export NYASH_LLVM_S3=1
  else
    export NYASH_LLVM_S3=0
  fi
fi

if [[ "${NYASH_LLVM_S3}" == "1" ]]; then
  echo "[phase2047] Running S3 (llvmlite+link+run) reps..."
  if [[ "${NYASH_LLVM_PREBUILD:-1}" == "1" ]]; then
    echo "[phase2047] Prebuilding nyash(llvm) + nyash_kernel (once) ..."
    timeout 180 cargo build --release -j 24 --features llvm >/dev/null 2>&1 || true
    (cd "$ROOT/crates/nyash-llvm-compiler" && timeout 180 cargo build --release -j 24 >/dev/null 2>&1) || true
    (cd "$ROOT/crates/nyash_kernel" && timeout 180 cargo build --release -j 24 >/dev/null 2>&1) || true
  fi
  NYASH_LLVM_S3=1 NYASH_LLVM_SKIP_BUILD=${NYASH_LLVM_SKIP_BUILD:-1} \
    bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2047/s3_*'
else
  echo "[phase2047] Skipping S3 (auto-disabled; export NYASH_LLVM_S3=1 to force)"
fi

echo "[phase2047] Done."

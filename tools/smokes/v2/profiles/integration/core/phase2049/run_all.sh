#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

echo "[phase2049] Running hv1 inline minimal reps (binop/unop/copy)..."
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2049/hv1_inline_*'

echo "[phase2049] Running dev/prod resolver SSOT reps..."
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2049/dev_preinclude_off_on_parity_canary_vm.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2049/dev_alias_entry_canary_vm.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2049/prod_alias_entry_canary_vm.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2049/prod_alias_json_v1_reader_canary_vm.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2049/dev_nested_prelude_json_reader_canary_vm.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2049/prod_nested_prelude_json_reader_canary_vm.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2049/dev_nested_prelude_field_extractor_canary_vm.sh'
bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2049/prod_nested_prelude_field_extractor_canary_vm.sh'

if [[ -z "${NYASH_LLVM_S3:-}" ]]; then
  if command -v llvm-config-18 >/dev/null 2>&1; then
    export NYASH_LLVM_S3=1
  else
    export NYASH_LLVM_S3=0
  fi
fi

if [[ "${NYASH_LLVM_S3}" == "1" ]]; then
  echo "[phase2049] Running S3 reps (llvmlite+link+run)..."
  # Optional prebuild to avoid long builds per test
  if [[ "${NYASH_LLVM_PREBUILD:-1}" == "1" ]]; then
    echo "[phase2049] Prebuilding nyash(llvm) + nyash_kernel (once) ..."
    timeout 180 cargo build --release -j 24 --features llvm >/dev/null 2>&1 || true
    (cd "$ROOT/crates/nyash-llvm-compiler" && timeout 180 cargo build --release -j 24 >/dev/null 2>&1) || true
    (cd "$ROOT/crates/nyash_kernel" && timeout 180 cargo build --release -j 24 >/dev/null 2>&1) || true
  fi
  NYASH_LLVM_S3=1 NYASH_LLVM_SKIP_BUILD=${NYASH_LLVM_SKIP_BUILD:-1} \
    bash "$ROOT/tools/smokes/v2/run.sh" --profile quick --filter 'phase2049/s3_link_run_llvmlite_*'
else
  echo "[phase2049] Skipping S3 reps (auto-disabled; export NYASH_LLVM_S3=1 to force)"
fi
echo "[phase2049] Done."

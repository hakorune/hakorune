#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

echo "[phase2044] llvmlite monitor-only keep"

FILTERS=(
  'core/phase2044/codegen_provider_llvmlite_canary_vm.sh'
  'core/phase2044/codegen_provider_llvmlite_compare_branch_canary_vm.sh'
  'core/phase2044/codegen_provider_llvmlite_const42_canary_vm.sh'
)

for filter in "${FILTERS[@]}"; do
  bash "$ROOT/tools/smokes/v2/run.sh" --profile integration --filter "$filter"
done

echo "[phase2044] llvmlite monitor-only keep done."

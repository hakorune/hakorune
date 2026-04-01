#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

echo "[phase2044] hako_primary_no_fallback bucket"

FILTERS=(
  'core/phase2044/hako_primary_no_fallback_array_size_core_exec_canary_vm.sh'
  'core/phase2044/hako_primary_no_fallback_if_compare_core_exec_canary_vm.sh'
  'core/phase2044/hako_primary_no_fallback_load_store_core_exec_canary_vm.sh'
  'core/phase2044/hako_primary_no_fallback_return_binop_core_exec_canary_vm.sh'
  'core/phase2044/hako_primary_no_fallback_return_bool_core_exec_canary_vm.sh'
  'core/phase2044/hako_primary_no_fallback_return_logical_and_core_exec_canary_vm.sh'
  'core/phase2044/hako_primary_no_fallback_return_logical_and_only_core_exec_canary_vm.sh'
)

for filter in "${FILTERS[@]}"; do
  bash "$ROOT/tools/smokes/v2/run.sh" --profile integration --filter "$filter"
done

echo "[phase2044] hako_primary_no_fallback bucket done."

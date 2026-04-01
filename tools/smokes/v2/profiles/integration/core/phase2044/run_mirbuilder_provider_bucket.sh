#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

echo "[phase2044] mirbuilder_provider bucket"

FILTERS=(
  'core/phase2044/mirbuilder_provider_array_length_alias_core_exec_canary_vm.sh'
  'core/phase2044/mirbuilder_provider_array_push_size_core_exec_canary_vm.sh'
  'core/phase2044/mirbuilder_provider_array_push_size_rc_core_exec_canary_vm.sh'
  'core/phase2044/mirbuilder_provider_emit_core_exec_canary_vm.sh'
  'core/phase2044/mirbuilder_provider_if_compare_core_exec_canary_vm.sh'
  'core/phase2044/mirbuilder_provider_if_elseif_chain_core_exec_canary_vm.sh'
  'core/phase2044/mirbuilder_provider_if_nested_multi_core_exec_canary_vm.sh'
  'core/phase2044/mirbuilder_provider_if_then_match_core_exec_canary_vm.sh'
  'core/phase2044/mirbuilder_provider_local_load_store_core_exec_canary_vm.sh'
  'core/phase2044/mirbuilder_provider_logical_core_exec_canary_vm.sh'
  'core/phase2044/mirbuilder_provider_map_length_alias_core_exec_canary_vm.sh'
  'core/phase2044/mirbuilder_provider_map_set_size_core_exec_canary_vm.sh'
  'core/phase2044/mirbuilder_provider_map_set_size_rc_core_exec_canary_vm.sh'
  'core/phase2044/mirbuilder_provider_match_core_exec_canary_vm.sh'
  'core/phase2044/mirbuilder_provider_match_in_else_core_exec_canary_vm.sh'
  'core/phase2044/mirbuilder_provider_return_binop_core_exec_canary_vm.sh'
  'core/phase2044/mirbuilder_provider_ternary_core_exec_canary_vm.sh'
)

for filter in "${FILTERS[@]}"; do
  bash "$ROOT/tools/smokes/v2/run.sh" --profile integration --filter "$filter"
done

echo "[phase2044] mirbuilder_provider bucket done."

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
BASE_GUARD="$ROOT_DIR/tools/checks/k2_wide_phase296x_perf_userbox_startup_executable_ret0_bucket_nyash_kernel_runtime_registry_rebuild_cache_get_factory_order_by_policy_exact_top_symbol_absence_guard.sh"
TMP_DIR="$(mktemp -d /tmp/hakorune_perf_userbox_startup_executable_ret0_bucket_nyash_kernel_runtime_registry_rebuild_cache_ring0_exact_top_symbol_dominance.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[perf-userbox-startup-executable-ret0-bucket-nyash-kernel-runtime-registry-rebuild-cache-ring0-exact-top-symbol-dominance] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

get_value() {
  local file="$1"
  local key="$2"
  awk -F= -v key="$key" '$1 == key { print $2; exit }' "$file"
}

BASE_REPORT="$TMP_DIR/base.out"
"$BASE_GUARD" >"$BASE_REPORT"
require_line "$BASE_REPORT" "output_contract=perf-userbox-startup-executable-ret0-bucket-nyash-kernel-runtime-registry-rebuild-cache-get-factory-order-by-policy-exact-top-symbol-absence-v0"
require_line "$BASE_REPORT" "summary=ok"

primary_bucket_env_count="$(get_value "$BASE_REPORT" "primary_bucket_env_count")"
primary_bucket_once_count="$(get_value "$BASE_REPORT" "primary_bucket_once_count")"
primary_bucket_path_count="$(get_value "$BASE_REPORT" "primary_bucket_path_count")"
primary_bucket_minimal_main_count="$(get_value "$BASE_REPORT" "primary_bucket_minimal_main_count")"
primary_bucket_nyash_kernel_runtime_count="$(get_value "$BASE_REPORT" "primary_bucket_nyash_kernel_runtime_count")"
primary_bucket_kernel_count="$(get_value "$BASE_REPORT" "primary_bucket_kernel_count")"
primary_bucket_other_count="$(get_value "$BASE_REPORT" "primary_bucket_other_count")"
primary_bucket_missing_count="$(get_value "$BASE_REPORT" "primary_bucket_missing_count")"
primary_bucket_alloc_count="$(get_value "$BASE_REPORT" "primary_bucket_alloc_count")"
primary_bucket_ffi_count="$(get_value "$BASE_REPORT" "primary_bucket_ffi_count")"
primary_bucket_string_count="$(get_value "$BASE_REPORT" "primary_bucket_string_count")"
ret0_count_min="$(get_value "$BASE_REPORT" "ret0_count_min")"
ret0_count_max="$(get_value "$BASE_REPORT" "ret0_count_max")"
trial_summaries="$(get_value "$BASE_REPORT" "trial_summaries")"
registry_top_0_mode="$(get_value "$BASE_REPORT" "registry_top_0_mode")"
registry_top_0_mode_count="$(get_value "$BASE_REPORT" "registry_top_0_mode_count")"
registry_top_1_mode="$(get_value "$BASE_REPORT" "registry_top_1_mode")"
registry_top_1_mode_count="$(get_value "$BASE_REPORT" "registry_top_1_mode_count")"
registry_top_2_mode="$(get_value "$BASE_REPORT" "registry_top_2_mode")"
registry_top_2_mode_count="$(get_value "$BASE_REPORT" "registry_top_2_mode_count")"
registry_exact_rebuild_cache_mode="$(get_value "$BASE_REPORT" "registry_exact_rebuild_cache_mode")"
registry_exact_rebuild_cache_mode_count="$(get_value "$BASE_REPORT" "registry_exact_rebuild_cache_mode_count")"
registry_exact_register_many_mode="$(get_value "$BASE_REPORT" "registry_exact_register_many_mode")"
registry_exact_register_many_mode_count="$(get_value "$BASE_REPORT" "registry_exact_register_many_mode_count")"
registry_exact_create_default_registry_mode="$(get_value "$BASE_REPORT" "registry_exact_create_default_registry_mode")"
registry_exact_create_default_registry_mode_count="$(get_value "$BASE_REPORT" "registry_exact_create_default_registry_mode_count")"
registry_exact_ring0_mode="$(get_value "$BASE_REPORT" "registry_exact_ring0_mode")"
registry_exact_ring0_mode_count="$(get_value "$BASE_REPORT" "registry_exact_ring0_mode_count")"
registry_exact_scheduler_mode="$(get_value "$BASE_REPORT" "registry_exact_scheduler_mode")"
registry_exact_scheduler_mode_count="$(get_value "$BASE_REPORT" "registry_exact_scheduler_mode_count")"
registry_exact_runtime_mode="$(get_value "$BASE_REPORT" "registry_exact_runtime_mode")"
registry_exact_runtime_mode_count="$(get_value "$BASE_REPORT" "registry_exact_runtime_mode_count")"
registry_exact_other_mode="$(get_value "$BASE_REPORT" "registry_exact_other_mode")"
registry_exact_other_mode_count="$(get_value "$BASE_REPORT" "registry_exact_other_mode_count")"
registry_exact_get_factory_order_by_policy_mode_count="$(get_value "$BASE_REPORT" "registry_exact_get_factory_order_by_policy_mode_count")"

if [ "$registry_top_0_mode" != "nyash_rust::box_factory::registry::UnifiedBoxRegistry::rebuild_cache" ]; then
  echo "[perf-userbox-startup-executable-ret0-bucket-nyash-kernel-runtime-registry-rebuild-cache-ring0-exact-top-symbol-dominance] expected rebuild_cache to stay dominant" >&2
  exit 1
fi

cat <<EOF
output_contract=perf-userbox-startup-executable-ret0-bucket-nyash-kernel-runtime-registry-rebuild-cache-ring0-exact-top-symbol-dominance-v0
input_contract=perf-userbox-startup-executable-ret0-bucket-nyash-kernel-runtime-registry-rebuild-cache-get-factory-order-by-policy-exact-top-symbol-absence-v0
measurement_scope=exact_aot_startup_executable_ret0_bucket_nyash_kernel_runtime_registry_rebuild_cache_ring0_exact_top_symbol_dominance
primary_bucket_env_count=$primary_bucket_env_count
primary_bucket_once_count=$primary_bucket_once_count
primary_bucket_path_count=$primary_bucket_path_count
primary_bucket_minimal_main_count=$primary_bucket_minimal_main_count
primary_bucket_nyash_kernel_runtime_count=$primary_bucket_nyash_kernel_runtime_count
primary_bucket_kernel_count=$primary_bucket_kernel_count
primary_bucket_other_count=$primary_bucket_other_count
primary_bucket_missing_count=$primary_bucket_missing_count
primary_bucket_alloc_count=$primary_bucket_alloc_count
primary_bucket_ffi_count=$primary_bucket_ffi_count
primary_bucket_string_count=$primary_bucket_string_count
ret0_count_min=$ret0_count_min
ret0_count_max=$ret0_count_max
trial_summaries=$trial_summaries
registry_top_0_mode=$registry_top_0_mode
registry_top_0_mode_count=$registry_top_0_mode_count
registry_top_1_mode=$registry_top_1_mode
registry_top_1_mode_count=$registry_top_1_mode_count
registry_top_2_mode=$registry_top_2_mode
registry_top_2_mode_count=$registry_top_2_mode_count
registry_exact_rebuild_cache_mode=$registry_exact_rebuild_cache_mode
registry_exact_rebuild_cache_mode_count=$registry_exact_rebuild_cache_mode_count
registry_exact_register_many_mode=$registry_exact_register_many_mode
registry_exact_register_many_mode_count=$registry_exact_register_many_mode_count
registry_exact_create_default_registry_mode=$registry_exact_create_default_registry_mode
registry_exact_create_default_registry_mode_count=$registry_exact_create_default_registry_mode_count
registry_exact_ring0_mode=$registry_exact_ring0_mode
registry_exact_ring0_mode_count=$registry_exact_ring0_mode_count
registry_exact_scheduler_mode=$registry_exact_scheduler_mode
registry_exact_scheduler_mode_count=$registry_exact_scheduler_mode_count
registry_exact_runtime_mode=$registry_exact_runtime_mode
registry_exact_runtime_mode_count=$registry_exact_runtime_mode_count
registry_exact_other_mode=$registry_exact_other_mode
registry_exact_other_mode_count=$registry_exact_other_mode_count
registry_exact_get_factory_order_by_policy_mode_count=$registry_exact_get_factory_order_by_policy_mode_count
registry_focus_dominant_mode=rebuild_cache
registry_focus_dominant_mode_count=$registry_exact_rebuild_cache_mode_count
registry_focus_runner_up_mode=$registry_top_1_mode
registry_focus_runner_up_mode_count=$registry_top_1_mode_count
registry_focus_ring0_mode=$registry_exact_ring0_mode
registry_focus_ring0_mode_count=$registry_exact_ring0_mode_count
summary=ok
EOF

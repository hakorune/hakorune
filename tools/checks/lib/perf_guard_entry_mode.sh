#!/usr/bin/env bash
# perf_guard_entry_mode.sh - entry-mode helpers for phase21_5 regression guard

PERF_GUARD_LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${PERF_GUARD_LIB_DIR}/perf_guard_common.sh"

perf_guard_entry_mode_load_baseline() {
  local baseline_file="$1"
  baseline_entry_source="$(perf_guard_baseline_get_int_or_zero "$baseline_file" '.apps_entry_mode_source_total_ms')"
  baseline_entry_prebuilt="$(perf_guard_baseline_get_int_or_zero "$baseline_file" '.apps_entry_mode_prebuilt_total_ms')"
  baseline_entry_delta_abs="$(perf_guard_baseline_get_int_or_zero "$baseline_file" '.apps_entry_mode_delta_abs_ms')"
  baseline_entry_hotspot_name="$(perf_guard_baseline_get_str_or_default "$baseline_file" '.apps_entry_mode_hotspot_case' "")"
  baseline_entry_hotspot_abs="$(perf_guard_baseline_get_int_or_zero "$baseline_file" '.apps_entry_mode_hotspot_delta_abs_ms')"
}

perf_guard_entry_mode_run_once() {
  local entry_compare_script="$1"
  local vm_timeout="$2"
  local entry_mode_samples="$3"
  local warmup="$4"
  local repeat="$5"
  PERF_VM_TIMEOUT="$vm_timeout" \
  PERF_APPS_ENTRY_MODE_DELTA_SAMPLES="$entry_mode_samples" \
  bash "$entry_compare_script" "$warmup" "$repeat" vm
}

perf_guard_entry_mode_parse_current_json() {
  local output="$1"
  current_entry_source="$(perf_guard_json_get_int_or_zero "$output" '.source_total_ms')"
  current_entry_prebuilt="$(perf_guard_json_get_int_or_zero "$output" '.mir_shape_prebuilt_total_ms')"
  current_entry_delta_abs="$(perf_guard_json_get_int_or_zero "$output" '.delta_ms_abs')"
  current_entry_hotspot_name="$(perf_guard_json_get_str_or_default "$output" '.hotspot_case_delta.case' "n/a")"
  current_entry_hotspot_abs="$(perf_guard_json_get_int_or_zero "$output" '.hotspot_case_delta.delta_ms_abs')"

  [[ "$current_entry_source" =~ ^[0-9]+$ ]] && [[ "$current_entry_source" -gt 0 ]] \
    && [[ "$current_entry_prebuilt" =~ ^[0-9]+$ ]] && [[ "$current_entry_prebuilt" -gt 0 ]] \
    && [[ "$current_entry_delta_abs" =~ ^[0-9]+$ ]] && [[ "$current_entry_delta_abs" -ge 0 ]] \
    && [[ "$current_entry_hotspot_abs" =~ ^[0-9]+$ ]] && [[ "$current_entry_hotspot_abs" -ge 0 ]]
}

perf_guard_entry_mode_validate_baseline() {
  local tag="$1"
  perf_guard_assert_uint_ge "$tag" "apps_entry_mode_source_total_ms in baseline" "$baseline_entry_source" 0
  perf_guard_assert_uint_ge "$tag" "apps_entry_mode_prebuilt_total_ms in baseline" "$baseline_entry_prebuilt" 0
  perf_guard_assert_uint_ge "$tag" "apps_entry_mode_delta_abs_ms in baseline" "$baseline_entry_delta_abs" 0
  perf_guard_assert_uint_ge "$tag" "apps_entry_mode_hotspot_delta_abs_ms in baseline" "$baseline_entry_hotspot_abs" 0
}

perf_guard_entry_mode_collect_current() {
  local tag="$1"
  local entry_compare_script="$2"
  local vm_timeout="$3"
  local entry_mode_samples="$4"
  local warmup="$5"
  local repeat="$6"
  local entry_retries="$7"

  ENTRY_OUT=""
  current_entry_source=0
  current_entry_prebuilt=0
  current_entry_delta_abs=0
  current_entry_hotspot_name="n/a"
  current_entry_hotspot_abs=0

  if [[ "$baseline_entry_source" -gt 0 ]] || [[ "$baseline_entry_prebuilt" -gt 0 ]] || [[ "$baseline_entry_delta_abs" -gt 0 ]]; then
    if [[ ! -f "$entry_compare_script" ]]; then
      echo "[$tag] ERROR: entry-mode compare script missing: $entry_compare_script" >&2
      exit 1
    fi
    if ! perf_guard_retry_capture "$tag" "bench_apps_entry_mode_compare" "$entry_retries" ENTRY_OUT perf_guard_entry_mode_parse_current_json \
        perf_guard_entry_mode_run_once "$entry_compare_script" "$vm_timeout" "$entry_mode_samples" "$warmup" "$repeat"; then
      exit 1
    fi
  fi
}

perf_guard_entry_mode_verify_and_report() {
  local tag="$1"
  local entry_source_max_degrade_pct="$2"
  local entry_prebuilt_max_degrade_pct="$3"
  local entry_delta_min_ratio="$4"

  if [[ "$baseline_entry_source" -gt 0 ]] && [[ "$baseline_entry_prebuilt" -gt 0 ]] && [[ "$baseline_entry_delta_abs" -gt 0 ]]; then
    entry_source_degrade="$(perf_guard_calc_degrade_pct "$baseline_entry_source" "$current_entry_source")"
    entry_prebuilt_degrade="$(perf_guard_calc_degrade_pct "$baseline_entry_prebuilt" "$current_entry_prebuilt")"
    entry_delta_ratio="$(perf_guard_calc_ratio "$baseline_entry_delta_abs" "$current_entry_delta_abs")"
    echo "[$tag] baseline_entry_source_total_ms=${baseline_entry_source} current_entry_source_total_ms=${current_entry_source} degrade_pct=${entry_source_degrade} limit_pct=${entry_source_max_degrade_pct}"
    echo "[$tag] baseline_entry_prebuilt_total_ms=${baseline_entry_prebuilt} current_entry_prebuilt_total_ms=${current_entry_prebuilt} degrade_pct=${entry_prebuilt_degrade} limit_pct=${entry_prebuilt_max_degrade_pct}"
    echo "[$tag] baseline_entry_delta_abs_ms=${baseline_entry_delta_abs} current_entry_delta_abs_ms=${current_entry_delta_abs} ratio=${entry_delta_ratio} min_ratio=${entry_delta_min_ratio}"

    perf_guard_assert_max_pct "$tag" "entry-mode source regression exceeded threshold" "$entry_source_degrade" "$entry_source_max_degrade_pct"
    perf_guard_assert_max_pct "$tag" "entry-mode prebuilt regression exceeded threshold" "$entry_prebuilt_degrade" "$entry_prebuilt_max_degrade_pct"
    perf_guard_assert_min_ratio "$tag" "entry-mode delta advantage collapsed below min ratio" "$entry_delta_ratio" "$entry_delta_min_ratio"
  else
    echo "[$tag] entry_mode baseline missing; skip (reseed baseline to enable)"
  fi
}

perf_guard_entry_mode_print_hotspot() {
  local tag="$1"
  if [[ "$baseline_entry_delta_abs" -gt 0 ]]; then
    if [[ -z "$baseline_entry_hotspot_name" ]]; then
      baseline_entry_hotspot_name="n/a"
    fi
    echo "[$tag] entry_mode_hotspot baseline=${baseline_entry_hotspot_name}:${baseline_entry_hotspot_abs} current=${current_entry_hotspot_name}:${current_entry_hotspot_abs}"
  fi
}

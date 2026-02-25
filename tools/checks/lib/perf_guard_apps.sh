#!/usr/bin/env bash
# perf_guard_apps.sh - app wallclock helpers for phase21_5 regression guard

PERF_GUARD_LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${PERF_GUARD_LIB_DIR}/perf_guard_common.sh"

perf_guard_apps_load_baseline() {
  local baseline_file="$1"
  baseline_apps="$(perf_guard_baseline_get_int_or_zero "$baseline_file" '.apps_vm_total_ms')"
  baseline_app_count="$(perf_guard_baseline_get_int_or_zero "$baseline_file" '(.apps_vm_per_app_ms | keys | length)')"
}

perf_guard_apps_run_once() {
  local root_dir="$1"
  local vm_timeout="$2"
  local warmup="$3"
  local repeat="$4"
  PERF_VM_TIMEOUT="$vm_timeout" \
  PERF_APPS_OUTPUT=json \
  bash "$root_dir/tools/perf/bench_apps_wallclock.sh" "$warmup" "$repeat" vm
}

perf_guard_apps_parse_current_json() {
  local output="$1"
  current_apps="$(perf_guard_json_get_int_or_zero "$output" '.total_ms')"
  cases_count="$(perf_guard_json_get_int_or_zero "$output" '(.cases | keys | length)')"
  [[ "$cases_count" -ge 1 ]] && [[ "$current_apps" =~ ^[0-9]+$ ]] && [[ "$current_apps" -gt 0 ]]
}

perf_guard_apps_validate_baseline() {
  local tag="$1"
  perf_guard_assert_uint_ge "$tag" "apps_vm_total_ms in baseline" "$baseline_apps" 1
  if ! [[ "$baseline_app_count" =~ ^[0-9]+$ ]] || [[ "$baseline_app_count" -lt 1 ]]; then
    echo "[$tag] ERROR: invalid apps_vm_per_app_ms in baseline (missing or empty)" >&2
    echo "[$tag] hint: reseed baseline with:" >&2
    echo "[$tag]   PERF_STABILITY_INCLUDE_MEDIUM=1 PERF_STABILITY_INCLUDE_APPS=1 PERF_STABILITY_WRITE_BASELINE=1 tools/perf/record_baseline_stability_21_5.sh 2 1 1" >&2
    exit 1
  fi
}

perf_guard_apps_collect_current() {
  local tag="$1"
  local root_dir="$2"
  local vm_timeout="$3"
  local warmup="$4"
  local repeat="$5"
  local apps_retries="$6"

  APPS_OUT=""
  current_apps=0
  cases_count=0
  if ! perf_guard_retry_capture "$tag" "bench_apps_wallclock" "$apps_retries" APPS_OUT perf_guard_apps_parse_current_json \
      perf_guard_apps_run_once "$root_dir" "$vm_timeout" "$warmup" "$repeat"; then
    exit 1
  fi
}

perf_guard_apps_verify_totals() {
  local tag="$1"
  local apps_max_degrade_pct="$2"
  apps_degrade="$(perf_guard_calc_degrade_pct "$baseline_apps" "$current_apps")"
  echo "[$tag] baseline_apps_total_ms=${baseline_apps} current_apps_total_ms=${current_apps} degrade_pct=${apps_degrade} limit_pct=${apps_max_degrade_pct}"
  perf_guard_assert_max_pct "$tag" "app wallclock regression exceeded threshold" "$apps_degrade" "$apps_max_degrade_pct"
}

perf_guard_apps_verify_per_app_and_hotspot() {
  local tag="$1"
  local baseline_file="$2"
  local per_app_max_degrade_pct="$3"

  if [[ "$cases_count" -lt "$baseline_app_count" ]]; then
    echo "[$tag] ERROR: app case count shrunk (baseline=${baseline_app_count}, current=${cases_count})" >&2
    exit 1
  fi

  while IFS=$'\t' read -r app_name baseline_app_ms; do
    [[ -z "$app_name" ]] && continue
    if ! [[ "$baseline_app_ms" =~ ^[0-9]+$ ]] || [[ "$baseline_app_ms" -lt 1 ]]; then
      echo "[$tag] ERROR: invalid baseline app metric: name=${app_name} ms=${baseline_app_ms}" >&2
      exit 1
    fi
    current_app_ms="$(perf_guard_json_get_int_or_zero "$APPS_OUT" ".cases[\"${app_name}\"]")"
    if ! [[ "$current_app_ms" =~ ^[0-9]+$ ]] || [[ "$current_app_ms" -lt 1 ]]; then
      echo "[$tag] ERROR: missing/invalid current app metric: name=${app_name} ms=${current_app_ms}" >&2
      echo "$APPS_OUT" >&2
      exit 1
    fi
    app_degrade="$(perf_guard_calc_degrade_pct "$baseline_app_ms" "$current_app_ms")"
    echo "[$tag] baseline_app_ms[$app_name]=${baseline_app_ms} current_app_ms[$app_name]=${current_app_ms} degrade_pct=${app_degrade} limit_pct=${per_app_max_degrade_pct}"
    perf_guard_assert_max_pct "$tag" "app regression exceeded threshold for ${app_name}" "$app_degrade" "$per_app_max_degrade_pct"
  done < <(jq -r '.apps_vm_per_app_ms // {} | to_entries[] | "\(.key)\t\(.value)"' "$baseline_file")

  baseline_hotspot_name="$(perf_guard_baseline_get_str_or_default "$baseline_file" '.apps_vm_per_app_ms | to_entries | max_by(.value) | .key' "n/a")"
  baseline_hotspot_ms="$(perf_guard_baseline_get_int_or_zero "$baseline_file" '.apps_vm_per_app_ms | to_entries | max_by(.value) | .value')"
  current_hotspot_name="$(perf_guard_json_get_str_or_default "$APPS_OUT" '.cases | to_entries | max_by(.value) | .key' "n/a")"
  current_hotspot_ms="$(perf_guard_json_get_int_or_zero "$APPS_OUT" '.cases | to_entries | max_by(.value) | .value')"
  echo "[$tag] apps_hotspot baseline=${baseline_hotspot_name}:${baseline_hotspot_ms} current=${current_hotspot_name}:${current_hotspot_ms}"
}

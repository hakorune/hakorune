#!/usr/bin/env bash
# perf_guard_common.sh - shared math/assert helpers for perf regression guards

perf_guard_calc_degrade_pct() {
  local base="$1"
  local cur="$2"
  awk -v b="$base" -v c="$cur" 'BEGIN { d=((c-b)*100.0)/b; if (d < 0) d = 0; printf "%.2f", d }'
}

perf_guard_calc_ratio() {
  local base="$1"
  local cur="$2"
  awk -v b="$base" -v c="$cur" 'BEGIN { if (b <= 0) { printf "1.0000"; exit } printf "%.4f", c/b }'
}

perf_guard_assert_max_pct() {
  local tag="$1"
  local error_label="$2"
  local degrade_pct="$3"
  local max_pct="$4"
  if awk -v d="$degrade_pct" -v m="$max_pct" 'BEGIN { exit !(d > m) }'; then
    echo "[$tag] ERROR: ${error_label}" >&2
    exit 1
  fi
}

perf_guard_assert_min_ratio() {
  local tag="$1"
  local error_label="$2"
  local ratio="$3"
  local min_ratio="$4"
  if awk -v r="$ratio" -v m="$min_ratio" 'BEGIN { exit !(r < m) }'; then
    echo "[$tag] ERROR: ${error_label}" >&2
    exit 1
  fi
}

perf_guard_assert_uint_ge() {
  local tag="$1"
  local field="$2"
  local value="$3"
  local min_value="$4"
  if ! [[ "$value" =~ ^[0-9]+$ ]] || [[ "$value" -lt "$min_value" ]]; then
    echo "[$tag] ERROR: invalid ${field}: ${value} (expected uint >= ${min_value})" >&2
    exit 1
  fi
}

perf_guard_json_get_int_or_zero() {
  local json_text="$1"
  local jq_expr="$2"
  local value
  value="$(printf '%s\n' "$json_text" | jq -r "$jq_expr // 0" 2>/dev/null || echo 0)"
  if [[ "$value" =~ ^-?[0-9]+$ ]]; then
    printf '%s\n' "$value"
  else
    printf '0\n'
  fi
}

perf_guard_json_get_str_or_default() {
  local json_text="$1"
  local jq_expr="$2"
  local default_value="$3"
  local value
  value="$(printf '%s\n' "$json_text" | jq -r "$jq_expr // \"${default_value}\"" 2>/dev/null || echo "$default_value")"
  if [[ -z "$value" ]]; then
    printf '%s\n' "$default_value"
  else
    printf '%s\n' "$value"
  fi
}

perf_guard_baseline_get_int_or_zero() {
  local baseline_file="$1"
  local jq_expr="$2"
  local value
  value="$(jq -r "$jq_expr // 0" "$baseline_file" 2>/dev/null || echo 0)"
  if [[ "$value" =~ ^-?[0-9]+$ ]]; then
    printf '%s\n' "$value"
  else
    printf '0\n'
  fi
}

perf_guard_baseline_get_str_or_default() {
  local baseline_file="$1"
  local jq_expr="$2"
  local default_value="$3"
  local value
  value="$(jq -r "$jq_expr // \"${default_value}\"" "$baseline_file" 2>/dev/null || echo "$default_value")"
  if [[ -z "$value" ]]; then
    printf '%s\n' "$default_value"
  else
    printf '%s\n' "$value"
  fi
}

perf_guard_retry_capture() {
  local tag="$1"
  local label="$2"
  local retries="$3"
  local out_var="$4"
  local parse_fn="$5"
  shift 5

  if ! [[ "$retries" =~ ^[0-9]+$ ]] || [[ "$retries" -lt 1 ]]; then
    echo "[$tag] ERROR: invalid retries for ${label}: ${retries} (expected uint >= 1)" >&2
    return 2
  fi

  if [[ -n "$parse_fn" ]] && ! declare -F "$parse_fn" >/dev/null 2>&1; then
    echo "[$tag] ERROR: parse fn not found: ${parse_fn}" >&2
    return 2
  fi

  local try=1
  local output=""
  while [[ "$try" -le "$retries" ]]; do
    output="$("$@" 2>&1)" || {
      if [[ "$try" -eq "$retries" ]]; then
        echo "[$tag] ERROR: ${label} failed after ${retries} attempts" >&2
        echo "$output" >&2
        return 1
      fi
      try=$((try + 1))
      continue
    }

    printf -v "$out_var" '%s' "$output"

    if [[ -z "$parse_fn" ]] || "$parse_fn" "$output"; then
      return 0
    fi

    if [[ "$try" -eq "$retries" ]]; then
      echo "[$tag] ERROR: could not parse ${label} output after ${retries} attempts" >&2
      echo "$output" >&2
      return 1
    fi
    try=$((try + 1))
  done

  return 1
}

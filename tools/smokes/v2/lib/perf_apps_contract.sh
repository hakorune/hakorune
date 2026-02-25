#!/bin/bash
# perf_apps_contract.sh
# Shared validation helpers for Phase 21.5 app-wallclock JSON contracts.

perf_apps_require_json() {
  local smoke_name="$1"
  local out="$2"
  if ! printf '%s\n' "$out" | jq -e . >/dev/null 2>&1; then
    echo "$out"
    test_fail "$smoke_name: output is not valid JSON"
    return 1
  fi
}

perf_apps_json_get() {
  local out="$1"
  local expr="$2"
  printf '%s\n' "$out" | jq -r "$expr"
}

perf_apps_assert_uint() {
  local smoke_name="$1"
  local label="$2"
  local value="$3"
  local out="$4"
  if ! [[ "$value" =~ ^[0-9]+$ ]]; then
    echo "$out"
    test_fail "$smoke_name: non-numeric ${label}: ${value}"
    return 1
  fi
}

perf_apps_assert_positive_uint() {
  local smoke_name="$1"
  local label="$2"
  local value="$3"
  local out="$4"
  perf_apps_assert_uint "$smoke_name" "$label" "$value" "$out" || return 1
  if [ "$value" -le 0 ]; then
    echo "$out"
    test_fail "$smoke_name: invalid ${label}: ${value}"
    return 1
  fi
}

perf_apps_assert_backend_vm() {
  local smoke_name="$1"
  local backend="$2"
  local out="$3"
  if [ "$backend" != "vm" ]; then
    echo "$out"
    test_fail "$smoke_name: backend must be vm (got: $backend)"
    return 1
  fi
}

perf_apps_assert_case_name() {
  local smoke_name="$1"
  local label="$2"
  local case_name="$3"
  local out="$4"
  perf_apps_assert_case_name_in_object "$smoke_name" "$label" "$case_name" "$out" '.cases // {}'
}

perf_apps_assert_case_name_in_object() {
  local smoke_name="$1"
  local label="$2"
  local case_name="$3"
  local out="$4"
  local object_expr="$5"
  if ! printf '%s\n' "$out" | jq -e --arg k "$case_name" "${object_expr} | has(\$k)" >/dev/null 2>&1; then
    echo "$out"
    test_fail "$smoke_name: invalid ${label}: ${case_name}"
    return 1
  fi
}

perf_apps_assert_eq_uint() {
  local smoke_name="$1"
  local label="$2"
  local lhs="$3"
  local rhs="$4"
  local out="$5"
  perf_apps_assert_uint "$smoke_name" "$label(lhs)" "$lhs" "$out" || return 1
  perf_apps_assert_uint "$smoke_name" "$label(rhs)" "$rhs" "$out" || return 1
  if [ "$lhs" -ne "$rhs" ]; then
    echo "$out"
    test_fail "$smoke_name: ${label} mismatch (${lhs} != ${rhs})"
    return 1
  fi
}

perf_apps_assert_le_uint() {
  local smoke_name="$1"
  local label="$2"
  local lhs="$3"
  local rhs="$4"
  local out="$5"
  perf_apps_assert_uint "$smoke_name" "$label(lhs)" "$lhs" "$out" || return 1
  perf_apps_assert_uint "$smoke_name" "$label(rhs)" "$rhs" "$out" || return 1
  if [ "$lhs" -gt "$rhs" ]; then
    echo "$out"
    test_fail "$smoke_name: ${label} violated (${lhs} > ${rhs})"
    return 1
  fi
}

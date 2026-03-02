#!/bin/bash
# perf_crosslang_contract.sh
# Shared validation helpers for APP-PERF cross-language benchmark smokes.

perf_crosslang_require_inputs() {
  local smoke_name="$1"
  local script_path="$2"
  local key="$3"
  local root="${NYASH_ROOT:-}"
  local dataset_key="${key}"

  if [[ -z "${root}" ]]; then
    test_fail "${smoke_name}: NYASH_ROOT is not set"
    return 1
  fi

  if [[ ! -x "${script_path}" ]]; then
    test_fail "${smoke_name}: Script not executable: ${script_path}"
    return 1
  fi
  if [[ -f "${root}/tools/perf/lib/bench_key_alias.sh" ]]; then
    # shellcheck source=tools/perf/lib/bench_key_alias.sh
    source "${root}/tools/perf/lib/bench_key_alias.sh"
    dataset_key="$(perf_resolve_bench_dataset_key "${key}")"
  fi
  if [[ ! -f "${root}/benchmarks/bench_${dataset_key}.hako" ]]; then
    test_fail "${smoke_name}: Hako benchmark not found"
    return 1
  fi
  if [[ ! -f "${root}/benchmarks/c/bench_${dataset_key}.c" ]]; then
    test_fail "${smoke_name}: C benchmark not found"
    return 1
  fi
  if [[ ! -f "${root}/benchmarks/python/bench_${dataset_key}.py" ]]; then
    test_fail "${smoke_name}: Python benchmark not found"
    return 1
  fi
}

perf_crosslang_assert_output() {
  local smoke_name="$1"
  local key="$2"
  local output="$3"

  if ! printf '%s\n' "${output}" | grep -q "\[bench4\] name=${key}"; then
    printf '%s\n' "${output}" | tail -n 40 || true
    test_fail "${smoke_name}: Missing [bench4] name=${key} marker"
    return 1
  fi

  if ! printf '%s\n' "${output}" | grep -q "aot_status=ok"; then
    printf '%s\n' "${output}" | tail -n 40 || true
    test_fail "${smoke_name}: aot_status is not 'ok'"
    return 1
  fi

  local metric_key
  for metric_key in c_ms py_ms ny_vm_ms ny_aot_ms; do
    if ! printf '%s\n' "${output}" | grep -qE "${metric_key}=[0-9]+"; then
      printf '%s\n' "${output}" | tail -n 40 || true
      test_fail "${smoke_name}: Missing timing key: ${metric_key}="
      return 1
    fi
  done

  for metric_key in ratio_c_vm ratio_c_py ratio_c_aot; do
    if ! printf '%s\n' "${output}" | grep -qE "${metric_key}=[0-9.]+"; then
      printf '%s\n' "${output}" | tail -n 40 || true
      test_fail "${smoke_name}: Missing ratio key: ${metric_key}="
      return 1
    fi
  done

  if ! printf '%s\n' "${output}" | grep -q "\[bench4-route\] name=${key} "; then
    printf '%s\n' "${output}" | tail -n 40 || true
    test_fail "${smoke_name}: Missing [bench4-route] marker for ${key}"
    return 1
  fi
  for route_key in kernel_lane fallback_guard vm_engine vm_lane route_probe result_parity; do
    if ! printf '%s\n' "${output}" | grep -qE "${route_key}=[^[:space:]]+"; then
      printf '%s\n' "${output}" | tail -n 40 || true
      test_fail "${smoke_name}: Missing route key: ${route_key}="
      return 1
    fi
  done

  if [[ "${key}" == *_hk ]]; then
    if ! printf '%s\n' "${output}" | grep -q 'kernel_lane=hk'; then
      printf '%s\n' "${output}" | tail -n 40 || true
      test_fail "${smoke_name}: hk key must report kernel_lane=hk"
      return 1
    fi
    if ! printf '%s\n' "${output}" | grep -q 'fallback_guard=strict-no-fallback'; then
      printf '%s\n' "${output}" | tail -n 40 || true
      test_fail "${smoke_name}: hk key must report fallback_guard=strict-no-fallback"
      return 1
    fi
    if ! printf '%s\n' "${output}" | grep -q 'result_parity=ok'; then
      printf '%s\n' "${output}" | tail -n 40 || true
      test_fail "${smoke_name}: hk key must report result_parity=ok"
      return 1
    fi
  fi
}

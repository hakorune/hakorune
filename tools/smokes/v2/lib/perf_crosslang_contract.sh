#!/bin/bash
# perf_crosslang_contract.sh
# Shared validation helpers for APP-PERF cross-language benchmark smokes.

perf_crosslang_require_inputs() {
  local smoke_name="$1"
  local script_path="$2"
  local key="$3"
  local root="${NYASH_ROOT:-}"

  if [[ -z "${root}" ]]; then
    test_fail "${smoke_name}: NYASH_ROOT is not set"
    return 1
  fi

  if [[ ! -x "${script_path}" ]]; then
    test_fail "${smoke_name}: Script not executable: ${script_path}"
    return 1
  fi
  if [[ ! -f "${root}/benchmarks/bench_${key}.hako" ]]; then
    test_fail "${smoke_name}: Hako benchmark not found"
    return 1
  fi
  if [[ ! -f "${root}/benchmarks/c/bench_${key}.c" ]]; then
    test_fail "${smoke_name}: C benchmark not found"
    return 1
  fi
  if [[ ! -f "${root}/benchmarks/python/bench_${key}.py" ]]; then
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
}

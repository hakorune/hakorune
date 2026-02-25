#!/usr/bin/env bash

# Shared helpers for bench_compare scripts.
# Keep timing/preflight behavior consistent across bench lanes.

perf_time_ms() {
  date +%s%3N
}

perf_measure_cmd_ms() {
  local cmd=("$@")
  local t1 t2 dt
  t1=$(perf_time_ms)
  "${cmd[@]}" >/dev/null 2>&1 || true
  t2=$(perf_time_ms)
  dt=$((t2 - t1))
  echo "$dt"
}

perf_median_ms() {
  awk 'NF{print $1}' | sort -n | awk ' { a[NR]=$1 } END { if (NR==0) {print 0; exit} n=int((NR+1)/2); print a[n] }'
}

perf_collect_series() {
  local warmup=$1
  shift
  local repeat=$1
  shift
  local -a cmd=("$@")
  for _ in $(seq 1 "${warmup}"); do
    perf_measure_cmd_ms "${cmd[@]}" >/dev/null || true
  done
  for _ in $(seq 1 "${repeat}"); do
    perf_measure_cmd_ms "${cmd[@]}"
  done
}

perf_print_vm_fail_hints() {
  local key="$1"
  local vm_timeout="$2"
  local script_name="$3"
  local warmup="$4"
  local repeat="$5"
  local rc="$6"
  local stderr_log="$7"

  if [[ "${stderr_log}" == *"vm step budget exceeded"* ]]; then
    echo "[hint] VM step budget exceeded for benchmark '${key}'." >&2
    echo "[hint] Try raising HAKO_VM_MAX_STEPS (example):" >&2
    echo "[hint]   HAKO_VM_MAX_STEPS=200000000 bash ${script_name} ${key} ${warmup} ${repeat}" >&2
    echo "[hint] For diagnostics only: HAKO_VM_MAX_STEPS=0 (unlimited)." >&2
    return
  fi
  if [[ "${rc}" -eq 124 ]]; then
    echo "[hint] VM benchmark timed out (${vm_timeout})." >&2
    echo "[hint] This can be caused by step budget or very slow execution." >&2
    echo "[hint] Consider raising HAKO_VM_MAX_STEPS or lowering benchmark size for diagnosis." >&2
  fi
}

perf_run_vm_preflight_or_fail() {
  local key="$1"
  local vm_timeout="$2"
  local script_name="$3"
  local warmup="$4"
  local repeat="$5"
  shift 5
  local -a cmd=("$@")

  local rc=0
  local vm_err=""
  set +e
  vm_err=$("${cmd[@]}" 2>&1 >/dev/null)
  rc=$?
  set -e

  local hard_fail=0
  if [[ "${rc}" -eq 124 ]]; then
    hard_fail=1
  fi
  if [[ "${vm_err}" == *"vm step budget exceeded"* || "${vm_err}" == *"[ERROR]"* ]]; then
    hard_fail=1
  fi
  # Bench programs may intentionally return non-zero numeric values.
  # Treat as success unless timeout/VM error markers are observed.
  if [[ "${rc}" -ne 0 && "${vm_err}" != *"RC:"* && "${hard_fail}" -eq 0 ]]; then
    hard_fail=1
  fi
  if [[ "${hard_fail}" -eq 1 ]]; then
    echo "[error] VM benchmark preflight failed: key=${key} rc=${rc}" >&2
    perf_print_vm_fail_hints "${key}" "${vm_timeout}" "${script_name}" "${warmup}" "${repeat}" "${rc}" "${vm_err}"
    if [[ -n "${vm_err}" ]]; then
      echo "[error] VM stderr (first lines):" >&2
      printf "%s\n" "${vm_err}" | sed -n '1,8p' >&2
    fi
    return 1
  fi
  return 0
}

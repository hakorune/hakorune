#!/usr/bin/env bash

# Shared helpers for PERF_AOT flows.
# Keep behavior consistent between bench_compare and baseline recorder.

PERF_AOT_LAST_STATUS="skip"
PERF_AOT_LAST_REASON="not_attempted"
PERF_AOT_LAST_STAGE="none"
PERF_AOT_LAST_EMIT_ROUTE="none"
PERF_AOT_LAST_MED_MS=0

# Perf lane policy for AOT series/probe:
# run with deterministic runtime knobs (GC off + poll off) by default.
# Callers can override via env exports.
perf_aot_runtime_env_cmd() {
  env \
    NYASH_GC_MODE="${NYASH_GC_MODE:-off}" \
    NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}" \
    "$@"
}

perf_aot_set_status() {
  local status=$1
  local reason=$2
  local stage=${3:-none}
  PERF_AOT_LAST_STATUS="$status"
  PERF_AOT_LAST_REASON="$reason"
  PERF_AOT_LAST_STAGE="$stage"
}

perf_aot_reset_status() {
  perf_aot_set_status "skip" "not_attempted" "none"
  PERF_AOT_LAST_EMIT_ROUTE="none"
  PERF_AOT_LAST_MED_MS=0
}

perf_aot_resolve_skip_build() {
  local root_dir=$1
  local requested="${PERF_AOT_SKIP_BUILD:-auto}"
  case "${requested}" in
    0|1)
      printf '%s\n' "${requested}"
      return 0
      ;;
    auto)
      ;;
    *)
      echo "[error] PERF_AOT_SKIP_BUILD must be auto|0|1: got '${requested}'" >&2
      return 2
      ;;
  esac

  local bin_hakorune="${root_dir}/target/release/hakorune"
  local bin_llvmc="${root_dir}/target/release/ny-llvmc"
  local lib_kernel="${root_dir}/target/release/libnyash_kernel.a"
  if [[ -x "${bin_hakorune}" && -x "${bin_llvmc}" && -f "${lib_kernel}" ]]; then
    printf '1\n'
  else
    printf '0\n'
  fi
}

perf_aot_resolve_bool_01() {
  local name="$1"
  local value="$2"
  case "${value}" in
    0|1)
      printf '%s\n' "${value}"
      return 0
      ;;
    *)
      echo "[error] ${name} must be 0|1: got '${value}'" >&2
      return 1
      ;;
  esac
}

perf_aot_should_retry_helper_after_build_fail() {
  local root_dir=$1
  local skip_build
  # Retry helper only when build artifacts are expected to be already present.
  # When skip_build=0 (full rebuild lane), retrying doubles expensive rebuild work
  # and does not help with timeout/toolchain failures.
  if ! skip_build="$(perf_aot_resolve_skip_build "${root_dir}")"; then
    return 1
  fi
  [[ "${skip_build}" == "1" ]]
}

perf_emit_mir_json_helper() {
  local root_dir=$1
  local hako_prog=$2
  local out_json=$3
  local emit_route="${root_dir}/tools/smokes/v2/lib/emit_mir_route.sh"
  if [[ ! -x "${emit_route}" ]]; then
    PERF_AOT_LAST_EMIT_ROUTE="none"
    perf_aot_set_status "skip" "emit_route_helper_missing" "emit"
    return 1
  fi
  if "${emit_route}" --route hako-helper --timeout-secs "${PERF_AOT_EMIT_TIMEOUT_SECS:-60}" --out "${out_json}" --input "${hako_prog}" >/dev/null 2>&1; then
    PERF_AOT_LAST_EMIT_ROUTE="helper"
    return 0
  fi
  PERF_AOT_LAST_EMIT_ROUTE="none"
  return 1
}

perf_emit_mir_json() {
  local root_dir=$1
  local hako_bin=$2
  local hako_prog=$3
  local out_json=$4
  local direct_tried=0
  local prefer_helper="${PERF_AOT_PREFER_HELPER:-0}"
  local helper_only="${PERF_AOT_HELPER_ONLY:-0}"
  local direct_only

  direct_only="$(perf_aot_resolve_bool_01 "PERF_AOT_DIRECT_ONLY" "${PERF_AOT_DIRECT_ONLY:-0}")" || return 1

  if [[ "${prefer_helper}" == "1" ]]; then
    if perf_emit_mir_json_helper "${root_dir}" "${hako_prog}" "${out_json}"; then
      return 0
    fi
    if [[ "${helper_only}" == "1" ]]; then
      PERF_AOT_LAST_EMIT_ROUTE="none"
      perf_aot_set_status "skip" "emit_helper_only_failed" "emit"
      return 1
    fi
  fi

  if [[ -n "${hako_bin}" && -x "${hako_bin}" ]]; then
    direct_tried=1
    if "${hako_bin}" --emit-mir-json "${out_json}" "${hako_prog}" >/dev/null 2>&1; then
      PERF_AOT_LAST_EMIT_ROUTE="direct"
      return 0
    fi
    if [[ "${direct_only}" == "1" ]]; then
      PERF_AOT_LAST_EMIT_ROUTE="none"
      perf_aot_set_status "skip" "emit_direct_only_failed" "emit"
      return 1
    fi
  elif [[ "${direct_only}" == "1" ]]; then
    PERF_AOT_LAST_EMIT_ROUTE="none"
    perf_aot_set_status "skip" "emit_direct_binary_missing" "emit"
    return 1
  fi
  if perf_emit_mir_json_helper "${root_dir}" "${hako_prog}" "${out_json}"; then
    if [[ "${direct_only}" == "1" ]]; then
      PERF_AOT_LAST_EMIT_ROUTE="none"
      perf_aot_set_status "skip" "emit_direct_only_rejected_helper_route" "emit"
      return 1
    fi
    return 0
  fi
  PERF_AOT_LAST_EMIT_ROUTE="none"
  if [[ "$direct_tried" -eq 1 ]]; then
    perf_aot_set_status "skip" "emit_direct_and_helper_failed" "emit"
  else
    perf_aot_set_status "skip" "emit_helper_failed" "emit"
  fi
  return 1
}

perf_build_aot_exe() {
  local root_dir=$1
  local in_json=$2
  local out_exe=$3
  local skip_build

  if ! skip_build="$(perf_aot_resolve_skip_build "${root_dir}")"; then
    perf_aot_set_status "skip" "invalid_skip_build_env" "contract"
    return 1
  fi

  if ! NYASH_LLVM_SKIP_BUILD="${skip_build}" \
      NYASH_LLVM_FAST=1 \
      NYASH_LLVM_FAST_INT="${NYASH_LLVM_FAST_INT:-1}" \
      NYASH_GC_MODE="${NYASH_GC_MODE:-off}" \
      NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}" \
      bash "${root_dir}/tools/ny_mir_builder.sh" \
      --in "${in_json}" \
      --emit exe \
      -o "${out_exe}" \
      --quiet >/dev/null 2>&1; then
    perf_aot_set_status "skip" "build_failed" "build"
    return 1
  fi
  return 0
}

perf_emit_and_build_aot_exe() {
  local root_dir=$1
  local hako_bin=$2
  local hako_prog=$3
  local out_exe=$4
  local tmp_json

  perf_aot_reset_status
  tmp_json=$(mktemp --suffix .json)
  if ! perf_emit_mir_json "${root_dir}" "${hako_bin}" "${hako_prog}" "${tmp_json}"; then
    rm -f "${tmp_json}" || true
    return 1
  fi
  if ! perf_build_aot_exe "${root_dir}" "${tmp_json}" "${out_exe}"; then
    # Some fixtures still produce invalid MIR in direct emit route; retry once via helper route.
    if [[ "${PERF_AOT_LAST_EMIT_ROUTE}" == "direct" ]] \
      && perf_aot_should_retry_helper_after_build_fail "${root_dir}"; then
      if ! perf_emit_mir_json_helper "${root_dir}" "${hako_prog}" "${tmp_json}"; then
        perf_aot_set_status "skip" "emit_helper_retry_failed" "emit_retry"
        rm -f "${tmp_json}" || true
        return 1
      fi
      if ! perf_build_aot_exe "${root_dir}" "${tmp_json}" "${out_exe}"; then
        perf_aot_set_status "skip" "build_failed_after_helper_retry" "build_retry"
        rm -f "${tmp_json}" || true
        return 1
      fi
      rm -f "${tmp_json}" || true
      perf_aot_set_status "ok" "ok_retry_helper" "done"
      return 0
    fi
    rm -f "${tmp_json}" || true
    return 1
  fi
  rm -f "${tmp_json}" || true
  perf_aot_set_status "ok" "ok" "done"
  return 0
}

perf_build_ret0_aot_exe() {
  local root_dir=$1
  local hako_bin=$2
  local out_exe=$3
  local tmp_ret0_hako tmp_ret0_json
  tmp_ret0_hako=$(mktemp --suffix .hako)
  cat >"${tmp_ret0_hako}" <<'HAKO'
static box Main { main() { return 0 } }
HAKO
  tmp_ret0_json=$(mktemp --suffix .json)
  if perf_emit_mir_json "${root_dir}" "${hako_bin}" "${tmp_ret0_hako}" "${tmp_ret0_json}" \
    && perf_build_aot_exe "${root_dir}" "${tmp_ret0_json}" "${out_exe}"; then
    rm -f "${tmp_ret0_hako}" "${tmp_ret0_json}" || true
    return 0
  fi
  rm -f "${tmp_ret0_hako}" "${tmp_ret0_json}" || true
  return 1
}

perf_probe_aot_exe() {
  local exe_path=$1
  local timeout_sec=${2:-20}
  local out_log err_log rc
  out_log=$(mktemp --suffix .aot_probe.out)
  err_log=$(mktemp --suffix .aot_probe.err)

  set +e
  perf_aot_runtime_env_cmd timeout "${timeout_sec}s" "${exe_path}" >"${out_log}" 2>"${err_log}"
  rc=$?
  set -e

  if [[ "$rc" -eq 124 ]]; then
    perf_aot_set_status "fail" "exe_runtime_timeout" "run"
    rm -f "${out_log}" "${err_log}" || true
    return 1
  fi

  if grep -Eq '\[nyrt_error\]|Unknown Box type' "${err_log}"; then
    perf_aot_set_status "fail" "exe_runtime_nyrt_error" "run"
    rm -f "${out_log}" "${err_log}" || true
    return 1
  fi

  rm -f "${out_log}" "${err_log}" || true
  return 0
}

perf_aot_require_series_helpers() {
  if command -v perf_collect_series >/dev/null 2>&1 && command -v perf_median_ms >/dev/null 2>&1; then
    return 0
  fi
  if command -v collect_series >/dev/null 2>&1 && command -v median_ms >/dev/null 2>&1; then
    return 0
  fi
  perf_aot_set_status "skip" "series_helper_missing" "contract"
  return 1
}

perf_aot_collect_series() {
  local warmup=$1
  local repeat=$2
  shift 2
  if command -v perf_collect_series >/dev/null 2>&1; then
    perf_collect_series "${warmup}" "${repeat}" "$@"
    return $?
  fi
  if command -v collect_series >/dev/null 2>&1; then
    collect_series "${warmup}" "${repeat}" "$@"
    return $?
  fi
  return 1
}

perf_aot_median_ms() {
  if command -v perf_median_ms >/dev/null 2>&1; then
    perf_median_ms
    return $?
  fi
  if command -v median_ms >/dev/null 2>&1; then
    median_ms
    return $?
  fi
  return 1
}

perf_measure_aot_exe_series() {
  local exe_path=$1
  local warmup=$2
  local repeat=$3
  local timeout_sec=${4:-20}
  local series

  PERF_AOT_LAST_MED_MS=0
  if ! perf_aot_require_series_helpers; then
    return 1
  fi
  if ! perf_probe_aot_exe "${exe_path}" "${timeout_sec}"; then
    return 1
  fi

  series=$(perf_aot_collect_series "${warmup}" "${repeat}" env \
    NYASH_GC_MODE="${NYASH_GC_MODE:-off}" \
    NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}" \
    timeout "${timeout_sec}s" "${exe_path}")
  PERF_AOT_LAST_MED_MS=$(printf "%s\n" "${series}" | perf_aot_median_ms)
  return 0
}

perf_run_aot_bench_series() {
  local root_dir=$1
  local hako_bin=$2
  local hako_prog=$3
  local out_exe=$4
  local warmup=$5
  local repeat=$6
  local timeout_sec=${7:-20}

  PERF_AOT_LAST_MED_MS=0
  if ! perf_emit_and_build_aot_exe "${root_dir}" "${hako_bin}" "${hako_prog}" "${out_exe}"; then
    return 1
  fi
  if ! perf_measure_aot_exe_series "${out_exe}" "${warmup}" "${repeat}" "${timeout_sec}"; then
    return 1
  fi
  perf_aot_set_status "ok" "ok" "run"
  return 0
}

perf_run_ret0_aot_series() {
  local root_dir=$1
  local hako_bin=$2
  local out_exe=$3
  local warmup=${4:-1}
  local repeat=${5:-3}
  local timeout_sec=${6:-20}

  PERF_AOT_LAST_MED_MS=0
  if ! perf_build_ret0_aot_exe "${root_dir}" "${hako_bin}" "${out_exe}"; then
    return 1
  fi
  if ! perf_measure_aot_exe_series "${out_exe}" "${warmup}" "${repeat}" "${timeout_sec}"; then
    return 1
  fi
  perf_aot_set_status "ok" "ok" "run"
  return 0
}

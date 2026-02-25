#!/usr/bin/env bash

# Shared environment for VM benchmark runs.
# Keep this minimal and deterministic to reduce startup noise.
NYASH_VM_BENCH_ENV=(
  NYASH_FEATURES=stage3
  NYASH_PARSER_ALLOW_SEMICOLON=1
  NYASH_DISABLE_PLUGINS=1
  NYASH_SKIP_TOML_ENV=1
  NYASH_USE_NY_COMPILER=0
  NYASH_ENABLE_USING=0
  # Keep perf runs independent from smoke/dev JoinIR toggles.
  NYASH_JOINIR_DEV=0
  HAKO_JOINIR_STRICT=0
  NYASH_VM_FAST=1
  # VM hot-path dense register file (opt-out with NYASH_VM_FAST_REGFILE=0)
  NYASH_VM_FAST_REGFILE=${NYASH_VM_FAST_REGFILE:-1}
  # Perf-only policy: benchmarks in this lane are sync kernels, so avoid
  # scheduler poll overhead on safepoint bridge by default.
  # Callers can override with NYASH_SCHED_POLL_IN_SAFEPOINT=1.
  NYASH_SCHED_POLL_IN_SAFEPOINT=${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}
  NYASH_STR_CP=0
  # Can be overridden from caller shell:
  #   HAKO_VM_MAX_STEPS=200000000 tools/perf/bench_compare_c_vs_hako.sh ...
  HAKO_VM_MAX_STEPS=${HAKO_VM_MAX_STEPS:-100000000}
)

# VM timeout defaults used by perf scripts.
# - fast: single-bench scripts / quick probes
# - heavy: app/stability/regression-style scripts
PERF_VM_TIMEOUT_DEFAULT_FAST="${PERF_VM_TIMEOUT_DEFAULT_FAST:-20s}"
PERF_VM_TIMEOUT_DEFAULT_HEAVY="${PERF_VM_TIMEOUT_DEFAULT_HEAVY:-60s}"

perf_vm_timeout_resolve() {
  local profile="${1:-fast}"
  local default_value=""
  case "$profile" in
    fast) default_value="$PERF_VM_TIMEOUT_DEFAULT_FAST" ;;
    heavy) default_value="$PERF_VM_TIMEOUT_DEFAULT_HEAVY" ;;
    *)
      echo "[error] invalid perf timeout profile: ${profile} (expected fast|heavy)" >&2
      return 2
      ;;
  esac
  printf '%s\n' "${PERF_VM_TIMEOUT:-$default_value}"
}

perf_require_bool_01() {
  local key="${1:-}"
  local value="${2:-}"
  if [[ "${value}" != "0" && "${value}" != "1" ]]; then
    echo "[error] ${key} must be 0|1: got '${value}'" >&2
    return 2
  fi
  return 0
}

perf_resolve_bool_01_env() {
  local key="${1:-}"
  local default_value="${2:-0}"
  local value=""
  if [[ -z "${key}" ]]; then
    echo "[error] perf_resolve_bool_01_env requires env key" >&2
    return 2
  fi
  if ! perf_require_bool_01 "default(${key})" "${default_value}"; then
    return 2
  fi
  value="${!key:-$default_value}"
  if ! perf_require_bool_01 "${key}" "${value}"; then
    return 2
  fi
  printf '%s\n' "${value}"
}

perf_require_uint_env_value() {
  local key="${1:-}"
  local value="${2:-}"
  if ! [[ "${value}" =~ ^[0-9]+$ ]]; then
    echo "[error] ${key} must be numeric seconds: got '${value}'" >&2
    return 2
  fi
  return 0
}

perf_resolve_uint_env() {
  local key="${1:-}"
  local default_value="${2:-0}"
  local value=""
  if [[ -z "${key}" ]]; then
    echo "[error] perf_resolve_uint_env requires env key" >&2
    return 2
  fi
  if ! perf_require_uint_env_value "default(${key})" "${default_value}"; then
    return 2
  fi
  value="${!key:-$default_value}"
  if ! perf_require_uint_env_value "${key}" "${value}"; then
    return 2
  fi
  printf '%s\n' "${value}"
}

perf_resolve_aot_timeout_sec() {
  perf_resolve_uint_env "PERF_AOT_TIMEOUT_SEC" "20"
}

perf_resolve_aot_auto_safepoint() {
  # Perf-lane SSOT:
  # 1) PERF_AOT_AUTO_SAFEPOINT explicit
  # 2) NYASH_LLVM_AUTO_SAFEPOINT fallback (compat)
  # 3) default 0 for bench4 AOT comparability
  local value=""
  local source="PERF_AOT_AUTO_SAFEPOINT"

  if [[ -n "${PERF_AOT_AUTO_SAFEPOINT:-}" ]]; then
    value="${PERF_AOT_AUTO_SAFEPOINT}"
    source="PERF_AOT_AUTO_SAFEPOINT"
  elif [[ -n "${NYASH_LLVM_AUTO_SAFEPOINT:-}" ]]; then
    value="${NYASH_LLVM_AUTO_SAFEPOINT}"
    source="NYASH_LLVM_AUTO_SAFEPOINT"
  else
    value="0"
    source="default"
  fi

  if ! perf_require_bool_01 "${source}" "${value}"; then
    return 2
  fi
  printf '%s\n' "${value}"
}

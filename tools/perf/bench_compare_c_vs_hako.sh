#!/usr/bin/env bash
set -euo pipefail

# Compare a C baseline vs Hakorune VM for a given bench key.
# Usage: bench_compare_c_vs_hako.sh <bench_key> [warmup] [repeat]
#   bench_key: box_create_destroy_small | method_call_only_small
# Env:
#   PERF_VM_TIMEOUT=<duration>  # default from bench_env fast profile
#   PERF_AOT_TIMEOUT_SEC=20     # timeout seconds for AOT series/probe
#   PERF_SKIP_VM_PREFLIGHT=0    # set 1 to skip VM preflight fail-fast probe
# Output: [bench] name=<key> c_ms=<med> ny_ms=<med> ratio=<c/ny>

KEY=${1:-}
WARMUP=${2:-2}
REPEAT=${3:-5}

if [[ -z "${KEY}" ]]; then
  echo "Usage: $0 <bench_key> [warmup] [repeat]" >&2
  exit 2
fi

ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
TARGET_DIR="${ROOT_DIR}/target"
C_SRC="${ROOT_DIR}/benchmarks/c/bench_${KEY}.c"
C_BIN="${TARGET_DIR}/perf_c_${KEY}"
HAKO_PROG="${ROOT_DIR}/benchmarks/bench_${KEY}.hako"
HAKORUNE_BIN="${TARGET_DIR}/release/hakorune"
TMP_AOT_FILES=()

cleanup_tmp_aot_files() {
  local f
  for f in "${TMP_AOT_FILES[@]:-}"; do
    rm -f "$f" >/dev/null 2>&1 || true
  done
}
trap cleanup_tmp_aot_files EXIT

source "${ROOT_DIR}/tools/perf/lib/bench_env.sh"
source "${ROOT_DIR}/tools/perf/lib/bench_compare_common.sh"
source "${ROOT_DIR}/tools/perf/lib/aot_helpers.sh"
VM_TIMEOUT="$(perf_vm_timeout_resolve fast)"
AOT_TIMEOUT_SEC="$(perf_resolve_aot_timeout_sec)" || exit $?
SKIP_VM_PREFLIGHT="$(perf_resolve_bool_01_env PERF_SKIP_VM_PREFLIGHT 0)" || exit $?
SUBTRACT_STARTUP="$(perf_resolve_bool_01_env PERF_SUBTRACT_STARTUP 0)" || exit $?
PERF_AOT_ENABLED="$(perf_resolve_bool_01_env PERF_AOT 0)" || exit $?

if [[ ! -x "${HAKORUNE_BIN}" ]]; then
  echo "[hint] hakorune not built. Run: cargo build --release" >&2
  exit 2
fi

if [[ ! -f "${C_SRC}" ]]; then
  echo "[error] C source not found: ${C_SRC}" >&2
  exit 2
fi
if [[ ! -f "${HAKO_PROG}" ]]; then
  echo "[error] Hako program not found: ${HAKO_PROG}" >&2
  exit 2
fi

mkdir -p "${TARGET_DIR}"

# Build C baseline
cc -O3 -march=native -mtune=native -o "${C_BIN}" "${C_SRC}" 2>/dev/null || cc -O3 -o "${C_BIN}" "${C_SRC}"

# C series
C_SERIES=$(perf_collect_series "${WARMUP}" "${REPEAT}" "${C_BIN}")
C_MED=$(printf "%s\n" "${C_SERIES}" | perf_median_ms)

# Hako series (VM)
VM_BENCH_CMD=(env "${NYASH_VM_BENCH_ENV[@]}" timeout "${VM_TIMEOUT}" "${HAKORUNE_BIN}" --backend vm "${HAKO_PROG}")
if [[ "${SKIP_VM_PREFLIGHT}" != "1" ]]; then
  if ! perf_run_vm_preflight_or_fail "${KEY}" "${VM_TIMEOUT}" "tools/perf/bench_compare_c_vs_hako.sh" "${WARMUP}" "${REPEAT}" "${VM_BENCH_CMD[@]}"; then
    exit 1
  fi
fi
HAKO_SERIES=$(perf_collect_series "${WARMUP}" "${REPEAT}" "${VM_BENCH_CMD[@]}")
HAKO_MED=$(printf "%s\n" "${HAKO_SERIES}" | perf_median_ms)
if [[ "${SUBTRACT_STARTUP}" == "1" ]]; then
  tmp_ret0=$(mktemp --suffix .hako)
  cat >"${tmp_ret0}" <<'HAKO'
static box Main { main() { return 0 } }
HAKO
  base_series=$(perf_collect_series 1 3 env "${NYASH_VM_BENCH_ENV[@]}" timeout "${VM_TIMEOUT}" "${HAKORUNE_BIN}" --backend vm "${tmp_ret0}")
  base_med=$(printf "%s\n" "${base_series}" | perf_median_ms)
  rm -f "${tmp_ret0}" || true
  if [[ "${base_med}" =~ ^[0-9]+$ ]]; then
    HAKO_MED=$(( HAKO_MED>base_med ? HAKO_MED-base_med : 0 ))
  fi
fi

# ratio = c / ny (1.0 means parity)
ratio() {
  python3 - "$@" <<'PY'
import sys
c, n = map(float, sys.argv[1:3])
print(f"{(c/n) if n>0 else 0.0:.2f}")
PY
}

RATIO=$(ratio "${C_MED}" "${HAKO_MED}")

printf "[bench] name=%-24s c_ms=%s ny_ms=%s ratio=%s\n" "${KEY}" "${C_MED}" "${HAKO_MED}" "${RATIO}"

# Optional AOT
if [[ "${PERF_AOT_ENABLED}" == "1" ]]; then
  AOT_MED=0
  AOT_STATUS="skip"
  AOT_REASON="not_attempted"
  AOT_STAGE="none"
  EXE_OUT="${TARGET_DIR}/perf_ny_${KEY}.${BASHPID}.exe"
  TMP_AOT_FILES+=("${EXE_OUT}")
  if perf_run_aot_bench_series "${ROOT_DIR}" "${HAKORUNE_BIN}" "${HAKO_PROG}" "${EXE_OUT}" "${WARMUP}" "${REPEAT}" "${AOT_TIMEOUT_SEC}"; then
    AOT_MED="${PERF_AOT_LAST_MED_MS}"
    AOT_STATUS="${PERF_AOT_LAST_STATUS}"
    AOT_REASON="${PERF_AOT_LAST_REASON}"
    AOT_STAGE="${PERF_AOT_LAST_STAGE}"
    if [[ "${SUBTRACT_STARTUP}" == "1" ]]; then
      EXE_R0="${TARGET_DIR}/perf_ny_ret0.${BASHPID}.exe"
      TMP_AOT_FILES+=("${EXE_R0}")
      if perf_run_ret0_aot_series "${ROOT_DIR}" "${HAKORUNE_BIN}" "${EXE_R0}" 1 3 "${AOT_TIMEOUT_SEC}"; then
        BASE_MED="${PERF_AOT_LAST_MED_MS}"
        if [[ "${BASE_MED}" =~ ^[0-9]+$ ]]; then
          AOT_MED=$(( AOT_MED>BASE_MED ? AOT_MED-BASE_MED : 0 ))
        fi
      else
        AOT_REASON="ok_ret0_skip"
        AOT_STAGE="run"
      fi
    fi
  else
    AOT_STATUS="${PERF_AOT_LAST_STATUS}"
    AOT_REASON="${PERF_AOT_LAST_REASON}"
    AOT_STAGE="${PERF_AOT_LAST_STAGE}"
  fi
  RATIO_AOT=$(ratio "${C_MED}" "${AOT_MED}")
  printf "[bench] name=%-24s c_ms=%s ny_aot_ms=%s ratio=%s status=%s reason=%s stage=%s\n" \
    "${KEY} (aot)" "${C_MED}" "${AOT_MED}" "${RATIO_AOT}" "${AOT_STATUS}" "${AOT_REASON}" "${AOT_STAGE}"
fi

#!/usr/bin/env bash
set -euo pipefail

# Compare C / Python / Hakorune VM / Hakorune LLVM-AOT for a given bench key.
# Usage: bench_compare_c_py_vs_hako.sh <bench_key> [warmup] [repeat]
#   bench_key: chip8_kernel_small | kilo_kernel_small
# Env:
#   PERF_VM_TIMEOUT=<duration>  # default from bench_env fast profile
#   PERF_AOT_TIMEOUT_SEC=20     # timeout seconds for AOT series/probe
#   PERF_SKIP_VM_PREFLIGHT=0    # set 1 to skip VM preflight fail-fast probe
#   PERF_AOT_AUTO_SAFEPOINT=0|1 # default 0 in bench4 AOT lane (invalid values fail-fast)
# Output: [bench4] name=<key> c_ms=<n> py_ms=<n> ny_vm_ms=<n> ny_aot_ms=<n> ratio_c_vm=<r> ratio_c_py=<r> ratio_c_aot=<r> aot_status=<ok|skip|fail>
#
# Ratio definitions (1.00 = parity with C):
#   ratio_c_vm  = c_ms / ny_vm_ms
#   ratio_c_py  = c_ms / py_ms
#   ratio_c_aot = c_ms / ny_aot_ms
#
# Note: Output parsing should use key-based extraction (not fixed-width)
# Example: grep -oP 'c_ms=\K[0-9]+' or awk '{for(i=1;i<=NF;i++) if($i~/^c_ms=/) print $i}'

KEY=${1:-}
WARMUP=${2:-2}
REPEAT=${3:-5}

if [[ -z "${KEY}" ]]; then
  echo "Usage: $0 <bench_key> [warmup] [repeat]" >&2
  exit 2
fi

is_allowed_key() {
  local key="$1"
  case "${key}" in
    chip8_kernel_small|kilo_kernel_small)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

if ! is_allowed_key "${KEY}"; then
  echo "[error] Unsupported bench key for bench4 contract: ${KEY}" >&2
  echo "[hint] Allowed keys: chip8_kernel_small kilo_kernel_small" >&2
  exit 2
fi

ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
TARGET_DIR="${ROOT_DIR}/target"
C_SRC="${ROOT_DIR}/benchmarks/c/bench_${KEY}.c"
C_BIN="${TARGET_DIR}/perf_c_${KEY}"
PY_SRC="${ROOT_DIR}/benchmarks/python/bench_${KEY}.py"
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

if [[ ! -x "${HAKORUNE_BIN}" ]]; then
  echo "[hint] hakorune not built. Run: cargo build --release" >&2
  exit 2
fi

if [[ ! -f "${C_SRC}" ]]; then
  echo "[error] C source not found: ${C_SRC}" >&2
  exit 2
fi
if [[ ! -f "${PY_SRC}" ]]; then
  echo "[error] Python source not found: ${PY_SRC}" >&2
  exit 2
fi
if [[ ! -f "${HAKO_PROG}" ]]; then
  echo "[error] Hako program not found: ${HAKO_PROG}" >&2
  exit 2
fi

mkdir -p "${TARGET_DIR}"

# Build C baseline
cc -O3 -march=native -mtune=native -o "${C_BIN}" "${C_SRC}" 2>/dev/null || cc -O3 -o "${C_BIN}" "${C_SRC}"

# Ratio helper (c / other, 1.00 = parity)
ratio() {
  python3 - "$@" <<'PY'
import sys
c, other = map(float, sys.argv[1:3])
print(f"{(c/other) if other>0 else 0.0:.2f}")
PY
}

# C series
C_SERIES=$(perf_collect_series "${WARMUP}" "${REPEAT}" "${C_BIN}")
C_MED=$(printf "%s\n" "${C_SERIES}" | perf_median_ms)

# Python series
PY_SERIES=$(perf_collect_series "${WARMUP}" "${REPEAT}" python3 "${PY_SRC}")
PY_MED=$(printf "%s\n" "${PY_SERIES}" | perf_median_ms)

# Hakorune VM series
VM_BENCH_CMD=(env "${NYASH_VM_BENCH_ENV[@]}" timeout "${VM_TIMEOUT}" "${HAKORUNE_BIN}" --backend vm "${HAKO_PROG}")
if [[ "${SKIP_VM_PREFLIGHT}" != "1" ]]; then
  if ! perf_run_vm_preflight_or_fail "${KEY}" "${VM_TIMEOUT}" "tools/perf/bench_compare_c_py_vs_hako.sh" "${WARMUP}" "${REPEAT}" "${VM_BENCH_CMD[@]}"; then
    exit 1
  fi
fi
HAKO_SERIES=$(perf_collect_series "${WARMUP}" "${REPEAT}" "${VM_BENCH_CMD[@]}")
NY_VM_MED=$(printf "%s\n" "${HAKO_SERIES}" | perf_median_ms)

# Optional AOT
NY_AOT_MED=0
AOT_STATUS="skip"
AOT_REASON="not_attempted"
AOT_STAGE="none"
EXE_OUT="${TARGET_DIR}/perf_ny_${KEY}.${BASHPID}.exe"
TMP_AOT_FILES+=("${EXE_OUT}")
AOT_AUTO_SAFEPOINT="$(perf_resolve_aot_auto_safepoint)" || exit $?
if NYASH_LLVM_AUTO_SAFEPOINT="${AOT_AUTO_SAFEPOINT}" \
  perf_run_aot_bench_series "${ROOT_DIR}" "${HAKORUNE_BIN}" "${HAKO_PROG}" "${EXE_OUT}" "${WARMUP}" "${REPEAT}" "${AOT_TIMEOUT_SEC}"; then
  NY_AOT_MED="${PERF_AOT_LAST_MED_MS}"
  AOT_STATUS="${PERF_AOT_LAST_STATUS}"
  AOT_REASON="${PERF_AOT_LAST_REASON}"
  AOT_STAGE="${PERF_AOT_LAST_STAGE}"
else
  AOT_STATUS="${PERF_AOT_LAST_STATUS}"
  AOT_REASON="${PERF_AOT_LAST_REASON}"
  AOT_STAGE="${PERF_AOT_LAST_STAGE}"
fi

# Calculate ratios
RATIO_C_VM=$(ratio "${C_MED}" "${NY_VM_MED}")
RATIO_C_PY=$(ratio "${C_MED}" "${PY_MED}")
RATIO_C_AOT=$(ratio "${C_MED}" "${NY_AOT_MED}")

# Normalize aot_status for output (ok|skip|fail)
if [[ "${AOT_STATUS}" == "ok" ]]; then
  AOT_STATUS_OUT="ok"
elif [[ "${AOT_STATUS}" == "skip" ]]; then
  AOT_STATUS_OUT="skip"
else
  AOT_STATUS_OUT="fail"
fi

# Output single line (parse by key, not fixed-width)
printf "[bench4] name=%s c_ms=%s py_ms=%s ny_vm_ms=%s ny_aot_ms=%s ratio_c_vm=%s ratio_c_py=%s ratio_c_aot=%s aot_status=%s\n" \
  "${KEY}" "${C_MED}" "${PY_MED}" "${NY_VM_MED}" "${NY_AOT_MED}" "${RATIO_C_VM}" "${RATIO_C_PY}" "${RATIO_C_AOT}" "${AOT_STATUS_OUT}"

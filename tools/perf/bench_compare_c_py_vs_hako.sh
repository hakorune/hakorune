#!/usr/bin/env bash
set -euo pipefail

# Compare C / Python / Hakorune VM / Hakorune LLVM-AOT for a given bench key.
#
# Judgment policy:
#   - repeat < 3 is probe-only
#   - keep/reject decisions require at least 3 runs plus a quick ASM probe
#   - WSL / allocator-like noisy lanes should be rechecked with repeat=20 before closing
#
# Usage: bench_compare_c_py_vs_hako.sh <bench_key> [warmup] [repeat]
#   bench_key:
#     chip8_kernel_small | chip8_kernel_small_hk | chip8_kernel_small_rk
#     kilo_kernel_small  | kilo_kernel_small_hk  | kilo_kernel_small_rk
# Env:
#   PERF_VM_TIMEOUT=<duration>  # default from bench_env fast profile
#   PERF_AOT_TIMEOUT_SEC=20     # timeout seconds for AOT series/probe
#   PERF_SKIP_VM_PREFLIGHT=0    # set 1 to skip VM preflight fail-fast probe
#   PERF_VM_FORCE_NO_FALLBACK=0 # set 1 to force NYASH_VM_USE_FALLBACK=0 (route strict)
#   PERF_AOT_AUTO_SAFEPOINT=0|1 # default 0 in bench4 AOT lane (invalid values fail-fast)
#   PERF_ROUTE_PROBE=1          # set 0 to disable one-shot vm route marker probe
#   PERF_AOT_DIRECT_ONLY=0|1    # default 1 for *_hk, direct --emit-mir-json only
#   PERF_AOT_PREFER_HELPER=0|1  # when 1, prefer emit_mir_route.sh --route hako-helper for MIR emit
#   PERF_AOT_HELPER_ONLY=0|1    # when 1, fail-fast if helper emit fails (no direct fallback)
#   PERF_REQUIRE_AOT_RESULT_PARITY=0|1
#     - default: 1 for *_hk keys, 0 otherwise
#     - when 1, fail-fast if VM RC marker and AOT Result marker diverge
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

if (( REPEAT < 3 )); then
  echo "[warn] repeat=${REPEAT} is below the perf judgment minimum (3); treat this run as probe-only" >&2
fi

if [[ -z "${KEY}" ]]; then
  echo "Usage: $0 <bench_key> [warmup] [repeat]" >&2
  exit 2
fi

ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
source "${ROOT_DIR}/tools/perf/lib/bench_key_alias.sh"

if ! perf_is_supported_bench4_key "${KEY}"; then
  echo "[error] Unsupported bench key for bench4 contract: ${KEY}" >&2
  echo "[hint] Allowed keys: chip8_kernel_small chip8_kernel_small_hk chip8_kernel_small_rk kilo_kernel_small kilo_kernel_small_hk kilo_kernel_small_rk" >&2
  exit 2
fi

TARGET_DIR="${ROOT_DIR}/target"
BENCH_DATASET_KEY="$(perf_resolve_bench_dataset_key "${KEY}")"
C_SRC="${ROOT_DIR}/benchmarks/c/bench_${BENCH_DATASET_KEY}.c"
C_BIN="${TARGET_DIR}/perf_c_${KEY}"
PY_SRC="${ROOT_DIR}/benchmarks/python/bench_${BENCH_DATASET_KEY}.py"
HAKO_PROG="${ROOT_DIR}/benchmarks/bench_${BENCH_DATASET_KEY}.hako"
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

# Perf bench4 compares are boundary-owned AOT routes, not llvmlite keep-lane probes.
# Smoke env defaults NYASH_LLVM_USE_HARNESS=1 for legacy LLVM diagnostics, so clear it here.
export NYASH_LLVM_USE_HARNESS=0

VM_TIMEOUT="$(perf_vm_timeout_resolve fast)"
AOT_TIMEOUT_SEC="$(perf_resolve_aot_timeout_sec)" || exit $?
SKIP_VM_PREFLIGHT="$(perf_resolve_bool_01_env PERF_SKIP_VM_PREFLIGHT 0)" || exit $?
VM_FORCE_NO_FALLBACK="$(perf_resolve_bool_01_env PERF_VM_FORCE_NO_FALLBACK 0)" || exit $?
ROUTE_PROBE_ENABLED="$(perf_resolve_bool_01_env PERF_ROUTE_PROBE 1)" || exit $?

KERNEL_LANE="default"
KERNEL_NAME="kernel-bootstrap"
if [[ "${KEY}" == *_hk ]]; then
  KERNEL_LANE="hk"
  KERNEL_NAME="kernel-mainline"
elif [[ "${KEY}" == *_rk ]]; then
  KERNEL_LANE="rk"
  KERNEL_NAME="kernel-bootstrap"
fi

if [[ "${KERNEL_LANE}" == "hk" && "${VM_FORCE_NO_FALLBACK}" != "1" ]]; then
  echo "[error] bench key '${KEY}' requires strict route: set PERF_VM_FORCE_NO_FALLBACK=1" >&2
  echo "[hint] hk line is kernel-mainline (.hako no-compat). Silent fallback is blocked." >&2
  exit 2
fi

RESULT_PARITY_DEFAULT="0"
if [[ "${KERNEL_LANE}" == "hk" ]]; then
  RESULT_PARITY_DEFAULT="1"
fi
REQUIRE_AOT_RESULT_PARITY="$(perf_resolve_bool_01_env PERF_REQUIRE_AOT_RESULT_PARITY "${RESULT_PARITY_DEFAULT}")" || exit $?
AOT_HELPER_ONLY_DEFAULT="0"
PERF_AOT_DIRECT_ONLY_DEFAULT="0"
if [[ "${KERNEL_LANE}" == "hk" ]]; then
  AOT_HELPER_ONLY_DEFAULT="0"
  PERF_AOT_DIRECT_ONLY_DEFAULT="1"
fi
AOT_PREFER_HELPER_DEFAULT="0"
AOT_HELPER_ONLY="$(perf_resolve_bool_01_env PERF_AOT_HELPER_ONLY "${AOT_HELPER_ONLY_DEFAULT}")" || exit $?
AOT_DIRECT_ONLY="$(perf_resolve_bool_01_env PERF_AOT_DIRECT_ONLY "${PERF_AOT_DIRECT_ONLY_DEFAULT}")" || exit $?
AOT_PREFER_HELPER="$(perf_resolve_bool_01_env PERF_AOT_PREFER_HELPER "${AOT_PREFER_HELPER_DEFAULT}")" || exit $?
if [[ "${AOT_HELPER_ONLY}" == "1" && "${AOT_PREFER_HELPER}" != "1" ]]; then
  echo "[error] PERF_AOT_HELPER_ONLY=1 requires PERF_AOT_PREFER_HELPER=1" >&2
  exit 2
fi
if [[ "${AOT_DIRECT_ONLY}" == "1" && "${AOT_PREFER_HELPER}" == "1" ]]; then
  echo "[error] PERF_AOT_DIRECT_ONLY=1 is incompatible with PERF_AOT_PREFER_HELPER=1" >&2
  exit 2
fi
if [[ "${AOT_DIRECT_ONLY}" == "1" && "${AOT_HELPER_ONLY}" == "1" ]]; then
  echo "[error] PERF_AOT_DIRECT_ONLY=1 is incompatible with PERF_AOT_HELPER_ONLY=1" >&2
  exit 2
fi

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

extract_route_field() {
  local line="$1"
  local key="$2"
  printf '%s\n' "${line}" | sed -n "s/.*${key}=\\([^ ]*\\).*/\\1/p" | tail -n 1
}

extract_vm_rc_marker() {
  local text="$1"
  printf '%s\n' "${text}" | sed -n 's/.*RC:[[:space:]]*\(-\?[0-9]\+\).*/\1/p' | tail -n 1
}

extract_aot_result_marker() {
  local text="$1"
  printf '%s\n' "${text}" | sed -n 's/.*Result:[[:space:]]*\(-\?[0-9]\+\).*/\1/p' | tail -n 1
}

# C series
C_SERIES=$(perf_collect_series "${WARMUP}" "${REPEAT}" "${C_BIN}")
C_MED=$(printf "%s\n" "${C_SERIES}" | perf_median_ms)

# Python series
PY_SERIES=$(perf_collect_series "${WARMUP}" "${REPEAT}" python3 "${PY_SRC}")
PY_MED=$(printf "%s\n" "${PY_SERIES}" | perf_median_ms)

# Hakorune VM series
VM_ENV=("${NYASH_VM_BENCH_ENV[@]}")
if [[ "${VM_FORCE_NO_FALLBACK}" == "1" ]]; then
  VM_ENV+=(NYASH_VM_USE_FALLBACK=0)
fi
VM_BENCH_CMD=(env "${VM_ENV[@]}" timeout "${VM_TIMEOUT}" "${HAKORUNE_BIN}" --backend vm "${HAKO_PROG}")

VM_ROUTE_LINE=""
DERUST_ROUTE_LINE=""
VM_ROUTE_LANE="unknown"
VM_ROUTE_REASON="unknown"
DERUST_ROUTE_LANE="unknown"
DERUST_SOURCE="unknown"
DERUST_REASON="unknown"
VM_ENGINE="unknown"
if [[ "${ROUTE_PROBE_ENABLED}" == "1" ]]; then
  ROUTE_PROBE_OUT="$(
    env "${VM_ENV[@]}" NYASH_VM_ROUTE_TRACE=1 HAKO_VM_MAX_STEPS=1 \
      timeout "${VM_TIMEOUT}" "${HAKORUNE_BIN}" --backend vm "${HAKO_PROG}" 2>&1 || true
  )"
  VM_ROUTE_LINE="$(printf '%s\n' "${ROUTE_PROBE_OUT}" | grep '^\[vm-route/select\]' | tail -n 1 || true)"
  DERUST_ROUTE_LINE="$(printf '%s\n' "${ROUTE_PROBE_OUT}" | grep '^\[derust-route/select\]' | tail -n 1 || true)"
  if [[ -n "${VM_ROUTE_LINE}" ]]; then
    VM_ROUTE_LANE="$(extract_route_field "${VM_ROUTE_LINE}" "lane")"
    VM_ROUTE_REASON="$(extract_route_field "${VM_ROUTE_LINE}" "reason")"
    [[ -n "${VM_ROUTE_LANE}" ]] || VM_ROUTE_LANE="unknown"
    [[ -n "${VM_ROUTE_REASON}" ]] || VM_ROUTE_REASON="unknown"
  fi
  if [[ -n "${DERUST_ROUTE_LINE}" ]]; then
    DERUST_ROUTE_LANE="$(extract_route_field "${DERUST_ROUTE_LINE}" "lane")"
    DERUST_SOURCE="$(extract_route_field "${DERUST_ROUTE_LINE}" "source")"
    DERUST_REASON="$(extract_route_field "${DERUST_ROUTE_LINE}" "reason")"
    [[ -n "${DERUST_ROUTE_LANE}" ]] || DERUST_ROUTE_LANE="unknown"
    [[ -n "${DERUST_SOURCE}" ]] || DERUST_SOURCE="unknown"
    [[ -n "${DERUST_REASON}" ]] || DERUST_REASON="unknown"
  fi
  case "${DERUST_SOURCE}" in
    rust* | *rust*)
      VM_ENGINE="rust-vm"
      ;;
    hako* | *hako*)
      VM_ENGINE="hako-vm"
      ;;
    *)
      VM_ENGINE="unknown"
      ;;
  esac
fi

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
AOT_EMIT_ROUTE="none"
AOT_COMPILE_RECIPE="unknown"
AOT_COMPAT_REPLAY="unknown"
AOT_REPLAY_LANE="none"
AOT_REPLAY_REASON="not_attempted"
RESULT_PARITY_STATUS="skip"
VM_RESULT_MARKER="na"
AOT_RESULT_MARKER="na"
EXE_OUT="${TARGET_DIR}/perf_ny_${KEY}.${BASHPID}.exe"
TMP_AOT_FILES+=("${EXE_OUT}")
AOT_AUTO_SAFEPOINT="$(perf_resolve_aot_auto_safepoint)" || exit $?
if NYASH_LLVM_AUTO_SAFEPOINT="${AOT_AUTO_SAFEPOINT}" \
  PERF_AOT_DIRECT_ONLY="${AOT_DIRECT_ONLY}" PERF_AOT_PREFER_HELPER="${AOT_PREFER_HELPER}" PERF_AOT_HELPER_ONLY="${AOT_HELPER_ONLY}" \
  perf_run_aot_bench_series "${ROOT_DIR}" "${HAKORUNE_BIN}" "${HAKO_PROG}" "${EXE_OUT}" "${WARMUP}" "${REPEAT}" "${AOT_TIMEOUT_SEC}"; then
  NY_AOT_MED="${PERF_AOT_LAST_MED_MS}"
  AOT_STATUS="${PERF_AOT_LAST_STATUS}"
  AOT_REASON="${PERF_AOT_LAST_REASON}"
  AOT_STAGE="${PERF_AOT_LAST_STAGE}"
  AOT_EMIT_ROUTE="${PERF_AOT_LAST_EMIT_ROUTE:-none}"
  AOT_COMPILE_RECIPE="${PERF_AOT_LAST_COMPILE_RECIPE:-unknown}"
  AOT_COMPAT_REPLAY="${PERF_AOT_LAST_COMPAT_REPLAY:-unknown}"
  AOT_REPLAY_LANE="${PERF_AOT_LAST_REPLAY_LANE:-none}"
  AOT_REPLAY_REASON="${PERF_AOT_LAST_REPLAY_REASON:-not_attempted}"
else
  AOT_STATUS="${PERF_AOT_LAST_STATUS}"
  AOT_REASON="${PERF_AOT_LAST_REASON}"
  AOT_STAGE="${PERF_AOT_LAST_STAGE}"
  AOT_EMIT_ROUTE="${PERF_AOT_LAST_EMIT_ROUTE:-none}"
  AOT_COMPILE_RECIPE="${PERF_AOT_LAST_COMPILE_RECIPE:-unknown}"
  AOT_COMPAT_REPLAY="${PERF_AOT_LAST_COMPAT_REPLAY:-unknown}"
  AOT_REPLAY_LANE="${PERF_AOT_LAST_REPLAY_LANE:-none}"
  AOT_REPLAY_REASON="${PERF_AOT_LAST_REPLAY_REASON:-not_attempted}"
fi

if [[ "${REQUIRE_AOT_RESULT_PARITY}" == "1" ]]; then
  if [[ "${AOT_STATUS}" != "ok" ]]; then
    echo "[error] Result parity requires AOT status=ok (key=${KEY}, got ${AOT_STATUS})" >&2
    echo "[hint] Set PERF_REQUIRE_AOT_RESULT_PARITY=0 only for diagnostic runs." >&2
    exit 1
  fi
  set +e
  VM_RESULT_OUT="$(${VM_BENCH_CMD[@]} 2>&1)"
  VM_RESULT_RC=$?
  set -e
  if [[ "${VM_RESULT_RC}" -eq 124 || "${VM_RESULT_OUT}" == *"vm step budget exceeded"* || "${VM_RESULT_OUT}" == *"[ERROR]"* ]]; then
    echo "[error] VM probe failed while checking result parity (key=${KEY}, rc=${VM_RESULT_RC})" >&2
    printf '%s\n' "${VM_RESULT_OUT}" | sed -n '1,8p' >&2
    exit 1
  fi
  VM_RESULT_MARKER="$(extract_vm_rc_marker "${VM_RESULT_OUT}")"
  if [[ -z "${VM_RESULT_MARKER}" ]]; then
    echo "[error] VM result marker (RC:) not found for parity check (key=${KEY})" >&2
    printf '%s\n' "${VM_RESULT_OUT}" | sed -n '1,8p' >&2
    exit 1
  fi

  set +e
  # `NYASH_VM_USE_FALLBACK` is VM-only policy. Passing it to AOT runtime mutates
  # nyrt behavior and can corrupt Result markers, so keep AOT probe env isolated.
  AOT_RESULT_OUT="$(perf_aot_runtime_env_cmd timeout "${AOT_TIMEOUT_SEC}s" "${EXE_OUT}" 2>&1)"
  AOT_RESULT_RC=$?
  set -e
  if [[ "${AOT_RESULT_RC}" -eq 124 ]]; then
    echo "[error] AOT probe timeout while checking result parity (key=${KEY})" >&2
    exit 1
  fi
  AOT_RESULT_MARKER="$(extract_aot_result_marker "${AOT_RESULT_OUT}")"
  if [[ -z "${AOT_RESULT_MARKER}" ]]; then
    echo "[error] AOT result marker (Result:) not found for parity check (key=${KEY}, rc=${AOT_RESULT_RC})" >&2
    printf '%s\n' "${AOT_RESULT_OUT}" | sed -n '1,8p' >&2
    exit 1
  fi
  if [[ "${VM_RESULT_MARKER}" != "${AOT_RESULT_MARKER}" ]]; then
    echo "[error] VM/AOT result mismatch (key=${KEY} vm=${VM_RESULT_MARKER} aot=${AOT_RESULT_MARKER})" >&2
    echo "[hint] This usually means route drift or hidden fallback." >&2
    exit 1
  fi
  RESULT_PARITY_STATUS="ok"
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
printf "[bench4-route] name=%s dataset=%s kernel_lane=%s kernel_name=%s fallback_guard=%s vm_engine=%s vm_lane=%s vm_reason=%s derust_lane=%s derust_source=%s derust_reason=%s aot_direct_only=%s aot_emit_route=%s aot_recipe=%s aot_compat_replay=%s aot_replay_lane=%s aot_replay_reason=%s route_probe=%s result_parity=%s vm_result=%s aot_result=%s\n" \
  "${KEY}" "${BENCH_DATASET_KEY}" "${KERNEL_LANE}" "${KERNEL_NAME}" \
  "$([[ "${VM_FORCE_NO_FALLBACK}" == "1" ]] && printf '%s' "strict-no-fallback" || printf '%s' "runtime-default")" \
  "${VM_ENGINE}" "${VM_ROUTE_LANE}" "${VM_ROUTE_REASON}" "${DERUST_ROUTE_LANE}" "${DERUST_SOURCE}" "${DERUST_REASON}" "${AOT_DIRECT_ONLY}" "${AOT_EMIT_ROUTE}" \
  "${AOT_COMPILE_RECIPE}" "${AOT_COMPAT_REPLAY}" "${AOT_REPLAY_LANE}" "${AOT_REPLAY_REASON}" \
  "${ROUTE_PROBE_ENABLED}" "${RESULT_PARITY_STATUS}" "${VM_RESULT_MARKER}" "${AOT_RESULT_MARKER}"

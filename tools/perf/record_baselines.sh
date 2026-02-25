#!/usr/bin/env bash
set -euo pipefail

# Record baseline timings for C and Python (and Hakorune VM/AOT) into benchmarks/baselines/.
# Usage: record_baselines.sh <bench_key|all> [warmup=2] [repeat=7]
# Env:
#   PERF_SUBTRACT_STARTUP=1  subtract minimal startup baseline (ret0) for VM/AOT
#   NYASH_LLVM_BACKEND=crate|native  select LLVM builder backend for AOT (default auto)
#   PERF_VM_TIMEOUT=20s  per-run timeout for VM series
#   PERF_AOT_TIMEOUT_SEC=20  timeout seconds for AOT series/probe
#   bench_key: box_create_destroy_small | method_call_only_small | all

ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
TARGET_DIR="${ROOT_DIR}/target"
OUT_DIR="${PERF_BASELINE_OUT_DIR:-${ROOT_DIR}/benchmarks/baselines}"
PY_DIR="${ROOT_DIR}/benchmarks/python"
C_DIR="${ROOT_DIR}/benchmarks/c"
source "${ROOT_DIR}/tools/perf/lib/bench_env.sh"
source "${ROOT_DIR}/tools/perf/lib/aot_helpers.sh"

KEY=${1:-all}
WARMUP=${2:-2}
REPEAT=${3:-7}
VM_TIMEOUT=${PERF_VM_TIMEOUT:-20s}
AOT_TIMEOUT_SEC=${PERF_AOT_TIMEOUT_SEC:-20}

if ! [[ "${AOT_TIMEOUT_SEC}" =~ ^[0-9]+$ ]]; then
  echo "[error] PERF_AOT_TIMEOUT_SEC must be numeric seconds: got '${AOT_TIMEOUT_SEC}'" >&2
  exit 2
fi

mkdir -p "${OUT_DIR}"

time_ms() { date +%s%3N; }
measure_cmd_ms() { local t1 t2; t1=$(time_ms); "$@" >/dev/null 2>&1 || true; t2=$(time_ms); echo $((t2-t1)); }
median_ms() { awk 'NF{print $1}' | sort -n | awk '{a[NR]=$1} END{ if(NR==0){print 0; exit} n=int((NR+1)/2); print a[n] }'; }

collect_series() {
  local warmup=$1; shift
  local repeat=$1; shift
  local -a cmd=("$@")
  for _ in $(seq 1 "${warmup}"); do measure_cmd_ms "${cmd[@]}" >/dev/null || true; done
  for _ in $(seq 1 "${repeat}"); do measure_cmd_ms "${cmd[@]}"; done
}

ensure_c_built() {
  local key=$1
  local c_src="${C_DIR}/bench_${key}.c"
  local c_bin="${TARGET_DIR}/perf_c_${key}"
  if [[ ! -f "${c_src}" ]]; then echo "[error] missing ${c_src}" >&2; return 1; fi
  mkdir -p "${TARGET_DIR}"
  cc -O3 -march=native -mtune=native -o "${c_bin}" "${c_src}" 2>/dev/null || cc -O3 -o "${c_bin}" "${c_src}"
}

record_one() {
  local key=$1
  local ts host c_ms py_ms ny_vm_ms ny_aot_ms ny_aot_status ny_aot_reason

  # C
  ensure_c_built "${key}"
  local c_bin="${TARGET_DIR}/perf_c_${key}"
  local c_series; c_series=$(collect_series "${WARMUP}" "${REPEAT}" "${c_bin}")
  c_ms=$(printf "%s\n" "${c_series}" | median_ms)

  # Python
  local py_file="${PY_DIR}/bench_${key}.py"
  if command -v python3 >/dev/null 2>&1 && [[ -f "${py_file}" ]]; then
    local py_series; py_series=$(collect_series "${WARMUP}" "${REPEAT}" python3 "${py_file}")
    py_ms=$(printf "%s\n" "${py_series}" | median_ms)
  else
    py_ms=0
  fi

  # Hakorune VM (optional)
  local hako_bin="${TARGET_DIR}/release/hakorune"
  local hako_prog="${ROOT_DIR}/benchmarks/bench_${key}.hako"
  if [[ -x "${hako_bin}" && -f "${hako_prog}" ]]; then
    local hako_series; hako_series=$(collect_series "${WARMUP}" "${REPEAT}" env "${NYASH_VM_BENCH_ENV[@]}" timeout "${VM_TIMEOUT}" "${hako_bin}" --backend vm "${hako_prog}")
    ny_vm_ms=$(printf "%s\n" "${hako_series}" | median_ms)
    # Optional subtract: measure minimal VM startup (ret0)
    if [[ "${PERF_SUBTRACT_STARTUP:-0}" == "1" ]]; then
      local tmp_ret0="$(mktemp --suffix .hako)"; cat >"${tmp_ret0}" <<'HAKO'
static box Main { main() { return 0 } }
HAKO
      local base_series; base_series=$(collect_series 1 3 env "${NYASH_VM_BENCH_ENV[@]}" timeout "${VM_TIMEOUT}" "${hako_bin}" --backend vm "${tmp_ret0}")
      local base_ms; base_ms=$(printf "%s\n" "${base_series}" | median_ms)
      rm -f "${tmp_ret0}" || true
      if [[ "${base_ms}" =~ ^[0-9]+$ ]]; then
        ny_vm_ms=$(( ny_vm_ms>base_ms ? ny_vm_ms-base_ms : 0 ))
      fi
    fi
  else
    ny_vm_ms=0
  fi

  # AOT (crate/native backend)
  ny_aot_ms=0
  ny_aot_status="skip"
  ny_aot_reason="not_attempted"
  ny_aot_stage="none"
  if [[ -x "${hako_bin}" && -f "${hako_prog}" ]]; then
    local exe_path="${TARGET_DIR}/perf_ny_${key}.exe"
    if perf_run_aot_bench_series "${ROOT_DIR}" "${hako_bin}" "${hako_prog}" "${exe_path}" "${WARMUP}" "${REPEAT}" "${AOT_TIMEOUT_SEC}"; then
      ny_aot_ms="${PERF_AOT_LAST_MED_MS}"
      ny_aot_status="${PERF_AOT_LAST_STATUS}"
      ny_aot_reason="${PERF_AOT_LAST_REASON}"
      ny_aot_stage="${PERF_AOT_LAST_STAGE}"
      # Optional subtract: minimal AOT startup (ret0)
      if [[ "${PERF_SUBTRACT_STARTUP:-0}" == "1" ]]; then
        local ret0_exe="${TARGET_DIR}/perf_ny_ret0.exe"
        if perf_run_ret0_aot_series "${ROOT_DIR}" "${hako_bin}" "${ret0_exe}" 1 3 "${AOT_TIMEOUT_SEC}"; then
          local base_ms="${PERF_AOT_LAST_MED_MS}"
          if [[ "${base_ms}" =~ ^[0-9]+$ ]]; then
            ny_aot_ms=$(( ny_aot_ms>base_ms ? ny_aot_ms-base_ms : 0 ))
          fi
        else
          ny_aot_reason="ok_ret0_skip"
          ny_aot_stage="run"
        fi
      fi
    else
      ny_aot_status="${PERF_AOT_LAST_STATUS}"
      ny_aot_reason="${PERF_AOT_LAST_REASON}"
      ny_aot_stage="${PERF_AOT_LAST_STAGE}"
    fi
  fi

  ts=$(date -Is)
  host=$(hostname 2>/dev/null || echo unknown)

  local obj; obj=$(cat <<JSON
{
  "bench": "${key}",
  "ts": "${ts}",
  "host": "${host}",
  "unit": "ms",
  "warmup": ${WARMUP},
  "repeat": ${REPEAT},
  "c_ms": ${c_ms},
  "py_ms": ${py_ms},
  "ny_vm_ms": ${ny_vm_ms},
  "ny_aot_ms": ${ny_aot_ms},
  "ny_aot_status": "${ny_aot_status}",
  "ny_aot_reason": "${ny_aot_reason}",
  "ny_aot_stage": "${ny_aot_stage}"
}
JSON
)
  local obj_compact
  obj_compact=$(printf "%s\n" "${obj}" | jq -c . 2>/dev/null || printf "%s" "${obj}" | tr '\n' ' ')

  printf "%s\n" "${obj}" > "${OUT_DIR}/${key}.latest.json"
  printf "%s\n" "${obj_compact}" >> "${OUT_DIR}/${key}.ndjson"
  echo "[saved] ${OUT_DIR}/${key}.latest.json"
}

run_keys=("${KEY}")
if [[ "${KEY}" == "all" ]]; then
  run_keys=(box_create_destroy_small method_call_only_small)
fi

for k in "${run_keys[@]}"; do
  record_one "${k}"
done

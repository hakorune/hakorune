#!/usr/bin/env bash
set -euo pipefail

# bench_compare_compile_run_split.sh
#
# Measure compile/run split for one benchmark key:
#   1) total source route   (--backend vm <program.hako>)
#   2) compile only         (emit route: helper|direct)
#   3) run prebuilt MIR     (--backend vm --mir-json-file <tmp.mir>)
#
# Usage:
#   tools/perf/bench_compare_compile_run_split.sh <bench_key> [warmup] [repeat]
#
# Env:
#   PERF_SPLIT_OUTPUT=text|json             default: text
#   PERF_SPLIT_EMIT_ROUTE=helper|direct     default: direct
#   PERF_SPLIT_JSON_OPT_IN_RATIO_PCT=<n>   default: 40
#   PERF_SPLIT_MIN_TOTAL_MS=<n>            default: 100
#   PERF_SPLIT_EMIT_TIMEOUT=<duration>     default: PERF_VM_TIMEOUT (fast profile)
#   NYASH_STAGE1_BINARY_ONLY_DIRECT=1      default: 1 (when route=direct)
#   HAKO_EMIT_MIR_MAINLINE_ONLY=0|1        default: helper default (route=helper only)
# Output contract:
#   - success line/json always includes `status=ok`

KEY=${1:-}
WARMUP=${2:-1}
REPEAT=${3:-3}

if [[ -z "${KEY}" ]]; then
  echo "Usage: $0 <bench_key> [warmup] [repeat]" >&2
  exit 2
fi

ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
TARGET_DIR="${ROOT_DIR}/target"
HAKO_PROG="${ROOT_DIR}/benchmarks/bench_${KEY}.hako"
HAKORUNE_BIN="${TARGET_DIR}/release/hakorune"
EMIT_HELPER="${ROOT_DIR}/tools/hakorune_emit_mir.sh"

source "${ROOT_DIR}/tools/perf/lib/bench_env.sh"
VM_TIMEOUT="$(perf_vm_timeout_resolve fast)"
EMIT_TIMEOUT="${PERF_SPLIT_EMIT_TIMEOUT:-$VM_TIMEOUT}"

PERF_SPLIT_OUTPUT="${PERF_SPLIT_OUTPUT:-text}"
EMIT_ROUTE="${PERF_SPLIT_EMIT_ROUTE:-direct}"
SPLIT_RATIO_PCT="${PERF_SPLIT_JSON_OPT_IN_RATIO_PCT:-40}"
SPLIT_MIN_TOTAL_MS="${PERF_SPLIT_MIN_TOTAL_MS:-100}"

if ! [[ "${WARMUP}" =~ ^[0-9]+$ ]] || ! [[ "${REPEAT}" =~ ^[0-9]+$ ]]; then
  echo "[error] warmup/repeat must be numeric: warmup=${WARMUP} repeat=${REPEAT}" >&2
  exit 2
fi
if ! [[ "${SPLIT_RATIO_PCT}" =~ ^[0-9]+$ ]]; then
  echo "[error] PERF_SPLIT_JSON_OPT_IN_RATIO_PCT must be numeric: ${SPLIT_RATIO_PCT}" >&2
  exit 2
fi
if ! [[ "${SPLIT_MIN_TOTAL_MS}" =~ ^[0-9]+$ ]]; then
  echo "[error] PERF_SPLIT_MIN_TOTAL_MS must be numeric: ${SPLIT_MIN_TOTAL_MS}" >&2
  exit 2
fi
if [[ "${EMIT_ROUTE}" != "helper" && "${EMIT_ROUTE}" != "direct" ]]; then
  echo "[error] PERF_SPLIT_EMIT_ROUTE must be helper|direct: ${EMIT_ROUTE}" >&2
  exit 2
fi

if [[ ! -x "${HAKORUNE_BIN}" ]]; then
  echo "[hint] hakorune not built. Run: cargo build --release --bin hakorune" >&2
  exit 2
fi
if [[ ! -f "${HAKO_PROG}" ]]; then
  echo "[error] Hako program not found: ${HAKO_PROG}" >&2
  exit 2
fi
if [[ "${EMIT_ROUTE}" == "helper" && ! -f "${EMIT_HELPER}" ]]; then
  echo "[error] emit helper not found: ${EMIT_HELPER}" >&2
  exit 2
fi

time_ms() {
  date +%s%3N
}

median_ms() {
  awk 'NF{print $1}' | sort -n | awk '{ a[NR]=$1 } END { if (NR==0) {print 0; exit} n=int((NR+1)/2); print a[n] }'
}

measure_allow_nonzero_rc() {
  local cmd=("$@")
  local t1 t2 dt
  t1=$(time_ms)
  "${cmd[@]}" >/dev/null 2>&1 || true
  t2=$(time_ms)
  dt=$((t2 - t1))
  echo "$dt"
}

collect_series_allow_nonzero_rc() {
  local warmup="$1"; shift
  local repeat="$1"; shift
  local -a cmd=("$@")
  local i
  for ((i=0; i<warmup; i++)); do
    measure_allow_nonzero_rc "${cmd[@]}" >/dev/null || true
  done
  for ((i=0; i<repeat; i++)); do
    measure_allow_nonzero_rc "${cmd[@]}"
  done
}

print_vm_fail_hints() {
  local rc="$1"
  local stderr_log="$2"
  if [[ "${stderr_log}" == *"vm step budget exceeded"* ]]; then
    echo "[hint] VM step budget exceeded for benchmark '${KEY}'." >&2
    echo "[hint] Try raising HAKO_VM_MAX_STEPS (example):" >&2
    echo "[hint]   HAKO_VM_MAX_STEPS=200000000 bash tools/perf/bench_compare_compile_run_split.sh ${KEY} ${WARMUP} ${REPEAT}" >&2
    echo "[hint] For diagnostics only: HAKO_VM_MAX_STEPS=0 (unlimited)." >&2
    return
  fi
  if [[ "${rc}" -eq 124 ]]; then
    echo "[hint] benchmark timed out (${VM_TIMEOUT})." >&2
    echo "[hint] This can be caused by step budget or very slow execution." >&2
    echo "[hint] Consider raising HAKO_VM_MAX_STEPS or lowering benchmark size for diagnosis." >&2
  fi
}

run_vm_preflight_allow_numeric_rc() {
  local phase="$1"
  shift
  local -a cmd=("$@")
  local rc=0
  local cmd_err=""
  set +e
  cmd_err=$("${cmd[@]}" 2>&1 >/dev/null)
  rc=$?
  set -e
  local hard_fail=0
  if [[ "${rc}" -eq 124 ]]; then
    hard_fail=1
  fi
  if [[ "${cmd_err}" == *"vm step budget exceeded"* || "${cmd_err}" == *"[ERROR]"* ]]; then
    hard_fail=1
  fi
  if [[ "${hard_fail}" -eq 1 ]]; then
    echo "[error] preflight failed: phase=${phase} key=${KEY} rc=${rc}" >&2
    print_vm_fail_hints "${rc}" "${cmd_err}"
    if [[ -n "${cmd_err}" ]]; then
      echo "[error] stderr (first lines):" >&2
      printf '%s\n' "${cmd_err}" | sed -n '1,8p' >&2
    fi
    exit 1
  fi
}

TMP_MIR=$(mktemp /tmp/perf_compile_run_split.XXXXXX.mir.json)
cleanup() {
  rm -f "${TMP_MIR}" >/dev/null 2>&1 || true
}
trap cleanup EXIT

emit_once_strict() {
  local t1 t2 dt
  rm -f "${TMP_MIR}" || true
  t1=$(time_ms)
  set +e
  local out
  if [[ "${EMIT_ROUTE}" == "helper" ]]; then
    out=$(
      env NYASH_DISABLE_PLUGINS=1 \
          timeout "${EMIT_TIMEOUT}" \
          bash "${EMIT_HELPER}" "${HAKO_PROG}" "${TMP_MIR}" 2>&1
    )
  else
    out=$(
      env "${NYASH_VM_BENCH_ENV[@]}" \
          NYASH_STAGE1_BINARY_ONLY_DIRECT="${NYASH_STAGE1_BINARY_ONLY_DIRECT:-1}" \
          timeout "${EMIT_TIMEOUT}" \
          "${HAKORUNE_BIN}" --hako-emit-mir-json "${TMP_MIR}" "${HAKO_PROG}" 2>&1
    )
  fi
  local rc=$?
  set -e
  t2=$(time_ms)
  dt=$((t2 - t1))
  if [[ "${rc}" -ne 0 ]]; then
    echo "[error] emit failed: key=${KEY} route=${EMIT_ROUTE} rc=${rc}" >&2
    if [[ "${rc}" -eq 124 || "${out}" == *"timed out"* ]]; then
      echo "[hint] emit timeout (${EMIT_TIMEOUT}). Try PERF_SPLIT_EMIT_TIMEOUT=120s" >&2
    fi
    if [[ "${out}" == *"vm step budget exceeded"* ]]; then
      echo "[hint] Try raising HAKO_VM_MAX_STEPS (example): HAKO_VM_MAX_STEPS=200000000" >&2
    fi
    printf '%s\n' "${out}" | sed -n '1,12p' >&2
    return 1
  fi
  if [[ ! -s "${TMP_MIR}" ]]; then
    echo "[error] emit produced empty MIR file: ${TMP_MIR}" >&2
    return 1
  fi
  echo "${dt}"
}

collect_emit_series() {
  local warmup="$1"
  local repeat="$2"
  local i
  for ((i=0; i<warmup; i++)); do
    emit_once_strict >/dev/null
  done
  for ((i=0; i<repeat; i++)); do
    emit_once_strict
  done
}

# 1) total source route
TOTAL_CMD=(env "${NYASH_VM_BENCH_ENV[@]}" timeout "${VM_TIMEOUT}" "${HAKORUNE_BIN}" --backend vm "${HAKO_PROG}")
run_vm_preflight_allow_numeric_rc "total_source" "${TOTAL_CMD[@]}"
TOTAL_SERIES=$(collect_series_allow_nonzero_rc "${WARMUP}" "${REPEAT}" "${TOTAL_CMD[@]}")
TOTAL_MED=$(printf '%s\n' "${TOTAL_SERIES}" | median_ms)

# 2) compile only (emit MIR JSON)
EMIT_SERIES=$(collect_emit_series "${WARMUP}" "${REPEAT}")
EMIT_MED=$(printf '%s\n' "${EMIT_SERIES}" | median_ms)

# 3) run prebuilt MIR route (must be executable; bench payload non-zero RC is allowed)
PREBUILT_CMD=(env "${NYASH_VM_BENCH_ENV[@]}" timeout "${VM_TIMEOUT}" "${HAKORUNE_BIN}" --backend vm --mir-json-file "${TMP_MIR}")
run_vm_preflight_allow_numeric_rc "run_prebuilt" "${PREBUILT_CMD[@]}"
RUN_SERIES=$(collect_series_allow_nonzero_rc "${WARMUP}" "${REPEAT}" "${PREBUILT_CMD[@]}")
RUN_MED=$(printf '%s\n' "${RUN_SERIES}" | median_ms)

calc_pct() {
  python3 - "$@" <<'PY'
import sys
num = float(sys.argv[1])
den = float(sys.argv[2])
print(f"{(num*100.0/den) if den>0 else 0.0:.1f}")
PY
}

EMIT_SHARE_PCT=$(calc_pct "${EMIT_MED}" "${TOTAL_MED}")
RUN_SHARE_PCT=$(calc_pct "${RUN_MED}" "${TOTAL_MED}")

DECISION="vm_hotpath_priority"
if (( TOTAL_MED >= SPLIT_MIN_TOTAL_MS )) && (( EMIT_MED * 100 >= TOTAL_MED * SPLIT_RATIO_PCT )); then
  DECISION="json_opt_candidate"
fi
STATUS="ok"

if [[ "${PERF_SPLIT_OUTPUT}" == "json" ]]; then
  python3 - "$KEY" "$EMIT_ROUTE" "$WARMUP" "$REPEAT" "$TOTAL_MED" "$EMIT_MED" "$RUN_MED" "$EMIT_SHARE_PCT" "$RUN_SHARE_PCT" "$DECISION" "$SPLIT_RATIO_PCT" "$SPLIT_MIN_TOTAL_MS" "$STATUS" <<'PY'
import json
import sys
out = {
    "name": sys.argv[1],
    "emit_route": sys.argv[2],
    "warmup": int(sys.argv[3]),
    "repeat": int(sys.argv[4]),
    "total_ms": int(sys.argv[5]),
    "emit_ms": int(sys.argv[6]),
    "run_prebuilt_ms": int(sys.argv[7]),
    "emit_share_pct": float(sys.argv[8]),
    "run_prebuilt_share_pct": float(sys.argv[9]),
    "decision": sys.argv[10],
    "decision_thresholds": {
        "json_opt_in_ratio_pct": int(sys.argv[11]),
        "min_total_ms": int(sys.argv[12]),
    },
    "status": sys.argv[13],
}
print(json.dumps(out, separators=(",", ":")))
PY
else
  printf "[bench-split] name=%s status=%s emit_route=%s total_ms=%s emit_ms=%s run_prebuilt_ms=%s emit_share_pct=%s run_prebuilt_share_pct=%s decision=%s threshold_pct=%s min_total_ms=%s\n" \
    "${KEY}" "${STATUS}" "${EMIT_ROUTE}" "${TOTAL_MED}" "${EMIT_MED}" "${RUN_MED}" "${EMIT_SHARE_PCT}" "${RUN_SHARE_PCT}" "${DECISION}" "${SPLIT_RATIO_PCT}" "${SPLIT_MIN_TOTAL_MS}"
fi

#!/usr/bin/env bash
set -euo pipefail

# bench_apps_mir_mode_compare.sh
# Compare app wallclock totals between:
#   - PERF_APPS_MIR_SHAPE_INPUT_MODE=emit
#   - PERF_APPS_MIR_SHAPE_INPUT_MODE=prebuilt
#
# Usage:
#   tools/perf/bench_apps_mir_mode_compare.sh [warmup] [repeat] [backend]
#   tools/perf/bench_apps_mir_mode_compare.sh [warmup] [repeat] [backend] --json-lines <N>
# Env:
#   PERF_APPS_MIR_MODE_DELTA_SAMPLES=<N>  default 1
#   PERF_APPS_MIR_MODE_SIGNIFICANCE_MS=<N> default 10
# Notes:
#   --json-lines N prints N sample JSON lines and one summary JSON line.
# Output (summary JSON):
#   {
#     "backend":"vm",
#     "warmup":1,
#     "repeat":1,
#     "samples":1,
#     "emit_total_ms":123,
#     "prebuilt_total_ms":130,
#     "delta_ms":7,
#     "delta_pct":5.69,
#     "delta_ms_min":7,
#     "delta_ms_median":7,
#     "delta_ms_max":7,
#     "winner":"emit",
#     "hotspot_case_delta":{"case":"mir_shape_guard","delta_ms_abs":5,...}
#   }

WARMUP=1
REPEAT=1
BACKEND=vm
SAMPLES=${PERF_APPS_MIR_MODE_DELTA_SAMPLES:-1}
SIGNIFICANCE_MS=${PERF_APPS_MIR_MODE_SIGNIFICANCE_MS:-10}
JSON_LINES=0
SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
source "${SCRIPT_DIR}/lib/compare_mode_common.sh"

if ! perf_compare_parse_cli_args WARMUP REPEAT BACKEND SAMPLES JSON_LINES "$@"; then
  exit $?
fi

if ! [[ "${WARMUP}" =~ ^[0-9]+$ ]] || ! [[ "${REPEAT}" =~ ^[0-9]+$ ]]; then
  echo "[error] warmup/repeat must be non-negative integers: warmup=${WARMUP} repeat=${REPEAT}" >&2
  exit 2
fi
if ! [[ "${SAMPLES}" =~ ^[0-9]+$ ]] || [[ "${SAMPLES}" -le 0 ]]; then
  echo "[error] samples must be positive integer: ${SAMPLES}" >&2
  exit 2
fi
if ! [[ "${SIGNIFICANCE_MS}" =~ ^[0-9]+$ ]]; then
  echo "[error] PERF_APPS_MIR_MODE_SIGNIFICANCE_MS must be non-negative integer: ${SIGNIFICANCE_MS}" >&2
  exit 2
fi

ROOT_DIR=$(cd "${SCRIPT_DIR}/../.." && pwd)
APP_BENCH="${ROOT_DIR}/tools/perf/bench_apps_wallclock.sh"
PREBUILT_DEFAULT="${ROOT_DIR}/apps/tests/mir_shape_guard/method_call_only_small.prebuilt.mir.json"
PREBUILT="${PERF_APPS_MIR_SHAPE_PREBUILT:-$PREBUILT_DEFAULT}"

if [[ ! -f "${APP_BENCH}" ]]; then
  echo "[error] app bench script missing: ${APP_BENCH}" >&2
  exit 2
fi
if [[ ! -f "${PREBUILT}" ]]; then
  echo "[error] prebuilt MIR missing: ${PREBUILT}" >&2
  exit 2
fi
if [[ ! -s "${PREBUILT}" ]]; then
  echo "[error] prebuilt MIR is empty: ${PREBUILT}" >&2
  exit 2
fi

TMP_EMIT_SAMPLES=$(mktemp /tmp/perf_apps_mir_mode_emit.XXXXXX.jsonl)
TMP_PREBUILT_SAMPLES=$(mktemp /tmp/perf_apps_mir_mode_prebuilt.XXXXXX.jsonl)
cleanup() {
  rm -f "${TMP_EMIT_SAMPLES}" "${TMP_PREBUILT_SAMPLES}" >/dev/null 2>&1 || true
}
trap cleanup EXIT

run_mode_json() {
  local mode="$1"
  local out compact
  out="$(
    PERF_APPS_OUTPUT=json \
    PERF_APPS_MIR_SHAPE_INPUT_MODE="${mode}" \
    PERF_APPS_MIR_SHAPE_PREBUILT="${PREBUILT}" \
    bash "${APP_BENCH}" "${WARMUP}" "${REPEAT}" "${BACKEND}" 2>&1
  )" || {
    echo "[error] mode=${mode} run failed" >&2
    echo "${out}" >&2
    exit 1
  }

  compact="$(perf_compare_compact_json_or_fail "${mode}" "${out}")" || exit 1

  printf "%s\n" "${compact}"
}

series_stats() {
  awk 'NF{print $1}' | sort -n | awk '{ a[NR]=$1 } END { if (NR==0) { print "0 0 0"; exit } n=int((NR+1)/2); print a[1], a[n], a[NR] }'
}

calc_delta_pct() {
  python3 - "$1" "$2" <<'PY'
import sys
emit = float(sys.argv[1])
pre = float(sys.argv[2])
if emit <= 0:
    print("0.00")
else:
    pct = ((pre - emit) / emit) * 100.0
    print(f"{pct:.2f}")
PY
}

sample_winner() {
  local emit="$1"
  local pre="$2"
  if [[ "${emit}" -le "${pre}" ]]; then
    printf "emit\n"
  else
    printf "prebuilt\n"
  fi
}

emit_sample_json() {
  local backend="$1"
  local warmup="$2"
  local repeat="$3"
  local sample="$4"
  local emit_total="$5"
  local prebuilt_total="$6"
  local delta="$7"
  local delta_pct="$8"
  local winner="$9"
  python3 - "${backend}" "${warmup}" "${repeat}" "${sample}" "${emit_total}" "${prebuilt_total}" "${delta}" "${delta_pct}" "${winner}" <<'PY'
import json
import sys

print(json.dumps({
    "kind": "sample",
    "backend": sys.argv[1],
    "warmup": int(sys.argv[2]),
    "repeat": int(sys.argv[3]),
    "sample": int(sys.argv[4]),
    "emit_total_ms": int(sys.argv[5]),
    "prebuilt_total_ms": int(sys.argv[6]),
    "delta_ms": int(sys.argv[7]),
    "delta_pct": float(sys.argv[8]),
    "winner": sys.argv[9],
}, separators=(",", ":")))
PY
}

declare -a EMIT_SERIES=()
declare -a PREBUILT_SERIES=()
declare -a DELTA_SERIES=()

for i in $(seq 1 "${SAMPLES}"); do
  emit_json=$(run_mode_json "emit")
  prebuilt_json=$(run_mode_json "prebuilt")
  printf '%s\n' "${emit_json}" >> "${TMP_EMIT_SAMPLES}"
  printf '%s\n' "${prebuilt_json}" >> "${TMP_PREBUILT_SAMPLES}"

  emit_total_ms="$(printf '%s\n' "${emit_json}" | jq -r '.total_ms')"
  prebuilt_total_ms="$(printf '%s\n' "${prebuilt_json}" | jq -r '.total_ms')"
  delta_ms=$(( prebuilt_total_ms - emit_total_ms ))
  delta_pct=$(calc_delta_pct "${emit_total_ms}" "${prebuilt_total_ms}")
  winner=$(sample_winner "${emit_total_ms}" "${prebuilt_total_ms}")

  EMIT_SERIES+=("${emit_total_ms}")
  PREBUILT_SERIES+=("${prebuilt_total_ms}")
  DELTA_SERIES+=("${delta_ms}")

  if [[ "${JSON_LINES}" == "1" ]]; then
    emit_sample_json "${BACKEND}" "${WARMUP}" "${REPEAT}" "${i}" "${emit_total_ms}" "${prebuilt_total_ms}" "${delta_ms}" "${delta_pct}" "${winner}"
  fi
done

read emit_min emit_med emit_max < <(printf '%s\n' "${EMIT_SERIES[@]}" | series_stats)
read pre_min pre_med pre_max < <(printf '%s\n' "${PREBUILT_SERIES[@]}" | series_stats)
read delta_min delta_med delta_max < <(printf '%s\n' "${DELTA_SERIES[@]}" | series_stats)

emit_total_ms="${emit_med}"
prebuilt_total_ms="${pre_med}"
delta_ms=$(( prebuilt_total_ms - emit_total_ms ))
delta_pct=$(calc_delta_pct "${emit_total_ms}" "${prebuilt_total_ms}")
winner=$(sample_winner "${emit_total_ms}" "${prebuilt_total_ms}")
delta_ms_abs="${delta_ms}"
if [[ "${delta_ms_abs}" -lt 0 ]]; then
  delta_ms_abs=$(( -delta_ms_abs ))
fi
significant=0
winner_significant="tie"
if [[ "${delta_ms_abs}" -ge "${SIGNIFICANCE_MS}" ]]; then
  significant=1
  winner_significant="${winner}"
fi

case_stats_json="$(
  python3 - "${TMP_EMIT_SAMPLES}" "${TMP_PREBUILT_SAMPLES}" "${SIGNIFICANCE_MS}" <<'PY'
import json
import sys

emit_path = sys.argv[1]
prebuilt_path = sys.argv[2]
threshold = int(sys.argv[3])

def median(nums):
    ordered = sorted(nums)
    return ordered[(len(ordered) - 1) // 2]

def fail(msg):
    print(msg, file=sys.stderr)
    raise SystemExit(1)

with open(emit_path, "r", encoding="utf-8") as f:
    emit_rows = [json.loads(line) for line in f if line.strip()]
with open(prebuilt_path, "r", encoding="utf-8") as f:
    prebuilt_rows = [json.loads(line) for line in f if line.strip()]

if not emit_rows:
    fail("[error] no emit samples")
if len(emit_rows) != len(prebuilt_rows):
    fail("[error] emit/prebuilt sample size mismatch")

emit_samples = {}
prebuilt_samples = {}

for emit_obj, prebuilt_obj in zip(emit_rows, prebuilt_rows):
    emit_cases = emit_obj.get("cases")
    prebuilt_cases = prebuilt_obj.get("cases")
    if not isinstance(emit_cases, dict) or not emit_cases:
        fail("[error] emit cases missing or empty")
    if not isinstance(prebuilt_cases, dict) or not prebuilt_cases:
        fail("[error] prebuilt cases missing or empty")
    if set(emit_cases.keys()) != set(prebuilt_cases.keys()):
        fail("[error] emit/prebuilt case keys mismatch")

    for case_name in emit_cases.keys():
        emit_value = int(emit_cases[case_name])
        prebuilt_value = int(prebuilt_cases[case_name])
        if emit_value <= 0 or prebuilt_value <= 0:
            fail(f"[error] non-positive case value: {case_name}")
        emit_samples.setdefault(case_name, []).append(emit_value)
        prebuilt_samples.setdefault(case_name, []).append(prebuilt_value)

emit_cases_median = {}
prebuilt_cases_median = {}
case_delta = {}
case_winner = {}
hotspot = None

for case_name in sorted(emit_samples.keys()):
    emit_med = median(emit_samples[case_name])
    prebuilt_med = median(prebuilt_samples[case_name])
    delta = prebuilt_med - emit_med
    delta_abs = abs(delta)
    winner = "emit" if emit_med <= prebuilt_med else "prebuilt"
    is_significant = 1 if delta_abs >= threshold else 0

    emit_cases_median[case_name] = emit_med
    prebuilt_cases_median[case_name] = prebuilt_med
    case_delta[case_name] = delta
    case_winner[case_name] = winner

    candidate = {
        "case": case_name,
        "emit_ms": emit_med,
        "prebuilt_ms": prebuilt_med,
        "delta_ms": delta,
        "delta_ms_abs": delta_abs,
        "winner": winner,
        "significant": is_significant,
    }
    if (
        hotspot is None
        or candidate["delta_ms_abs"] > hotspot["delta_ms_abs"]
        or (
            candidate["delta_ms_abs"] == hotspot["delta_ms_abs"]
            and candidate["case"] < hotspot["case"]
        )
    ):
        hotspot = candidate

print(
    json.dumps(
        {
            "emit_cases_median_ms": emit_cases_median,
            "prebuilt_cases_median_ms": prebuilt_cases_median,
            "case_delta_ms": case_delta,
            "case_winner": case_winner,
            "hotspot_case_delta": hotspot,
        },
        separators=(",", ":"),
    )
)
PY
)"

if [[ "${JSON_LINES}" == "1" ]]; then
  kind="summary"
else
  kind=""
fi

python3 - "${BACKEND}" "${WARMUP}" "${REPEAT}" "${SAMPLES}" \
  "${emit_total_ms}" "${prebuilt_total_ms}" "${delta_ms}" "${delta_pct}" "${winner}" \
  "${emit_min}" "${emit_med}" "${emit_max}" \
  "${pre_min}" "${pre_med}" "${pre_max}" \
  "${delta_min}" "${delta_med}" "${delta_max}" \
  "${kind}" "${SIGNIFICANCE_MS}" "${delta_ms_abs}" "${significant}" "${winner_significant}" \
  "${case_stats_json}" <<'PY'
import json
import sys

case_stats = json.loads(sys.argv[24])
out = {
    "backend": sys.argv[1],
    "warmup": int(sys.argv[2]),
    "repeat": int(sys.argv[3]),
    "samples": int(sys.argv[4]),
    "emit_total_ms": int(sys.argv[5]),
    "prebuilt_total_ms": int(sys.argv[6]),
    "delta_ms": int(sys.argv[7]),
    "delta_pct": float(sys.argv[8]),
    "winner": sys.argv[9],
    "emit_total_ms_min": int(sys.argv[10]),
    "emit_total_ms_median": int(sys.argv[11]),
    "emit_total_ms_max": int(sys.argv[12]),
    "prebuilt_total_ms_min": int(sys.argv[13]),
    "prebuilt_total_ms_median": int(sys.argv[14]),
    "prebuilt_total_ms_max": int(sys.argv[15]),
    "delta_ms_min": int(sys.argv[16]),
    "delta_ms_median": int(sys.argv[17]),
    "delta_ms_max": int(sys.argv[18]),
    "significance_ms_threshold": int(sys.argv[20]),
    "delta_ms_abs": int(sys.argv[21]),
    "significant": int(sys.argv[22]),
    "winner_significant": sys.argv[23],
}
out.update(case_stats)

kind = sys.argv[19]
if kind:
    out["kind"] = kind

print(json.dumps(out, separators=(",", ":")))
PY

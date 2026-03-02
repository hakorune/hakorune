#!/usr/bin/env bash
set -euo pipefail

# Stable runner for bench4 (C/Py/VM/AOT) comparisons.
# Adds:
# - optional CPU pinning via taskset
# - larger default warmup/repeat
# - multi-round median consolidation
#
# Usage:
#   tools/perf/bench_compare_c_py_vs_hako_stable.sh <bench_key> [cpu_set] [rounds] [warmup] [repeat]
#
# Examples:
#   tools/perf/bench_compare_c_py_vs_hako_stable.sh kilo_kernel_small
#   tools/perf/bench_compare_c_py_vs_hako_stable.sh kilo_kernel_small_hk
#   tools/perf/bench_compare_c_py_vs_hako_stable.sh kilo_kernel_small_rk
#   tools/perf/bench_compare_c_py_vs_hako_stable.sh kilo_kernel_small auto 5 5 11
#   tools/perf/bench_compare_c_py_vs_hako_stable.sh kilo_kernel_small none 3 5 11

KEY="${1:-}"
CPU_SET="${2:-auto}"
ROUNDS="${3:-3}"
WARMUP="${4:-5}"
REPEAT="${5:-11}"

if [[ -z "${KEY}" ]]; then
  echo "Usage: $0 <bench_key> [cpu_set] [rounds] [warmup] [repeat]" >&2
  exit 2
fi

if ! [[ "${ROUNDS}" =~ ^[0-9]+$ ]] || [[ "${ROUNDS}" -lt 1 ]]; then
  echo "[error] rounds must be >= 1: got '${ROUNDS}'" >&2
  exit 2
fi
if ! [[ "${WARMUP}" =~ ^[0-9]+$ ]] || ! [[ "${REPEAT}" =~ ^[0-9]+$ ]]; then
  echo "[error] warmup/repeat must be non-negative integers: warmup='${WARMUP}' repeat='${REPEAT}'" >&2
  exit 2
fi

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
INNER="${ROOT_DIR}/tools/perf/bench_compare_c_py_vs_hako.sh"

source "${ROOT_DIR}/tools/perf/lib/bench_compare_common.sh"

resolve_cpu_set() {
  local requested="$1"
  if [[ "${requested}" != "auto" ]]; then
    printf '%s\n' "${requested}"
    return
  fi
  local sample_sec="${PERF_STABLE_CPU_SAMPLE_SEC:-0.2}"
  if command -v python3 >/dev/null 2>&1; then
    local best_cpu=""
    best_cpu="$(
      python3 - "${sample_sec}" <<'PY'
import re
import sys
import time

sample = float(sys.argv[1]) if len(sys.argv) > 1 else 0.2
cpu_pat = re.compile(r"^cpu(\d+)\s+(.+)$")

def read_stats():
    out = {}
    with open("/proc/stat", "r", encoding="utf-8") as f:
        for line in f:
            m = cpu_pat.match(line.strip())
            if not m:
                continue
            idx = int(m.group(1))
            vals = [int(x) for x in m.group(2).split()]
            if len(vals) < 5:
                continue
            idle = vals[3] + (vals[4] if len(vals) > 4 else 0)
            total = sum(vals[:8]) if len(vals) >= 8 else sum(vals)
            out[idx] = (total, idle)
    return out

s1 = read_stats()
time.sleep(sample)
s2 = read_stats()

best_idx = None
best_idle_ratio = -1.0
for idx in sorted(set(s1) & set(s2)):
    t1, i1 = s1[idx]
    t2, i2 = s2[idx]
    dt = t2 - t1
    di = i2 - i1
    if dt <= 0:
        continue
    idle_ratio = di / dt
    if idle_ratio > best_idle_ratio:
        best_idle_ratio = idle_ratio
        best_idx = idx

if best_idx is None:
    sys.exit(1)
print(best_idx)
PY
    )" || true
    if [[ -n "${best_cpu}" && "${best_cpu}" =~ ^[0-9]+$ ]]; then
      printf '%s\n' "${best_cpu}"
      return
    fi
  fi
  if ! command -v nproc >/dev/null 2>&1; then
    printf '%s\n' "none"
    return
  fi
  local cpu_count=0
  cpu_count="$(nproc || echo 0)"
  if [[ "${cpu_count}" =~ ^[0-9]+$ ]] && [[ "${cpu_count}" -gt 1 ]]; then
    printf '%s\n' "$((cpu_count - 1))"
  else
    printf '%s\n' "none"
  fi
}

extract_field() {
  local line="$1"
  local key="$2"
  awk -v k="${key}" '
    {
      for (i = 1; i <= NF; i++) {
        if ($i ~ ("^" k "=")) {
          split($i, kv, "=");
          print kv[2];
          exit;
        }
      }
    }
  ' <<<"${line}"
}

run_once() {
  local key="$1"
  local warmup="$2"
  local repeat="$3"
  local cpu_set="${RESOLVED_CPU_SET}"

  if [[ "${cpu_set}" != "none" ]] && command -v taskset >/dev/null 2>&1; then
    taskset -c "${cpu_set}" bash "${INNER}" "${key}" "${warmup}" "${repeat}"
  else
    bash "${INNER}" "${key}" "${warmup}" "${repeat}"
  fi
}

ratio() {
  python3 - "$@" <<'PY'
import sys
c, other = map(float, sys.argv[1:3])
print(f"{(c/other) if other>0 else 0.0:.2f}")
PY
}

tmp_c="$(mktemp)"
tmp_py="$(mktemp)"
tmp_vm="$(mktemp)"
tmp_aot="$(mktemp)"
trap 'rm -f "${tmp_c}" "${tmp_py}" "${tmp_vm}" "${tmp_aot}"' EXIT

RESOLVED_CPU_SET="$(resolve_cpu_set "${CPU_SET}")"

echo "[stable] key=${KEY} cpu_set=${CPU_SET} resolved_cpu_set=${RESOLVED_CPU_SET} rounds=${ROUNDS} warmup=${WARMUP} repeat=${REPEAT}"

for i in $(seq 1 "${ROUNDS}"); do
  out="$(run_once "${KEY}" "${WARMUP}" "${REPEAT}")"
  line="$(printf "%s\n" "${out}" | rg '^\[bench4\] ' | tail -n 1 || true)"
  if [[ -z "${line}" ]]; then
    echo "[error] bench4 summary line not found (round=${i})" >&2
    printf "%s\n" "${out}" | tail -n 40 >&2
    exit 1
  fi
  c_ms="$(extract_field "${line}" "c_ms")"
  py_ms="$(extract_field "${line}" "py_ms")"
  vm_ms="$(extract_field "${line}" "ny_vm_ms")"
  aot_ms="$(extract_field "${line}" "ny_aot_ms")"
  aot_status="$(extract_field "${line}" "aot_status")"
  echo "[stable/round] idx=${i} c_ms=${c_ms} py_ms=${py_ms} ny_vm_ms=${vm_ms} ny_aot_ms=${aot_ms} aot_status=${aot_status}"
  printf "%s\n" "${c_ms}" >>"${tmp_c}"
  printf "%s\n" "${py_ms}" >>"${tmp_py}"
  printf "%s\n" "${vm_ms}" >>"${tmp_vm}"
  printf "%s\n" "${aot_ms}" >>"${tmp_aot}"
done

c_med="$(perf_median_ms <"${tmp_c}")"
py_med="$(perf_median_ms <"${tmp_py}")"
vm_med="$(perf_median_ms <"${tmp_vm}")"
aot_med="$(perf_median_ms <"${tmp_aot}")"
ratio_c_vm="$(ratio "${c_med}" "${vm_med}")"
ratio_c_py="$(ratio "${c_med}" "${py_med}")"
ratio_c_aot="$(ratio "${c_med}" "${aot_med}")"
aot_min="$(sort -n "${tmp_aot}" | head -n1)"
aot_max="$(sort -n "${tmp_aot}" | tail -n1)"

printf "[stable] name=%s rounds=%s cpu_set=%s c_ms=%s py_ms=%s ny_vm_ms=%s ny_aot_ms=%s ratio_c_vm=%s ratio_c_py=%s ratio_c_aot=%s ny_aot_min=%s ny_aot_max=%s\n" \
  "${KEY}" "${ROUNDS}" "${RESOLVED_CPU_SET}" "${c_med}" "${py_med}" "${vm_med}" "${aot_med}" "${ratio_c_vm}" "${ratio_c_py}" "${ratio_c_aot}" "${aot_min}" "${aot_max}"

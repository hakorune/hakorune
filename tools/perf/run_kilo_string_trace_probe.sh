#!/usr/bin/env bash
set -euo pipefail

# Probe-only string trace collector for the current kilo wave.
#
# This script keeps the timing lanes clean:
#   - bench_compare_* stays timing-only and suppresses trace output
#   - this probe wrapper captures the string trace lines into files
#
# Usage:
#   tools/perf/run_kilo_string_trace_probe.sh [--out-dir <dir>]
#
# Captured unit probes:
#   - string_concat_hs_contract
#   - string_insert_hsi_contract
#   - string_concat3_hhh_contract
#   - substring_hii_short_slice_materializes_under_fast_contract

OUT_DIR=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out-dir)
      OUT_DIR="${2:-}"
      shift 2
      ;;
    -h|--help)
      sed -n '1,120p' "$0"
      exit 0
      ;;
    *)
      echo "[error] unknown arg: $1" >&2
      sed -n '1,120p' "$0" >&2
      exit 2
      ;;
  esac
done

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
if [[ -z "${OUT_DIR}" ]]; then
  ts="$(date +%Y%m%d-%H%M%S)"
  OUT_DIR="${ROOT_DIR}/target/trace_logs/kilo-string-trace/${ts}"
fi

mkdir -p "${OUT_DIR}"

SUMMARY="${OUT_DIR}/summary.txt"
: > "${SUMMARY}"

append_section() {
  local title="$1"
  local log="$2"
  {
    printf '== %s ==\n' "${title}"
    if [[ -f "${log}" ]]; then
      rg -n '^\[string/trace\]' "${log}" || true
    else
      printf '[warn] missing log: %s\n' "${log}"
    fi
    printf '\n'
  } | tee -a "${SUMMARY}" >/dev/null
}

run_probe() {
  local test_name="$1"
  local log="${OUT_DIR}/${test_name}.log"
  printf '[probe] test=%s\n' "${test_name}" | tee -a "${SUMMARY}" >/dev/null
  set +e
  NYASH_LLVM_ROUTE_TRACE=1 cargo test -p nyash_kernel "${test_name}" -- --nocapture >"${log}" 2>&1
  rc=$?
  set -e
  printf '[probe] test=%s rc=%s log=%s\n' "${test_name}" "${rc}" "${log}" | tee -a "${SUMMARY}" >/dev/null
  append_section "${test_name}" "${log}"
  if [[ "${rc}" -ne 0 ]]; then
    echo "[error] probe failed: test=${test_name} rc=${rc}" >&2
    tail -n 40 "${log}" >&2 || true
    exit "${rc}"
  fi
}

run_probe "string_concat_hs_contract"
run_probe "string_insert_hsi_contract"
run_probe "string_concat3_hhh_contract"
run_probe "substring_hii_short_slice_materializes_under_fast_contract"

cat <<EOF | tee -a "${SUMMARY}"
[probe] out_dir=${OUT_DIR}
[probe] note=bench_compare_c_py_vs_hako.sh remains timing-only and suppresses trace output
EOF

printf '[probe] out_dir=%s\n' "${OUT_DIR}"
printf '[probe] summary=%s\n' "${SUMMARY}"

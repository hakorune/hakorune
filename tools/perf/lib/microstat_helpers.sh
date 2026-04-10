#!/usr/bin/env bash

perf_microstat_median_file() {
  local f="$1"
  awk 'NF{print $1}' "${f}" | sort -n | awk '
    { a[NR]=$1 }
    END {
      if (NR == 0) { print 0; exit }
      n = int((NR + 1) / 2)
      print a[n]
    }'
}

perf_microstat_ratio_fmt() {
  python3 - "$@" <<'PY'
import sys
num, den = map(float, sys.argv[1:3])
print(f"{(num/den) if den > 0 else 0.0:.2f}")
PY
}

perf_microstat_ipc_fmt() {
  python3 - "$@" <<'PY'
import sys
ins, cyc = map(float, sys.argv[1:3])
print(f"{(ins/cyc) if cyc > 0 else 0.0:.2f}")
PY
}

perf_microstat_parse_perf_event() {
  local stat_file="$1"
  local event_prefix="$2"
  awk -F';' -v ev="${event_prefix}" '
    $3 ~ ("^" ev) {
      gsub(/ /, "", $1)
      print $1
      exit
    }' "${stat_file}"
}

perf_microstat_run_perf_stat_once() {
  local stat_file
  stat_file="$(mktemp --suffix .microstat)"
  set +e
  LC_ALL=C perf stat --no-big-num -x ';' -e instructions,cycles,cache-misses "$@" \
    >/dev/null 2>"${stat_file}"
  local rc=$?
  set -e

  if [[ "${rc}" -eq 124 ]]; then
    rm -f "${stat_file}" >/dev/null 2>&1 || true
    return 124
  fi

  local instr cycles miss
  instr="$(perf_microstat_parse_perf_event "${stat_file}" "instructions")"
  cycles="$(perf_microstat_parse_perf_event "${stat_file}" "cycles")"
  miss="$(perf_microstat_parse_perf_event "${stat_file}" "cache-misses")"
  rm -f "${stat_file}" >/dev/null 2>&1 || true

  if [[ -z "${instr}" || -z "${cycles}" || -z "${miss}" ]]; then
    return 2
  fi
  if ! [[ "${instr}" =~ ^[0-9]+$ && "${cycles}" =~ ^[0-9]+$ && "${miss}" =~ ^[0-9]+$ ]]; then
    return 2
  fi

  printf '%s;%s;%s\n' "${instr}" "${cycles}" "${miss}"
  return 0
}

perf_microstat_collect_series_medians() {
  local warmup="$1"
  local repeat="$2"
  shift 2
  local -a cmd=("$@")

  local f_instr f_cycles f_miss f_ms
  f_instr="$(mktemp --suffix .instr)"
  f_cycles="$(mktemp --suffix .cycles)"
  f_miss="$(mktemp --suffix .miss)"
  f_ms="$(mktemp --suffix .ms)"

  local i raw instr cycles miss ms
  for ((i = 0; i < warmup; i++)); do
    perf_microstat_run_perf_stat_once "${cmd[@]}" >/dev/null || true
    perf_measure_cmd_ms "${cmd[@]}" >/dev/null || true
  done
  for ((i = 0; i < repeat; i++)); do
    raw="$(perf_microstat_run_perf_stat_once "${cmd[@]}")"
    instr="${raw%%;*}"
    raw="${raw#*;}"
    cycles="${raw%%;*}"
    miss="${raw##*;}"
    ms="$(perf_measure_cmd_ms "${cmd[@]}")"
    printf '%s\n' "${instr}" >>"${f_instr}"
    printf '%s\n' "${cycles}" >>"${f_cycles}"
    printf '%s\n' "${miss}" >>"${f_miss}"
    printf '%s\n' "${ms}" >>"${f_ms}"
  done

  local med_instr med_cycles med_miss med_ms
  med_instr="$(perf_microstat_median_file "${f_instr}")"
  med_cycles="$(perf_microstat_median_file "${f_cycles}")"
  med_miss="$(perf_microstat_median_file "${f_miss}")"
  med_ms="$(perf_microstat_median_file "${f_ms}")"

  rm -f "${f_instr}" "${f_cycles}" "${f_miss}" "${f_ms}" >/dev/null 2>&1 || true
  printf '%s;%s;%s;%s\n' "${med_instr}" "${med_cycles}" "${med_miss}" "${med_ms}"
}

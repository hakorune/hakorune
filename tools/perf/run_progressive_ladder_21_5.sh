#!/usr/bin/env bash
set -euo pipefail

# run_progressive_ladder_21_5.sh
# Progressive performance ladder:
#   small benches -> medium benches -> app wall-clock.
#
# Usage:
#   tools/perf/run_progressive_ladder_21_5.sh [quick|default]
#
# Env:
#   PERF_LADDER_AOT_SMALL=1
#   PERF_LADDER_AOT_MEDIUM=0
#   PERF_LADDER_AOT_SENTINEL=0|1            # override profile default (quick=0, default=1)
#   PERF_LADDER_REGRESSION_GUARD=0|1         # override profile default (quick=0, default=1)
#   PERF_LADDER_MIR_HOTOPS=0|1               # optional MIR op profile report (default OFF)
#   PERF_LADDER_MIR_HOTOPS_KEYS="..."        # keys for MIR profile report
#   PERF_LADDER_APPS=1
#   PERF_LADDER_STABILITY=1
#   NYASH_LLVM_SKIP_BUILD=1
#   PERF_LADDER_FULL_MIN_INTERVAL_MIN=720   # default profile minimum interval
#   PERF_LADDER_FULL_STATE_FILE=...         # default: target/perf_state/phase21_5_last_full_epoch
#   PERF_FORCE_FULL=1                       # override full interval guard

PROFILE=${1:-quick}
ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
source "${ROOT_DIR}/tools/perf/lib/bench_env.sh"
STATE_DIR="${ROOT_DIR}/target/perf_state"
FULL_STATE_FILE="${PERF_LADDER_FULL_STATE_FILE:-${STATE_DIR}/phase21_5_last_full_epoch}"
FULL_MIN_INTERVAL_MIN="${PERF_LADDER_FULL_MIN_INTERVAL_MIN:-720}"

if ! [[ "${FULL_MIN_INTERVAL_MIN}" =~ ^[0-9]+$ ]]; then
  echo "[ladder] invalid PERF_LADDER_FULL_MIN_INTERVAL_MIN=${FULL_MIN_INTERVAL_MIN}" >&2
  exit 2
fi
FULL_MIN_INTERVAL_SEC=$((FULL_MIN_INTERVAL_MIN * 60))

fmt_duration() {
  local sec=$1
  if (( sec < 0 )); then
    sec=0
  fi
  local h=$((sec / 3600))
  local m=$(((sec % 3600) / 60))
  local s=$((sec % 60))
  if (( h > 0 )); then
    echo "${h}h${m}m${s}s"
  elif (( m > 0 )); then
    echo "${m}m${s}s"
  else
    echo "${s}s"
  fi
}

read_last_full() {
  LAST_FULL_EPOCH=""
  LAST_FULL_ISO=""
  if [[ -f "${FULL_STATE_FILE}" ]]; then
    read -r LAST_FULL_EPOCH LAST_FULL_ISO _ < "${FULL_STATE_FILE}" || true
    if ! [[ "${LAST_FULL_EPOCH:-}" =~ ^[0-9]+$ ]]; then
      LAST_FULL_EPOCH=""
      LAST_FULL_ISO=""
    fi
  fi
}

guard_default_interval() {
  if [[ "${PROFILE}" != "default" ]]; then
    return 0
  fi
  read_last_full
  local now_epoch elapsed remain
  now_epoch=$(date +%s)
  if [[ -n "${LAST_FULL_EPOCH}" ]]; then
    elapsed=$((now_epoch - LAST_FULL_EPOCH))
    if (( elapsed < FULL_MIN_INTERVAL_SEC )) && [[ "${PERF_FORCE_FULL:-0}" != "1" ]]; then
      remain=$((FULL_MIN_INTERVAL_SEC - elapsed))
      echo "[ladder] default profile blocked (last_full=${LAST_FULL_ISO:-unknown}, elapsed=$(fmt_duration "${elapsed}"), min_interval=$(fmt_duration "${FULL_MIN_INTERVAL_SEC}"), remain=$(fmt_duration "${remain}"))" >&2
      echo "[ladder] set PERF_FORCE_FULL=1 to override" >&2
      exit 3
    fi
  fi
}

print_last_full_info() {
  read_last_full
  if [[ -n "${LAST_FULL_EPOCH}" ]]; then
    local now_epoch elapsed
    now_epoch=$(date +%s)
    elapsed=$((now_epoch - LAST_FULL_EPOCH))
    echo "[ladder] last_full=${LAST_FULL_ISO:-unknown} elapsed=$(fmt_duration "${elapsed}")"
  else
    echo "[ladder] last_full=none"
  fi
}

case "$PROFILE" in
  quick)
    SMALL_WARMUP=1; SMALL_REPEAT=1
    MEDIUM_WARMUP=1; MEDIUM_REPEAT=1
    APP_WARMUP=1; APP_REPEAT=1
    STAB_ROUNDS=2; STAB_WARMUP=1; STAB_REPEAT=1
    AOT_SENTINEL_DEFAULT=0
    REGRESSION_GUARD_DEFAULT=0
    ;;
  default)
    SMALL_WARMUP=1; SMALL_REPEAT=3
    MEDIUM_WARMUP=1; MEDIUM_REPEAT=2
    APP_WARMUP=1; APP_REPEAT=3
    STAB_ROUNDS=3; STAB_WARMUP=1; STAB_REPEAT=3
    AOT_SENTINEL_DEFAULT=1
    REGRESSION_GUARD_DEFAULT=1
    ;;
  *)
    echo "Usage: $0 [quick|default]" >&2
    exit 2
    ;;
esac

run_step() {
  local label="$1"
  shift
  echo "[ladder] ${label}"
  "$@"
}

guard_default_interval
print_last_full_info

SMALL_KEYS=(box_create_destroy_small method_call_only_small)
MEDIUM_KEYS=(box_create_destroy method_call_only)
# opt-in extra medium benchmarks (does not affect default weight)
if [[ -n "${PERF_LADDER_EXTRA_MEDIUM_KEYS:-}" ]]; then
  for key in ${PERF_LADDER_EXTRA_MEDIUM_KEYS}; do
    MEDIUM_KEYS+=("$key")
  done
fi

LADDER_AOT_SMALL="$(perf_resolve_bool_01_env PERF_LADDER_AOT_SMALL 1)" || exit $?
LADDER_AOT_MEDIUM="$(perf_resolve_bool_01_env PERF_LADDER_AOT_MEDIUM 0)" || exit $?
LADDER_MIR_HOTOPS="$(perf_resolve_bool_01_env PERF_LADDER_MIR_HOTOPS 0)" || exit $?
LADDER_AOT_SENTINEL="$(perf_resolve_bool_01_env PERF_LADDER_AOT_SENTINEL "${AOT_SENTINEL_DEFAULT}")" || exit $?
LADDER_APPS="$(perf_resolve_bool_01_env PERF_LADDER_APPS 1)" || exit $?
LADDER_STABILITY="$(perf_resolve_bool_01_env PERF_LADDER_STABILITY 1)" || exit $?
LADDER_REGRESSION_GUARD="$(perf_resolve_bool_01_env PERF_LADDER_REGRESSION_GUARD "${REGRESSION_GUARD_DEFAULT}")" || exit $?

run_step "mir-shape contract (small perf benches)" \
  bash "${ROOT_DIR}/tools/smokes/v2/profiles/integration/apps/phase21_5_perf_mir_shape_contract_vm.sh"

run_step "direct-emit dominance fail-fast contract" \
  bash "${ROOT_DIR}/tools/smokes/v2/profiles/integration/apps/phase21_5_perf_direct_emit_dominance_block_vm.sh"

for key in "${SMALL_KEYS[@]}"; do
  run_step "small bench: ${key}" \
    env PERF_AOT="${LADDER_AOT_SMALL}" \
        NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
        bash "${ROOT_DIR}/tools/perf/bench_compare_c_vs_hako.sh" "${key}" "${SMALL_WARMUP}" "${SMALL_REPEAT}"
done

for key in "${MEDIUM_KEYS[@]}"; do
  run_step "medium bench: ${key}" \
    env PERF_AOT="${LADDER_AOT_MEDIUM}" \
        NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
        bash "${ROOT_DIR}/tools/perf/bench_compare_c_vs_hako.sh" "${key}" "${MEDIUM_WARMUP}" "${MEDIUM_REPEAT}"
done

if [[ "${LADDER_MIR_HOTOPS}" == "1" ]]; then
  MIR_HOTOPS_KEYS="${PERF_LADDER_MIR_HOTOPS_KEYS:-numeric_mixed_medium method_call_only_small}"
  for key in ${MIR_HOTOPS_KEYS}; do
    run_step "mir hotops: ${key}" \
      env NYASH_STAGE1_BINARY_ONLY_DIRECT="${NYASH_STAGE1_BINARY_ONLY_DIRECT:-1}" \
          PERF_MIR_SHAPE_TOP="${PERF_MIR_SHAPE_TOP:-12}" \
          bash "${ROOT_DIR}/tools/perf/report_mir_hotops.sh" "${key}"
  done
fi

# AOT sentinel (default: ON for default profile, OFF for quick)
if [[ "${LADDER_AOT_SENTINEL}" == "1" ]]; then
  run_step "AOT sentinel (numeric_mixed_medium)" \
    env PERF_AOT=1 NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
        bash "${ROOT_DIR}/tools/perf/bench_compare_c_vs_hako.sh" numeric_mixed_medium 1 1
fi

if [[ "${LADDER_APPS}" == "1" ]]; then
  run_step "apps wall-clock" \
    env NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
        bash "${ROOT_DIR}/tools/perf/bench_apps_wallclock.sh" "${APP_WARMUP}" "${APP_REPEAT}" vm
fi

if [[ "${LADDER_STABILITY}" == "1" ]]; then
  run_step "small baseline stability" \
    env NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" PERF_SUBTRACT_STARTUP=1 \
        bash "${ROOT_DIR}/tools/perf/record_baseline_stability_21_5.sh" \
        "${STAB_ROUNDS}" "${STAB_WARMUP}" "${STAB_REPEAT}"
fi

if [[ "${LADDER_REGRESSION_GUARD}" == "1" ]]; then
  run_step "regression guard (medium/app lock)" \
    env NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
        bash "${ROOT_DIR}/tools/smokes/v2/profiles/integration/apps/phase21_5_perf_regression_guard_contract_vm.sh"
fi

if [[ "${PROFILE}" == "default" ]]; then
  mkdir -p "$(dirname "${FULL_STATE_FILE}")"
  printf "%s %s\n" "$(date +%s)" "$(date -u '+%Y-%m-%dT%H:%M:%SZ')" > "${FULL_STATE_FILE}"
  echo "[ladder] full stamp updated: ${FULL_STATE_FILE}"
fi

echo "[ladder] done (profile=${PROFILE})"

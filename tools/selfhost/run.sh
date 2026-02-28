#!/bin/bash
# run.sh - unified selfhost entrypoint (thin dispatcher)
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
NYASH_ROOT="${NYASH_ROOT:-$ROOT_DIR}"
NYASH_BIN="${NYASH_BIN:-$NYASH_ROOT/target/release/hakorune}"

GATE_SCRIPT="$NYASH_ROOT/tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh"
DIRECT_STAGEB_SCRIPT="$NYASH_ROOT/tools/selfhost/run_stageb_compiler_vm.sh"
STEADY_STATE_SCRIPT="$NYASH_ROOT/tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_steady_state_vm.sh"

mode=""
source_file=""
runtime_input="apps/examples/string_p0.hako"
runtime_mode="stage-a"
timeout_secs=""
timeout_ms=""
max_cases=""
case_filter=""
planner_required="${HAKO_JOINIR_PLANNER_REQUIRED:-1}"
route_id=""
jobs=""
steady_with_runtime_parity="0"
steady_no_collect_blocker="0"
steady_quiet="0"
steady_cleanup_old_logs="0"

usage() {
  cat <<'USAGE' >&2
Usage:
  run.sh --gate [--max-cases <n>] [--filter <substring>] [--jobs <n>] [--timeout-secs <n>] [--planner-required 0|1]
  run.sh --steady-state [--with-runtime-parity] [--no-collect-blocker] [--quiet] [--cleanup-old-logs]
  run.sh --runtime [--runtime-mode <stage-a|exe>] [--input <file>] [--timeout-ms <n>] [--timeout-secs <n>]
  run.sh --direct --source-file <file> [--timeout-secs <n>] [--route-id <id>]

Examples:
  tools/selfhost/run.sh --gate --max-cases 5
  tools/selfhost/run.sh --gate --max-cases 20 --jobs 4
  tools/selfhost/run.sh --steady-state
  tools/selfhost/run.sh --steady-state --with-runtime-parity
  tools/selfhost/run.sh --steady-state --no-collect-blocker
  tools/selfhost/run.sh --steady-state --quiet
  tools/selfhost/run.sh --steady-state --cleanup-old-logs
  tools/selfhost/run.sh --runtime --input apps/examples/string_p0.hako
  tools/selfhost/run.sh --runtime --runtime-mode exe --input apps/examples/string_p0.hako
  tools/selfhost/run.sh --direct --source-file apps/tests/phase29bq_selfhost_cleanup_only_min.hako
USAGE
}

set_mode() {
  local next_mode="$1"
  if [ -n "$mode" ] && [ "$mode" != "$next_mode" ]; then
    echo "[selfhost/run] mode already selected: $mode" >&2
    exit 2
  fi
  mode="$next_mode"
}

while [ $# -gt 0 ]; do
  case "$1" in
    --gate)
      set_mode "gate"
      shift
      ;;
    --runtime)
      set_mode "runtime"
      shift
      ;;
    --steady-state)
      set_mode "steady-state"
      shift
      ;;
    --direct)
      set_mode "direct"
      shift
      ;;
    --with-runtime-parity)
      steady_with_runtime_parity="1"
      shift
      ;;
    --no-collect-blocker)
      steady_no_collect_blocker="1"
      shift
      ;;
    --quiet)
      steady_quiet="1"
      shift
      ;;
    --cleanup-old-logs)
      steady_cleanup_old_logs="1"
      shift
      ;;
    --source-file)
      [ $# -ge 2 ] || { usage; exit 2; }
      source_file="$2"
      shift 2
      ;;
    --input)
      [ $# -ge 2 ] || { usage; exit 2; }
      runtime_input="$2"
      shift 2
      ;;
    --runtime-mode)
      [ $# -ge 2 ] || { usage; exit 2; }
      runtime_mode="$2"
      shift 2
      ;;
    --timeout-secs)
      [ $# -ge 2 ] || { usage; exit 2; }
      timeout_secs="$2"
      shift 2
      ;;
    --timeout-ms)
      [ $# -ge 2 ] || { usage; exit 2; }
      timeout_ms="$2"
      shift 2
      ;;
    --max-cases)
      [ $# -ge 2 ] || { usage; exit 2; }
      max_cases="$2"
      shift 2
      ;;
    --filter)
      [ $# -ge 2 ] || { usage; exit 2; }
      case_filter="$2"
      shift 2
      ;;
    --planner-required)
      [ $# -ge 2 ] || { usage; exit 2; }
      planner_required="$2"
      shift 2
      ;;
    --jobs)
      [ $# -ge 2 ] || { usage; exit 2; }
      jobs="$2"
      shift 2
      ;;
    --route-id)
      [ $# -ge 2 ] || { usage; exit 2; }
      route_id="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[selfhost/run] unknown arg: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [ -z "$mode" ]; then
  echo "[selfhost/run] choose one mode: --gate | --steady-state | --runtime | --direct" >&2
  usage
  exit 2
fi

if [ "$mode" != "runtime" ] && [ "$runtime_mode" != "stage-a" ]; then
  echo "[selfhost/run] --runtime-mode is only valid with --runtime" >&2
  exit 2
fi

if [ "$mode" != "gate" ] && [ -n "$jobs" ]; then
  echo "[selfhost/run] --jobs is only valid with --gate" >&2
  exit 2
fi

if [ "$mode" != "steady-state" ] && [ "$steady_with_runtime_parity" = "1" ]; then
  echo "[selfhost/run] --with-runtime-parity is only valid with --steady-state" >&2
  exit 2
fi

if [ "$mode" != "steady-state" ] && [ "$steady_no_collect_blocker" = "1" ]; then
  echo "[selfhost/run] --no-collect-blocker is only valid with --steady-state" >&2
  exit 2
fi

if [ "$mode" != "steady-state" ] && [ "$steady_quiet" = "1" ]; then
  echo "[selfhost/run] --quiet is only valid with --steady-state" >&2
  exit 2
fi

if [ "$mode" != "steady-state" ] && [ "$steady_cleanup_old_logs" = "1" ]; then
  echo "[selfhost/run] --cleanup-old-logs is only valid with --steady-state" >&2
  exit 2
fi

if [ -n "$timeout_secs" ] && ! [[ "$timeout_secs" =~ ^[0-9]+$ ]]; then
  echo "[selfhost/run] --timeout-secs must be integer: $timeout_secs" >&2
  exit 2
fi

if [ -n "$timeout_ms" ] && ! [[ "$timeout_ms" =~ ^[0-9]+$ ]]; then
  echo "[selfhost/run] --timeout-ms must be integer: $timeout_ms" >&2
  exit 2
fi

resolve_path() {
  local candidate="$1"
  if [[ "$candidate" == /* ]]; then
    echo "$candidate"
  else
    echo "$NYASH_ROOT/$candidate"
  fi
}

run_gate() {
  if [ ! -f "$GATE_SCRIPT" ]; then
    echo "[selfhost/run] gate script not found: $GATE_SCRIPT" >&2
    exit 2
  fi

  if ! [[ "$planner_required" =~ ^[01]$ ]]; then
    echo "[selfhost/run] --planner-required must be 0 or 1: $planner_required" >&2
    exit 2
  fi

  if [ -n "$max_cases" ] && ! [[ "$max_cases" =~ ^[0-9]+$ ]]; then
    echo "[selfhost/run] --max-cases must be integer: $max_cases" >&2
    exit 2
  fi

  if [ -n "$jobs" ] && (! [[ "$jobs" =~ ^[0-9]+$ ]] || [ "$jobs" -lt 1 ]); then
    echo "[selfhost/run] --jobs must be integer >= 1: $jobs" >&2
    exit 2
  fi

  local -a env_prefix
  env_prefix=(
    "SMOKES_ENABLE_SELFHOST=1"
    "HAKO_JOINIR_PLANNER_REQUIRED=$planner_required"
  )
  if [ -z "$jobs" ]; then
    jobs="4"
  fi
  if [ -n "$timeout_secs" ]; then
    env_prefix+=("RUN_TIMEOUT_SECS=$timeout_secs")
  fi
  if [ -n "$max_cases" ]; then
    env_prefix+=("SMOKES_SELFHOST_MAX_CASES=$max_cases")
  fi
  if [ -n "$case_filter" ]; then
    env_prefix+=("SMOKES_SELFHOST_FILTER=$case_filter")
  fi
  if [ -n "$jobs" ]; then
    env_prefix+=("SMOKES_SELFHOST_JOBS=$jobs")
  fi

  echo "[selfhost/run] mode=gate script=$(basename "$GATE_SCRIPT") jobs=$jobs" >&2
  env "${env_prefix[@]}" bash "$GATE_SCRIPT"
}

run_runtime() {
  if [ ! -x "$NYASH_BIN" ]; then
    echo "[selfhost/run] nyash binary not found/executable: $NYASH_BIN" >&2
    exit 2
  fi

  local input_file
  input_file="$(resolve_path "$runtime_input")"
  if [ ! -f "$input_file" ]; then
    echo "[selfhost/run] runtime input not found: $input_file" >&2
    exit 2
  fi

  local -a env_prefix
  env_prefix=("NYASH_USE_NY_COMPILER=1" "NYASH_NY_COMPILER_USE_PY=0")
  case "$runtime_mode" in
    stage-a)
      env_prefix+=("NYASH_USE_NY_COMPILER_EXE=0")
      ;;
    exe)
      env_prefix+=("NYASH_USE_NY_COMPILER_EXE=1")
      ;;
    *)
      echo "[selfhost/run] --runtime-mode must be stage-a|exe (got: $runtime_mode)" >&2
      exit 2
      ;;
  esac
  if [ -n "$timeout_ms" ]; then
    env_prefix+=("NYASH_NY_COMPILER_TIMEOUT_MS=$timeout_ms")
  fi

  echo "[selfhost/run] mode=runtime runtime_mode=$runtime_mode input=$(basename "$input_file")" >&2
  if [ -n "$timeout_secs" ]; then
    env "${env_prefix[@]}" timeout "$timeout_secs" "$NYASH_BIN" --backend vm "$input_file"
  else
    env "${env_prefix[@]}" "$NYASH_BIN" --backend vm "$input_file"
  fi
}

run_direct() {
  if [ ! -f "$DIRECT_STAGEB_SCRIPT" ]; then
    echo "[selfhost/run] direct script not found: $DIRECT_STAGEB_SCRIPT" >&2
    exit 2
  fi

  if [ -z "$source_file" ]; then
    echo "[selfhost/run] --direct requires --source-file" >&2
    exit 2
  fi

  local source_path
  source_path="$(resolve_path "$source_file")"
  if [ ! -f "$source_path" ]; then
    echo "[selfhost/run] source file not found: $source_path" >&2
    exit 2
  fi

  local -a args
  args=(--source-file "$source_path")
  if [ -n "$timeout_secs" ]; then
    args+=(--timeout-secs "$timeout_secs")
  fi
  if [ -n "$route_id" ]; then
    args+=(--route-id "$route_id")
  fi

  echo "[selfhost/run] mode=direct source=$(basename "$source_path")" >&2
  bash "$DIRECT_STAGEB_SCRIPT" "${args[@]}"
}

run_steady_state() {
  if [ ! -f "$STEADY_STATE_SCRIPT" ]; then
    echo "[selfhost/run] steady-state script not found: $STEADY_STATE_SCRIPT" >&2
    exit 2
  fi

  local -a args
  args=()
  if [ "$steady_with_runtime_parity" = "1" ]; then
    args+=(--with-runtime-parity)
  fi
  if [ "$steady_no_collect_blocker" = "1" ]; then
    args+=(--no-collect-blocker)
  fi
  if [ "$steady_quiet" = "1" ]; then
    args+=(--quiet)
  fi
  if [ "$steady_cleanup_old_logs" = "1" ]; then
    args+=(--cleanup-old-logs)
  fi

  echo "[selfhost/run] mode=steady-state with_runtime_parity=$steady_with_runtime_parity no_collect_blocker=$steady_no_collect_blocker quiet=$steady_quiet cleanup_old_logs=$steady_cleanup_old_logs" >&2
  bash "$STEADY_STATE_SCRIPT" "${args[@]}"
}

case "$mode" in
  gate) run_gate ;;
  steady-state) run_steady_state ;;
  runtime) run_runtime ;;
  direct) run_direct ;;
  *)
    echo "[selfhost/run] internal error: unknown mode $mode" >&2
    exit 2
    ;;
esac

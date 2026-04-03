#!/usr/bin/env bash
# selfhost_run_routes.sh — route execution helpers for selfhost/run.sh
#
# Purpose:
# - Own the gate/runtime/direct/steady-state route bodies.
# - Keep selfhost/run.sh focused on parsing and thin dispatch only.

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

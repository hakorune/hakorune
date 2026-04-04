#!/usr/bin/env bash
# selfhost_run_routes.sh — route execution helpers for selfhost/run.sh
#
# Purpose:
# - Own the gate/runtime/direct/steady-state route bodies.
# - Keep selfhost/run.sh focused on parsing and thin dispatch only.

GATE_SCRIPT="${GATE_SCRIPT:-$NYASH_ROOT/tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh}"
PROOF_STAGEB_SCRIPT="$NYASH_ROOT/tools/selfhost/proof/run_stageb_compiler_vm.sh"
STEADY_STATE_SCRIPT="${STEADY_STATE_SCRIPT:-$NYASH_ROOT/tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_steady_state_vm.sh}"

resolve_path() {
  local candidate="$1"
  if [[ "$candidate" == /* ]]; then
    echo "$candidate"
  else
    echo "$NYASH_ROOT/$candidate"
  fi
}

emit_runtime_route_tag() {
  local mode="$1"
  local source="$2"
  if [ "$mode" = "stage-a" ] || [ "$mode" = "stage-a-compat" ]; then
    mode="stage-a-compat"
  fi
  echo "[selfhost/route] id=SH-RUNTIME-SELFHOST mode=$mode source=$source" >&2
}

canonical_runtime_route_name() {
  case "$1" in
    stage-a|stage-a-compat|compat)
      echo "compat"
      ;;
    exe|mainline|"")
      echo "mainline"
      ;;
    *)
      echo "$1"
      ;;
  esac
}

run_runtime_temp_mir_handoff() {
  # Shared helper body for the runtime temp-MIR handoff.
  # B1 introduced the body; B2 makes the exe route call it by default.
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

  local source_name
  source_name="$(basename "$input_file")"

  local tmp_mir tmp_emit_err
  tmp_mir="$(mktemp --suffix .runtime_temp_mir.json)"
  tmp_emit_err="$(mktemp --suffix .runtime_temp_mir.err)"

  local route_name
  route_name="$(canonical_runtime_route_name "$runtime_mode")"

  echo "[selfhost/run] mode=runtime runtime_route=$route_name runtime_mode=$runtime_mode input=$(basename "$input_file") handoff=temp-mir" >&2
  emit_runtime_route_tag "pipeline-entry" "$source_name"
  emit_runtime_route_tag "$runtime_mode" "$source_name"

  local emit_rc=0
  set +e
  if [ -n "$timeout_secs" ]; then
    timeout "$timeout_secs" \
      bash "$NYASH_ROOT/tools/selfhost/compat/run_stage1_cli.sh" --bin "$NYASH_BIN" emit mir-json "$input_file" \
      >"$tmp_mir" 2>"$tmp_emit_err"
  else
    bash "$NYASH_ROOT/tools/selfhost/compat/run_stage1_cli.sh" --bin "$NYASH_BIN" emit mir-json "$input_file" \
      >"$tmp_mir" 2>"$tmp_emit_err"
  fi
  emit_rc=$?
  set -e
  if [ "$emit_rc" -ne 0 ]; then
    echo "[selfhost/run] runtime temp-MIR emit failed (rc=$emit_rc)" >&2
    if [ -s "$tmp_emit_err" ]; then
      cat "$tmp_emit_err" >&2
    fi
    rm -f "$tmp_mir" "$tmp_emit_err" 2>/dev/null || true
    exit "$emit_rc"
  fi

  if [ ! -s "$tmp_mir" ]; then
    echo "[selfhost/run] runtime temp-MIR emit produced empty payload" >&2
    rm -f "$tmp_mir" "$tmp_emit_err" 2>/dev/null || true
    exit 1
  fi
  if ! grep -q '"functions"' "$tmp_mir"; then
    echo "[selfhost/run] runtime temp-MIR payload missing functions marker" >&2
    cat "$tmp_mir" >&2
    rm -f "$tmp_mir" "$tmp_emit_err" 2>/dev/null || true
    exit 1
  fi

  echo "[selfhost/run] mode=runtime runtime_route=$route_name runtime_mode=$runtime_mode handoff=mir-json-file" >&2

  local run_rc=0
  set +e
  if [ -n "$timeout_secs" ]; then
    timeout "$timeout_secs" \
      "$NYASH_BIN" --mir-json-file "$tmp_mir"
  else
    "$NYASH_BIN" --mir-json-file "$tmp_mir"
  fi
  run_rc=$?
  set -e

  rm -f "$tmp_mir" "$tmp_emit_err" 2>/dev/null || true
  return "$run_rc"
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
  local route_name
  route_name="$(canonical_runtime_route_name "$runtime_mode")"
  case "$runtime_mode" in
    stage-a-compat)
      # explicit compat-only keep: keep the fallback route narrow and non-growing
      env_prefix+=("NYASH_USE_NY_COMPILER_EXE=0")
      ;;
    exe)
      # mainline route: temp MIR handoff stays on the direct/core path
      env_prefix+=("NYASH_USE_NY_COMPILER_EXE=1")
      ;;
    *)
      echo "[selfhost/run] --runtime-route must resolve to mainline|compat (legacy mode alias: exe|stage-a-compat; got: $runtime_mode)" >&2
      exit 2
      ;;
  esac
  if [ -n "$timeout_ms" ]; then
    env_prefix+=("NYASH_NY_COMPILER_TIMEOUT_MS=$timeout_ms")
  fi

  if [ "$runtime_mode" = "stage-a-compat" ]; then
    echo "[selfhost/run] mode=runtime runtime_route=$route_name runtime_mode=$runtime_mode lane=compat-only input=$(basename "$input_file")" >&2
    emit_runtime_route_tag "stage-a-compat" "$(basename "$input_file")"
  else
    echo "[selfhost/run] mode=runtime runtime_route=$route_name runtime_mode=$runtime_mode lane=mainline input=$(basename "$input_file")" >&2
  fi
  if [ "$runtime_mode" = "exe" ]; then
    run_runtime_temp_mir_handoff
    return
  fi
  if [ -n "$timeout_secs" ]; then
    env "${env_prefix[@]}" timeout "$timeout_secs" "$NYASH_BIN" --backend vm "$input_file"
  else
    env "${env_prefix[@]}" "$NYASH_BIN" --backend vm "$input_file"
  fi
}

run_direct() {
  if [ ! -f "$PROOF_STAGEB_SCRIPT" ]; then
    echo "[selfhost/run] direct proof script not found: $PROOF_STAGEB_SCRIPT" >&2
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

  echo "[selfhost/run] mode=direct source=$(basename "$source_path") proof_only=1" >&2
  env NYASH_SELFHOST_STAGEB_PROOF_ONLY=1 bash "$PROOF_STAGEB_SCRIPT" "${args[@]}"
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

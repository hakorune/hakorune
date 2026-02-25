#!/bin/bash
# phase29y_binary_only_common.sh - shared helpers for phase29y binary-only smokes

set -euo pipefail

phase29y_binary_only_require_input_and_bin() {
  local smoke_name="$1"
  local input="$2"
  if [ ! -f "$input" ]; then
    test_fail "$smoke_name: fixture missing: $input"
    return 1
  fi
  if [ ! -x "$NYASH_BIN" ]; then
    test_fail "$smoke_name: binary missing: $NYASH_BIN"
    return 1
  fi
  return 0
}

phase29y_binary_only_prepare_workdir() {
  local input="$1"
  local prefix="$2"
  PHASE29Y_BINARY_ONLY_WORKDIR="$(mktemp -d "/tmp/${prefix}.XXXXXX")"
  cp "$input" "$PHASE29Y_BINARY_ONLY_WORKDIR/input.hako"
  cp "$NYASH_BIN" "$PHASE29Y_BINARY_ONLY_WORKDIR/hakorune"
  chmod +x "$PHASE29Y_BINARY_ONLY_WORKDIR/hakorune"
}

phase29y_binary_only_cleanup_workdir() {
  if [ -n "${PHASE29Y_BINARY_ONLY_WORKDIR:-}" ] && [ -d "${PHASE29Y_BINARY_ONLY_WORKDIR:-}" ]; then
    rm -rf "$PHASE29Y_BINARY_ONLY_WORKDIR"
  fi
}

phase29y_binary_only_run_in_workdir() {
  local run_timeout_secs="$1"
  local unset_nyash_root="$2"
  shift 2

  local -a env_flags=(
    NYASH_DISABLE_PLUGINS=1
    NYASH_STAGE1_BINARY_ONLY_DIRECT=1
    NYASH_STAGE1_BINARY_ONLY_RUN_DIRECT=1
    NYASH_VM_USE_FALLBACK=0
    NYASH_VM_HAKO_PREFER_STRICT_DEV=0
    NYASH_JOINIR_DEV=0
    NYASH_JOINIR_STRICT=0
    HAKO_JOINIR_STRICT=0
    HAKO_JOINIR_PLANNER_REQUIRED=0
  )
  local -a env_cmd=(env)
  if [ "$unset_nyash_root" = "1" ]; then
    env_cmd=(env -u NYASH_ROOT)
  fi

  set +e
  PHASE29Y_BINARY_ONLY_OUTPUT=$(
    cd "$PHASE29Y_BINARY_ONLY_WORKDIR" &&
      timeout "$run_timeout_secs" "${env_cmd[@]}" "${env_flags[@]}" ./hakorune "$@" 2>&1
  )
  PHASE29Y_BINARY_ONLY_RC=$?
  set -e
}

phase29y_binary_only_tail_output() {
  printf '%s\n' "${PHASE29Y_BINARY_ONLY_OUTPUT:-}" | tail -n 80 || true
}

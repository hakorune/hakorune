#!/usr/bin/env bash
# stage1_contract.sh — Stage1 CLI env contract helpers (SSOT)
#
# Purpose:
# - Keep Stage1 input/text contract resolution in one place.
# - Share the same env-injection contract across selfhost helpers.

_STAGE1_CONTRACT_TOOLS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
source "${_STAGE1_CONTRACT_TOOLS_DIR}/lib/program_json_v0_compat.sh"

stage1_contract_emit_marker() {
  local mode="$1"
  case "$mode" in
    emit-program)
      printf '%s' '"kind"[[:space:]]*:[[:space:]]*"Program"'
      ;;
    emit-mir|emit-mir-program)
      printf '%s' '"functions"[[:space:]]*:'
      ;;
    *)
      return 1
      ;;
  esac
}

stage1_contract_program_json_compat_mode() {
  printf '%s' 'emit-mir-program'
}

stage1_contract_program_json_compat_entry() {
  printf '%s' '__stage1_program_json__'
}

stage1_contract_artifact_kind() {
  local bin="$1"
  local meta="${bin}.artifact_kind"
  if [[ ! -f "$meta" ]]; then
    printf '%s' 'unknown'
    return 0
  fi
  local kind
  kind="$(awk -F= '$1=="artifact_kind"{print $2; exit}' "$meta" 2>/dev/null || true)"
  if [[ -z "$kind" ]]; then
    printf '%s' 'unknown'
    return 0
  fi
  printf '%s' "$kind"
}

stage1_contract_artifact_entry() {
  local bin="$1"
  local meta="${bin}.artifact_kind"
  if [[ ! -f "$meta" ]]; then
    printf '%s' 'unknown'
    return 0
  fi
  local entry
  entry="$(awk -F= '$1=="entry"{print substr($0, index($0, "=")+1); exit}' "$meta" 2>/dev/null || true)"
  if [[ -z "$entry" ]]; then
    printf '%s' 'unknown'
    return 0
  fi
  printf '%s' "$entry"
}

stage1_contract_artifact_is_reduced_stage1_cli() {
  local bin="$1"
  local kind entry
  kind="$(stage1_contract_artifact_kind "$bin")"
  entry="$(stage1_contract_artifact_entry "$bin")"
  [[ "$kind" == "stage1-cli" && "$entry" == */lang/src/runner/entry/stage1_cli_env_entry.hako ]]
}

# Shared mode decode for both live env wrappers and low-level compat probes.
# Keep mode -> emit-flag truth single-sourced here.
stage1_contract_emit_flags_for_mode() {
  local mode="$1"
  case "$mode" in
    emit-program)
      printf '%s\n' '1 0'
      ;;
    emit-mir|emit-mir-program)
      printf '%s\n' '0 1'
      ;;
    *)
      printf '%s\n' '0 0'
      ;;
  esac
}

stage1_contract_emit_stdout_has_marker() {
  local mode="$1"
  local stdout_file="$2"
  local marker
  marker="$(stage1_contract_emit_marker "$mode")" || return 1
  grep -Eq "${marker}" "$stdout_file"
}

stage1_contract_validate_emit_output() {
  local mode="$1"
  local stdout_file="$2"
  local stderr_file="$3"

  if stage1_contract_emit_stdout_has_marker "$mode" "$stdout_file"; then
    return 0
  fi

  echo "[stage1-contract/emit-invalid] mode=${mode} rc=0 but payload marker missing" >&2
  if [[ -s "$stderr_file" ]]; then
    cat "$stderr_file" >&2
  fi
  if [[ -s "$stdout_file" ]]; then
    echo "[stage1-contract/emit-invalid] stdout follows:" >&2
    cat "$stdout_file" >&2
  fi
  return 98
}

stage1_contract_report_emit_failure() {
  local mode="$1"
  local rc="$2"
  local stdout_file="$3"
  local stderr_file="$4"

  cat "$stdout_file"
  cat "$stderr_file" >&2
  if [[ ! -s "$stdout_file" && ! -s "$stderr_file" ]]; then
    echo "[stage1-contract/emit-failed] mode=${mode} rc=${rc} empty-output=1" >&2
  fi
}

stage1_contract_cleanup_exec_temp() {
  rm -f "${1:-}" "${2:-}" 2>/dev/null || true
}

stage1_contract_source_text() {
  local entry="$1"
  cat "$entry"
}

stage1_contract_repo_root() {
  if [[ -n "${NYASH_ROOT:-}" && -d "${NYASH_ROOT}" ]]; then
    printf "%s" "${NYASH_ROOT}"
    return 0
  fi
  local script_dir
  script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  (cd "${script_dir}/../../.." && pwd)
}

stage1_contract_normalize_stageb_list() {
  local raw="${1:-}"
  if [[ -z "${raw}" ]]; then
    printf ""
    return 0
  fi
  if [[ "${raw}" == *"|||"* ]]; then
    printf "%s" "${raw}"
    return 0
  fi
  # `local IFS='|||' ; echo "${arr[*]}"` joins by the first IFS char (`|`).
  # Stage1UsingResolverBox expects explicit "|||" delimiters.
  printf "%s" "${raw//|/|||}"
}

stage1_contract_export_stageb_module_env() {
  local root
  root="$(stage1_contract_repo_root)"
  export NYASH_ROOT="${NYASH_ROOT:-$root}"

  local env_lib="${root}/tools/smokes/v2/lib/env.sh"
  if [[ ! -f "${env_lib}" ]]; then
    return 0
  fi

  # Source helper functions only; keep smoke default exports disabled here.
  local prev_skip="${SMOKE_ENV_SKIP_EXPORTS-}"
  export SMOKE_ENV_SKIP_EXPORTS=1
  # shellcheck source=/dev/null
  source "${env_lib}"
  if [[ -z "${prev_skip}" ]]; then
    unset SMOKE_ENV_SKIP_EXPORTS
  else
    export SMOKE_ENV_SKIP_EXPORTS="${prev_skip}"
  fi

  if [[ -z "${HAKO_STAGEB_MODULES_LIST:-}" ]]; then
    local modules_list
    modules_list="$(collect_stageb_modules_list "$root" || true)"
    modules_list="$(stage1_contract_normalize_stageb_list "$modules_list")"
    if [[ -n "${modules_list}" ]]; then
      export HAKO_STAGEB_MODULES_LIST="${modules_list}"
    fi
  fi

  if [[ -z "${HAKO_STAGEB_MODULE_ROOTS_LIST:-}" ]]; then
    local module_roots_list
    module_roots_list="$(collect_stageb_module_roots_list "$root" || true)"
    module_roots_list="$(stage1_contract_normalize_stageb_list "$module_roots_list")"
    if [[ -n "${module_roots_list}" ]]; then
      export HAKO_STAGEB_MODULE_ROOTS_LIST="${module_roots_list}"
    fi
  fi
}

stage1_contract_export_runner_defaults() {
  export NYASH_NYRT_SILENT_RESULT="${NYASH_NYRT_SILENT_RESULT:-1}"
  export NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}"
  export NYASH_FILEBOX_MODE="${NYASH_FILEBOX_MODE:-core-ro}"
  export HAKO_MIR_BUILDER_METHODIZE="${HAKO_MIR_BUILDER_METHODIZE:-1}"
  export NYASH_MIR_UNIFIED_CALL="${NYASH_MIR_UNIFIED_CALL:-1}"
  export HAKO_SELFHOST_NO_DELEGATE="${HAKO_SELFHOST_NO_DELEGATE:-1}"
  export HAKO_MIR_BUILDER_DELEGATE="${HAKO_MIR_BUILDER_DELEGATE:-0}"
  export NYASH_STAGE1_EMIT_TIMEOUT_MS="${NYASH_STAGE1_EMIT_TIMEOUT_MS:-300000}"
  stage1_contract_export_stageb_module_env
}

stage1_contract_run_bin_with_env() {
  local bin="$1"
  local mode="$2"
  local entry="$3"
  local source_text_for_mode="$4"
  local emit_program_flag="$5"
  local emit_mir_flag="$6"
  local stdout_file="${7:-}"
  local stderr_file="${8:-}"
  # Exact mainline lock: stage1 env-route emit must not let caller env degrade
  # methodized/unified MIR back to legacy callsite dialect.
  local -a cmd_env=(
    "NYASH_NYRT_SILENT_RESULT=${NYASH_NYRT_SILENT_RESULT:-1}"
    "NYASH_DISABLE_PLUGINS=1"
    "NYASH_FILEBOX_MODE=core-ro"
    "HAKO_MIR_BUILDER_METHODIZE=1"
    "NYASH_MIR_UNIFIED_CALL=1"
    "HAKO_SELFHOST_NO_DELEGATE=${HAKO_SELFHOST_NO_DELEGATE:-1}"
    "HAKO_MIR_BUILDER_DELEGATE=${HAKO_MIR_BUILDER_DELEGATE:-0}"
    "NYASH_STAGE1_MODE=${mode}"
    "HAKO_STAGE1_MODE=${mode}"
    "STAGE1_EMIT_PROGRAM_JSON=${emit_program_flag}"
    "STAGE1_EMIT_MIR_JSON=${emit_mir_flag}"
    "HAKO_STAGE1_INPUT=${entry}"
    "NYASH_STAGE1_INPUT=${entry}"
    "STAGE1_SOURCE=${entry}"
    "STAGE1_INPUT=${entry}"
    "STAGE1_SOURCE_TEXT=${source_text_for_mode}"
    "NYASH_USE_STAGE1_CLI=1"
  )

  if [[ -n "${stdout_file}" ]]; then
    env "${cmd_env[@]}" "$bin" >"$stdout_file" 2>"$stderr_file"
    return $?
  fi

  env "${cmd_env[@]}" "$bin"
}

stage1_contract_exec_direct_emit_mode() {
  local bin="$1"
  local mode="$2"
  local entry="$3"
  local payload_file
  local stdout_file
  local stderr_file
  local rc=0

  payload_file="$(mktemp)"
  stdout_file="$(mktemp)"
  stderr_file="$(mktemp)"

  case "$mode" in
    emit-program)
      if program_json_v0_compat_emit_to_file "$bin" "$payload_file" "$entry" >/dev/null 2>"$stderr_file"; then
        :
      else
        rc=$?
        stage1_contract_report_emit_failure "$mode" "$rc" "$stdout_file" "$stderr_file"
        stage1_contract_cleanup_exec_temp "$payload_file" "$stdout_file"
        stage1_contract_cleanup_exec_temp "$stderr_file"
        return "$rc"
      fi
      ;;
    emit-mir|emit-mir-program)
      if "$bin" --emit-mir-json "$payload_file" "$entry" >/dev/null 2>"$stderr_file"; then
        :
      else
        rc=$?
        stage1_contract_report_emit_failure "$mode" "$rc" "$stdout_file" "$stderr_file"
        stage1_contract_cleanup_exec_temp "$payload_file" "$stdout_file"
        stage1_contract_cleanup_exec_temp "$stderr_file"
        return "$rc"
      fi
      ;;
    *)
      echo "[stage1-contract] unsupported direct emit mode: $mode" >&2
      stage1_contract_cleanup_exec_temp "$payload_file" "$stdout_file"
      stage1_contract_cleanup_exec_temp "$stderr_file"
      return 97
      ;;
  esac

  cat "$payload_file" >"$stdout_file"
  if stage1_contract_validate_emit_output "$mode" "$stdout_file" "$stderr_file"; then
    :
  else
    rc=$?
    stage1_contract_report_emit_failure "$mode" "$rc" "$stdout_file" "$stderr_file"
    stage1_contract_cleanup_exec_temp "$payload_file" "$stdout_file"
    stage1_contract_cleanup_exec_temp "$stderr_file"
    return "$rc"
  fi

  cat "$stdout_file"
  if [[ -s "$stderr_file" ]]; then
    cat "$stderr_file" >&2
  fi

  stage1_contract_cleanup_exec_temp "$payload_file" "$stdout_file"
  stage1_contract_cleanup_exec_temp "$stderr_file"
}

# Shared checked emit runner.
# Live wrappers and low-level probe helpers should both reuse this path so
# stdout/stderr validation stays single-sourced in shell space.
stage1_contract_exec_checked_mode() {
  local bin="$1"
  local mode="$2"
  local entry="$3"
  local source_text_for_mode="$4"
  local emit_program_flag="$5"
  local emit_mir_flag="$6"
  local tmp_stdout=""
  local tmp_stderr=""
  local rc=0

  if [[ "$emit_program_flag" -eq 1 || "$emit_mir_flag" -eq 1 ]]; then
    tmp_stdout="$(mktemp)"
    tmp_stderr="$(mktemp)"
  fi

  if [[ -n "$tmp_stdout" ]]; then
    if stage1_contract_run_bin_with_env \
      "$bin" \
      "$mode" \
      "$entry" \
      "$source_text_for_mode" \
      "$emit_program_flag" \
      "$emit_mir_flag" \
      "$tmp_stdout" \
      "$tmp_stderr"; then
      rc=0
    else
      rc=$?
    fi
    if [[ "$rc" -ne 0 ]]; then
      stage1_contract_report_emit_failure "$mode" "$rc" "$tmp_stdout" "$tmp_stderr"
      stage1_contract_cleanup_exec_temp "$tmp_stdout" "$tmp_stderr"
      return "$rc"
    fi
    stage1_contract_validate_emit_output "$mode" "$tmp_stdout" "$tmp_stderr" || {
      rc=$?
      stage1_contract_cleanup_exec_temp "$tmp_stdout" "$tmp_stderr"
      return "$rc"
    }
    cat "$tmp_stdout"
    cat "$tmp_stderr" >&2
    stage1_contract_cleanup_exec_temp "$tmp_stdout" "$tmp_stderr"
    return 0
  fi

  stage1_contract_run_bin_with_env \
    "$bin" \
    "$mode" \
    "$entry" \
    "$source_text_for_mode" \
    "$emit_program_flag" \
    "$emit_mir_flag" \
    ""
}

# Exact-only compat helper for the current live shell contract.
# Keep this as the sole explicit Program(JSON) compat helper entry.
stage1_contract_exec_program_json_compat() {
  local bin="$1"
  local program_json_text="$2"
  stage1_contract_exec_mode \
    "$bin" \
    "$(stage1_contract_program_json_compat_entry)" \
    "$program_json_text" \
    "$(stage1_contract_program_json_compat_mode)"
}

stage1_contract_exec_mode() {
  local bin="$1"
  local mode="$2"
  local entry="$3"
  local source_text="$4"
  local source_text_for_mode="$source_text"
  local emit_program_flag=0
  local emit_mir_flag=0
  local artifact_kind
  artifact_kind="$(stage1_contract_artifact_kind "$bin")"

  stage1_contract_export_runner_defaults

  read -r emit_program_flag emit_mir_flag < <(stage1_contract_emit_flags_for_mode "$mode")

  if [[ "$artifact_kind" == "stage1-cli" ]]; then
    case "$mode" in
      run)
        "$bin" run "$source_text_for_mode"
        return $?
        ;;
      emit-program|emit-mir|emit-mir-program)
        stage1_contract_exec_checked_mode \
          "$bin" \
          "$mode" \
          "$entry" \
          "$source_text_for_mode" \
          "$emit_program_flag" \
          "$emit_mir_flag"
        return $?
        ;;
    esac
  fi

  if [[ "$emit_program_flag" -eq 1 || "$emit_mir_flag" -eq 1 ]]; then
    stage1_contract_exec_direct_emit_mode "$bin" "$mode" "$entry"
    return $?
  fi

  stage1_contract_exec_checked_mode \
    "$bin" \
    "$mode" \
    "$entry" \
    "$source_text_for_mode" \
    "$emit_program_flag" \
    "$emit_mir_flag"
}

# Build-stage bootstrap capability SSOT.
# Keep the stage0 payload proof and reduced-artifact run liveness check in one
# shell helper so callers do not drift on the exact stage0/stage1 split.
stage1_contract_verify_stage1_cli_bootstrap_capability() {
  local bootstrap_bin="$1"
  local probe_source="$2"
  local reduced_bin="$3"
  local probe_text
  probe_text="$(stage1_contract_source_text "$probe_source")"

  if ! stage1_contract_exec_mode \
    "$bootstrap_bin" \
    "emit-program" \
    "$probe_source" \
    "$probe_text" >/dev/null 2>&1; then
    return 1
  fi

  if ! stage1_contract_exec_mode \
    "$bootstrap_bin" \
    "emit-mir" \
    "$probe_source" \
    "$probe_text" >/dev/null 2>&1; then
    return 2
  fi

  if ! stage1_contract_exec_mode \
    "$reduced_bin" \
    "run" \
    "$probe_source" \
    "$probe_text" >/dev/null 2>&1; then
    return 3
  fi

  return 0
}

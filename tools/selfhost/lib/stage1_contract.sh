#!/usr/bin/env bash
# stage1_contract.sh — Stage1 CLI env contract helpers (SSOT)
#
# Purpose:
# - Keep Stage1 input/program-json alias resolution in one place.
# - Share the same env-injection contract across selfhost helpers.

stage1_contract_emit_marker() {
  local mode="$1"
  case "$mode" in
    emit-program|emit_program_json|emit-program-json)
      printf '%s' '"kind"[[:space:]]*:[[:space:]]*"Program"'
      ;;
    emit-mir|emit_mir_json|emit-mir-json)
      printf '%s' '"functions"[[:space:]]*:'
      ;;
    *)
      return 1
      ;;
  esac
}

stage1_contract_emit_stdout_has_marker() {
  local mode="$1"
  local stdout_file="$2"
  local marker
  marker="$(stage1_contract_emit_marker "$mode")" || return 1
  grep -Eq "^[[:space:]]*\\{.*${marker}.*\\}[[:space:]]*$" "$stdout_file"
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
  export HAKO_SELFHOST_NO_DELEGATE="${HAKO_SELFHOST_NO_DELEGATE:-1}"
  export HAKO_MIR_BUILDER_DELEGATE="${HAKO_MIR_BUILDER_DELEGATE:-0}"
  stage1_contract_export_stageb_module_env
}

stage1_contract_run_bin_with_env() {
  local bin="$1"
  local mode="$2"
  local entry="$3"
  local source_text_for_mode="$4"
  local emit_program_flag="$5"
  local emit_mir_flag="$6"
  local program_json_path="${7:-}"
  local program_json_text="${8:-}"
  local stdout_file="${9:-}"
  local stderr_file="${10:-}"
  local -a cmd_env=(
    "NYASH_NYRT_SILENT_RESULT=${NYASH_NYRT_SILENT_RESULT:-1}"
    "NYASH_DISABLE_PLUGINS=1"
    "NYASH_FILEBOX_MODE=core-ro"
    "HAKO_SELFHOST_NO_DELEGATE=${HAKO_SELFHOST_NO_DELEGATE:-1}"
    "HAKO_MIR_BUILDER_DELEGATE=${HAKO_MIR_BUILDER_DELEGATE:-0}"
    "NYASH_USE_STAGE1_CLI=1"
    "NYASH_STAGE1_MODE=${mode}"
    "HAKO_STAGE1_MODE=${mode}"
    "STAGE1_EMIT_PROGRAM_JSON=${emit_program_flag}"
    "STAGE1_EMIT_MIR_JSON=${emit_mir_flag}"
    "HAKO_STAGE1_INPUT=${entry}"
    "NYASH_STAGE1_INPUT=${entry}"
    "STAGE1_SOURCE=${entry}"
    "STAGE1_INPUT=${entry}"
    "STAGE1_SOURCE_TEXT=${source_text_for_mode}"
  )

  if [[ -n "${program_json_path}" || -n "${program_json_text}" ]]; then
    cmd_env+=(
      "HAKO_STAGE1_PROGRAM_JSON=${program_json_path}"
      "NYASH_STAGE1_PROGRAM_JSON=${program_json_path}"
      "STAGE1_PROGRAM_JSON=${program_json_path}"
      "HAKO_STAGE1_PROGRAM_JSON_TEXT=${program_json_text}"
      "NYASH_STAGE1_PROGRAM_JSON_TEXT=${program_json_text}"
      "STAGE1_PROGRAM_JSON_TEXT=${program_json_text}"
    )
  fi

  if [[ -n "${stdout_file}" ]]; then
    env "${cmd_env[@]}" "$bin" >"$stdout_file" 2>"$stderr_file"
    return $?
  fi

  env "${cmd_env[@]}" "$bin"
}

stage1_contract_exec_mode() {
  local bin="$1"
  local mode="$2"
  local entry="$3"
  local source_text="$4"
  local program_json_path="${5:-}"
  local program_json_text=""
  local source_text_for_mode="$source_text"
  local emit_program_flag=0
  local emit_mir_flag=0
  local tmp_stdout=""
  local tmp_stderr=""
  local rc=0

  stage1_contract_export_runner_defaults

  case "$mode" in
    emit-program|emit_program_json|emit-program-json)
      emit_program_flag=1
      ;;
    emit-mir|emit_mir_json|emit-mir-json)
      emit_mir_flag=1
      ;;
  esac

  if [[ -n "$program_json_path" && "$mode" == "emit-mir" && -f "$program_json_path" ]]; then
    program_json_text="$(cat "$program_json_path")"
    source_text_for_mode="$program_json_text"
  fi

  if [[ "$emit_program_flag" -eq 1 || "$emit_mir_flag" -eq 1 ]]; then
    tmp_stdout="$(mktemp)"
    tmp_stderr="$(mktemp)"
  fi

  if [[ -n "$program_json_path" ]]; then
    if [[ -n "$tmp_stdout" ]]; then
      if stage1_contract_run_bin_with_env \
        "$bin" \
        "$mode" \
        "$entry" \
        "$source_text_for_mode" \
        "$emit_program_flag" \
        "$emit_mir_flag" \
        "$program_json_path" \
        "$program_json_text" \
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
      "$program_json_path" \
      "$program_json_text"
    return $?
  fi

  if [[ -n "$tmp_stdout" ]]; then
    if stage1_contract_run_bin_with_env \
      "$bin" \
      "$mode" \
      "$entry" \
      "$source_text_for_mode" \
      "$emit_program_flag" \
      "$emit_mir_flag" \
      "" \
      "" \
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
    "$emit_mir_flag"
}

stage1_contract_exec_legacy_emit_mir() {
  local bin="$1"
  local entry="$2"
  local source_text="$3"
  local program_json_path="${4:-}"
  local program_json_text=""
  local source_text_for_mode="$source_text"

  stage1_contract_export_runner_defaults

  if [[ -n "$program_json_path" && -f "$program_json_path" ]]; then
    program_json_text="$(cat "$program_json_path")"
    source_text_for_mode="$program_json_text"
  fi

  stage1_contract_run_bin_with_env \
    "$bin" \
    "emit-mir" \
    "$entry" \
    "$source_text_for_mode" \
    0 \
    1 \
    "$program_json_path" \
    "$program_json_text"
}

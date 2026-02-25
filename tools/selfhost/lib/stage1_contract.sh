#!/usr/bin/env bash
# stage1_contract.sh — Stage1 CLI env contract helpers (SSOT)
#
# Purpose:
# - Keep Stage1 input/program-json alias resolution in one place.
# - Share the same env-injection contract across selfhost helpers.

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
  stage1_contract_export_stageb_module_env
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

  if [[ -n "$program_json_path" ]]; then
    NYASH_NYRT_SILENT_RESULT="${NYASH_NYRT_SILENT_RESULT:-1}" \
      NYASH_DISABLE_PLUGINS=1 NYASH_FILEBOX_MODE=core-ro \
      NYASH_USE_STAGE1_CLI=1 \
      NYASH_STAGE1_MODE="$mode" HAKO_STAGE1_MODE="$mode" \
      STAGE1_EMIT_PROGRAM_JSON="$emit_program_flag" STAGE1_EMIT_MIR_JSON="$emit_mir_flag" \
      HAKO_STAGE1_INPUT="$entry" NYASH_STAGE1_INPUT="$entry" STAGE1_SOURCE="$entry" STAGE1_INPUT="$entry" \
      HAKO_STAGE1_PROGRAM_JSON="$program_json_path" NYASH_STAGE1_PROGRAM_JSON="$program_json_path" STAGE1_PROGRAM_JSON="$program_json_path" \
      HAKO_STAGE1_PROGRAM_JSON_TEXT="$program_json_text" NYASH_STAGE1_PROGRAM_JSON_TEXT="$program_json_text" STAGE1_PROGRAM_JSON_TEXT="$program_json_text" \
      STAGE1_SOURCE_TEXT="$source_text_for_mode" \
      "$bin"
    return $?
  fi

  NYASH_NYRT_SILENT_RESULT="${NYASH_NYRT_SILENT_RESULT:-1}" \
    NYASH_DISABLE_PLUGINS=1 NYASH_FILEBOX_MODE=core-ro \
    NYASH_USE_STAGE1_CLI=1 \
    NYASH_STAGE1_MODE="$mode" HAKO_STAGE1_MODE="$mode" \
    STAGE1_EMIT_PROGRAM_JSON="$emit_program_flag" STAGE1_EMIT_MIR_JSON="$emit_mir_flag" \
    HAKO_STAGE1_INPUT="$entry" NYASH_STAGE1_INPUT="$entry" STAGE1_SOURCE="$entry" STAGE1_INPUT="$entry" \
    STAGE1_SOURCE_TEXT="$source_text_for_mode" \
    "$bin"
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

  NYASH_NYRT_SILENT_RESULT="${NYASH_NYRT_SILENT_RESULT:-1}" \
    NYASH_DISABLE_PLUGINS=1 NYASH_FILEBOX_MODE=core-ro \
    NYASH_USE_STAGE1_CLI=1 \
    HAKO_STAGE1_MODE="emit-mir" NYASH_STAGE1_MODE="emit-mir" \
    STAGE1_EMIT_MIR_JSON=1 \
    STAGE1_EMIT_PROGRAM_JSON=0 \
    HAKO_STAGE1_INPUT="$entry" NYASH_STAGE1_INPUT="$entry" STAGE1_SOURCE="$entry" STAGE1_INPUT="$entry" \
    HAKO_STAGE1_PROGRAM_JSON="$program_json_path" NYASH_STAGE1_PROGRAM_JSON="$program_json_path" STAGE1_PROGRAM_JSON="$program_json_path" \
    HAKO_STAGE1_PROGRAM_JSON_TEXT="$program_json_text" NYASH_STAGE1_PROGRAM_JSON_TEXT="$program_json_text" STAGE1_PROGRAM_JSON_TEXT="$program_json_text" \
    STAGE1_SOURCE_TEXT="$source_text_for_mode" \
    "$bin"
}

#!/usr/bin/env bash
# compare_mode_common.sh
# Shared CLI parsing helpers for app mode-compare scripts.

perf_compare_parse_cli_args() {
  local warmup_var="$1"
  local repeat_var="$2"
  local backend_var="$3"
  local samples_var="$4"
  local json_lines_var="$5"
  shift 5

  local warmup="${!warmup_var}"
  local repeat="${!repeat_var}"
  local backend="${!backend_var}"
  local samples="${!samples_var}"
  local json_lines="${!json_lines_var}"
  local positional_idx=0

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --json-lines)
        if [[ $# -lt 2 ]]; then
          echo "[error] --json-lines requires count" >&2
          return 2
        fi
        json_lines=1
        samples="$2"
        shift 2
        ;;
      --)
        shift
        break
        ;;
      -*)
        echo "[error] unknown option: $1" >&2
        return 2
        ;;
      *)
        positional_idx=$(( positional_idx + 1 ))
        if [[ "$positional_idx" -eq 1 ]]; then
          warmup="$1"
        elif [[ "$positional_idx" -eq 2 ]]; then
          repeat="$1"
        elif [[ "$positional_idx" -eq 3 ]]; then
          backend="$1"
        else
          echo "[error] too many positional args: $1" >&2
          return 2
        fi
        shift
        ;;
    esac
  done

  printf -v "$warmup_var" '%s' "$warmup"
  printf -v "$repeat_var" '%s' "$repeat"
  printf -v "$backend_var" '%s' "$backend"
  printf -v "$samples_var" '%s' "$samples"
  printf -v "$json_lines_var" '%s' "$json_lines"
  return 0
}

perf_compare_compact_json_or_fail() {
  local mode="$1"
  local output="$2"
  local mode_expr="${3:-}"
  local expected_mode="${4:-}"

  if ! printf '%s\n' "${output}" | jq -e . >/dev/null 2>&1; then
    echo "[error] mode=${mode} output is not valid JSON" >&2
    echo "${output}" >&2
    return 1
  fi

  local compact total mode_actual
  compact="$(printf '%s\n' "${output}" | jq -c .)"
  total="$(printf '%s\n' "${compact}" | jq -r '.total_ms // -1')"
  if ! [[ "${total}" =~ ^[0-9]+$ ]] || [[ "${total}" -le 0 ]]; then
    echo "[error] mode=${mode} total_ms must be positive integer: ${total}" >&2
    echo "${compact}" >&2
    return 1
  fi

  if [[ -n "${mode_expr}" ]]; then
    mode_actual="$(printf '%s\n' "${compact}" | jq -r "${mode_expr}" 2>/dev/null || true)"
    if [[ "${mode_actual}" != "${expected_mode}" ]]; then
      echo "[error] mode=${mode} mode-field mismatch: ${mode_actual}" >&2
      echo "${compact}" >&2
      return 1
    fi
  fi

  if ! printf '%s\n' "${compact}" | jq -e '.cases | type == "object" and (keys | length) > 0' >/dev/null 2>&1; then
    echo "[error] mode=${mode} cases must be non-empty object" >&2
    echo "${compact}" >&2
    return 1
  fi
  if ! printf '%s\n' "${compact}" | jq -e '.cases | to_entries | all((.value|type=="number") and (.value > 0))' >/dev/null 2>&1; then
    echo "[error] mode=${mode} cases values must be positive numbers" >&2
    echo "${compact}" >&2
    return 1
  fi

  printf "%s\n" "${compact}"
}

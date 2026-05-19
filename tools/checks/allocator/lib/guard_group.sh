#!/usr/bin/env bash

bash_guard_group_iter() {
  local steps_file="$1"
  local callback="$2"
  local line_no=0
  local script=""
  local extra=""
  local include_file=""

  while IFS='|' read -r script extra || [[ -n "$script" || -n "$extra" ]]; do
    line_no=$((line_no + 1))
    if [[ -z "$script" || "${script:0:1}" == "#" ]]; then
      continue
    fi
    if [[ -n "$extra" ]]; then
      echo "[bash-guard-group] ERROR: ${steps_file}:${line_no}: unexpected extra field" >&2
      return 2
    fi
    if [[ "${script:0:1}" == "@" ]]; then
      include_file="${script#@}"
      if [[ ! -f "$include_file" ]]; then
        echo "[bash-guard-group] ERROR: ${steps_file}:${line_no}: missing include: ${include_file}" >&2
        return 2
      fi
      bash_guard_group_iter "$include_file" "$callback" || return $?
      continue
    fi
    if [[ ! -f "$script" ]]; then
      echo "[bash-guard-group] ERROR: ${steps_file}:${line_no}: missing script: ${script}" >&2
      return 2
    fi
    "$callback" "$script"
  done < "$steps_file"
}

bash_guard_group_list_one() {
  local script="$1"
  echo "  - ${script}"
}

bash_guard_group_run_one() {
  local script="$1"
  echo "[${BASH_GUARD_GROUP_TAG}] >>> ${script}"
  bash "$script"
}

bash_guard_group_list() {
  local tag="$1"
  local steps_file="$2"
  echo "[${tag}] steps:"
  bash_guard_group_iter "$steps_file" bash_guard_group_list_one
}

bash_guard_group_run() {
  local tag="$1"
  local steps_file="$2"
  BASH_GUARD_GROUP_TAG="$tag" bash_guard_group_iter "$steps_file" bash_guard_group_run_one
}

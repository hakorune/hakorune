#!/usr/bin/env bash

# bench_apps_wallclock case registry
#
# Input:
#   $1 ROOT_DIR
#   $2 FIXTURE_LOG path
#   $3 TMP_MIR path
# Output globals:
#   CASE_NAMES (indexed array)
#   CASE_APP (assoc array)
#   CASE_ENV_SPEC (assoc array, ';' separated env kv pairs)

declare -ar APPS_WALLCLOCK_CASE_NAMES=(
  "controlflow_probe"
  "gate_log_summarizer"
  "mir_shape_guard"
)

bench_apps_case_names() {
  printf '%s\n' "${APPS_WALLCLOCK_CASE_NAMES[@]}"
}

bench_apps_build_case_registry() {
  local root_dir="$1"
  local fixture_log="$2"
  local mir_shape_input="$3"

  CASE_NAMES=("${APPS_WALLCLOCK_CASE_NAMES[@]}")

  unset CASE_APP CASE_ENV_SPEC
  declare -gA CASE_APP
  declare -gA CASE_ENV_SPEC

  CASE_APP["controlflow_probe"]="${root_dir}/apps/tools/controlflow_probe/main.hako"
  CASE_APP["gate_log_summarizer"]="${root_dir}/apps/tools/gate_log_summarizer/main.hako"
  CASE_APP["mir_shape_guard"]="${root_dir}/apps/tools/mir_shape_guard/main.hako"

  CASE_ENV_SPEC["controlflow_probe"]=""
  CASE_ENV_SPEC["gate_log_summarizer"]="GATE_LOG_FILE=${fixture_log}"
  CASE_ENV_SPEC["mir_shape_guard"]="MIR_SHAPE_INPUT=${mir_shape_input};MIR_SHAPE_STRICT=1"
}

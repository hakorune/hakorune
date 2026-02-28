#!/bin/bash
# phase21_5_perf_gate_vm.sh
#
# Contract pin:
# - Phase 21.5 perf lane quick gate (low-cost bundle)
# - core fixed-order steps:
#   1) MIR shape contract
#   2) direct-emit dominance fail-fast contract
#   3) fast-regfile small-key contract
#   4) numeric_mixed_medium AOT sentinel contract
# - optional steps SSOT:
#   tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_optional_steps.tsv
#   - each row: <ENV_TOGGLE>\t<STEP_PATH>
#   - toggle semantics: 0=skip, 1=run, other=fail-fast

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/gate_steps.sh"
require_env || exit 2

OPTIONAL_STEPS_FILE="$(dirname "$0")/phase21_5_perf_gate_optional_steps.tsv"
if [ ! -f "$OPTIONAL_STEPS_FILE" ]; then
  test_fail "phase21_5_perf_gate_vm: optional steps file missing: $OPTIONAL_STEPS_FILE"
  exit 2
fi

declare -A EXECUTED_GATE_STEPS=()

run_gate_step_once() {
  local gate_name="$1"
  local step_path="$2"
  if [[ -n "${EXECUTED_GATE_STEPS[$step_path]+x}" ]]; then
    echo "[INFO] ${gate_name}: skip duplicate optional step ${step_path}"
    return 0
  fi
  run_gate_step "${gate_name}" "${step_path}"
  EXECUTED_GATE_STEPS["$step_path"]=1
}

run_gate_step_once "phase21_5_perf_gate_vm" "tools/smokes/v2/profiles/integration/apps/phase21_5_perf_mir_shape_contract_vm.sh"
run_gate_step_once "phase21_5_perf_gate_vm" "tools/smokes/v2/profiles/integration/apps/phase21_5_perf_direct_emit_dominance_block_vm.sh"
run_gate_step_once "phase21_5_perf_gate_vm" "tools/smokes/v2/profiles/integration/apps/phase21_5_perf_fast_regfile_contract_vm.sh"
run_gate_step_once "phase21_5_perf_gate_vm" "tools/smokes/v2/profiles/integration/apps/phase21_5_perf_numeric_mixed_medium_aot_contract_vm.sh"

while IFS=$'\t' read -r env_name step_path _rest; do
  env_name="${env_name%$'\r'}"
  step_path="${step_path%$'\r'}"
  if [[ -z "${env_name}" || "${env_name}" == \#* ]]; then
    continue
  fi
  if [[ -z "${step_path}" ]]; then
    test_fail "phase21_5_perf_gate_vm: invalid optional steps row (missing path) env=${env_name}"
    exit 2
  fi
  flag_value="${!env_name:-0}"
  if [[ "${flag_value}" == "1" ]]; then
    run_gate_step_once "phase21_5_perf_gate_vm" "${step_path}"
  elif [[ "${flag_value}" != "0" ]]; then
    test_fail "phase21_5_perf_gate_vm: invalid ${env_name}=${flag_value} (expected 0|1)"
    exit 2
  fi
done < "$OPTIONAL_STEPS_FILE"

test_pass "phase21_5_perf_gate_vm: PASS (phase21.5 quick contracts locked)"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require() {
  grep -Fq "$2" "$1" || {
    echo "[script-instance-box-transfer] missing '$2' in $1" >&2
    exit 1
  }
}

TRANSFER=src/mir/builder/normal_script_instance_box_transfer.rs
WINDOW=src/mir/resolved_semantics/shadow/script_root_window.rs
LIFECYCLE=src/mir/builder/normal_default_root_catalog_lifecycle.rs
RAW=src/mir/builder/calls/function_call_preflight_route.rs
PHYSICAL=src/mir/builder/normal_instance_constructor_admission.rs
DEMAND=src/mir/builder/normal_instance_constructor_demand_manifest.rs
DEMAND_LOAN=src/mir/builder/normal_instance_constructor_demand_loan.rs
SEMANTIC_SCOPE=src/mir/builder/normal_instance_constructor_semantic_scope.rs
RUNTIME_WORK=src/mir/builder/normal_script_runtime_work.rs
RUNTIME_DEMAND=src/mir/builder/normal_script_runtime_demand_manifest.rs
WORK_PLAN=src/mir/builder/program_root_work_plan.rs

require "$TRANSFER" 'package.selected_callable_sources().entries()'
require "$TRANSFER" 'package.instance_constructors().rows()'
require "$TRANSFER" 'ScriptInstanceBoxTransferIssueV1::MethodCoverage'
require "$TRANSFER" 'ScriptInstanceBoxTransferIssueV1::ConstructorCoverage'
require "$WINDOW" 'InstanceBoxSemanticOwner'
require "$LIFECYCLE" 'prepare_with_instance_box_transfers'
require "$RAW" 'is_brand_declared'
require "$PHYSICAL" 'ConstructorSourceIdV1'
require "$PHYSICAL" 'from_physical_cohort'
require "$PHYSICAL" 'validate_program'
require "$PHYSICAL" 'InstanceConstructorDemandRoleV1'
require "$PHYSICAL" 'demand_expectations'
require "$DEMAND" 'ImmediateDeclaration'
require "$DEMAND" 'ScriptRuntimePrefix'
require "$DEMAND" 'ScriptRuntimeFullLifecycle'
require "$DEMAND" 'validate_exact'
require "$DEMAND" 'duplicate-ticket'
require "$RUNTIME_WORK" 'normal_script_runtime_demand_manifest.rs'
require "$RUNTIME_DEMAND" 'constructor_demand_expectations'
require "$DEMAND_LOAN" 'InstanceConstructorDemandConsumptionV1'
require "$DEMAND_LOAN" 'ticket-reuse'
require "$SEMANTIC_SCOPE" 'CallableSemanticLoweringState::from_exact_source'
require "$TRANSFER" 'physical_constructor_demands_retain_one_parser_source_id'
require "$TRANSFER" '.source_id()'
require "$LIFECYCLE" 'prepare_with_instance_box_transfers_and_constructor_sources'
require "$WORK_PLAN" 'constructor_source_cohort'
require "$WORK_PLAN" 'issue_manifest_for_disposition'
require "$WORK_PLAN" 'constructor_demand_manifest'

if rg -n 'NormalInstanceConstructorSource(Key|BatchV1)::new\(' \
  "$PHYSICAL" "$WORK_PLAN" "$LIFECYCLE" "$TRANSFER"; then
  echo "[script-instance-box-transfer] legacy constructor source identity constructor remains" >&2
  exit 1
fi

for file in "$TRANSFER" "$WINDOW" "$LIFECYCLE" "$PHYSICAL" "$DEMAND" "$DEMAND_LOAN" "$SEMANTIC_SCOPE" "$RUNTIME_WORK" "$RUNTIME_DEMAND" "$WORK_PLAN"; do
  lines="$(wc -l < "$file")"
  if (( lines >= 760 )); then
    echo "[script-instance-box-transfer] source split required: $file has $lines lines" >&2
    exit 1
  fi
done

CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  normal_script_instance_box_transfer --lib
CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  normal_instance_constructor_admission --lib
CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  normal_instance_constructor_demand_loan --lib

echo "[script-instance-box-transfer] OK"

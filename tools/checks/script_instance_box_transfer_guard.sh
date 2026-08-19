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
require "$TRANSFER" 'physical_constructor_demands_retain_one_parser_source_id'
require "$TRANSFER" '.source_id()'
require "$LIFECYCLE" 'prepare_with_instance_box_transfers_and_constructor_sources'
require "$WORK_PLAN" 'constructor_source_cohort'

if rg -n 'NormalInstanceConstructorSource(Key|BatchV1)::new\(' \
  "$PHYSICAL" "$WORK_PLAN" "$LIFECYCLE" "$TRANSFER"; then
  echo "[script-instance-box-transfer] legacy constructor source identity constructor remains" >&2
  exit 1
fi

for file in "$TRANSFER" "$WINDOW" "$LIFECYCLE" "$PHYSICAL" "$WORK_PLAN"; do
  lines="$(wc -l < "$file")"
  if (( lines >= 760 )); then
    echo "[script-instance-box-transfer] source split required: $file has $lines lines" >&2
    exit 1
  fi
done

CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  normal_script_instance_box_transfer --lib

echo "[script-instance-box-transfer] OK"

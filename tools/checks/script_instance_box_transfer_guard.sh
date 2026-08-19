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

require "$TRANSFER" 'package.selected_callable_sources().entries()'
require "$TRANSFER" 'package.instance_constructors().rows()'
require "$TRANSFER" 'ScriptInstanceBoxTransferIssueV1::MethodCoverage'
require "$TRANSFER" 'ScriptInstanceBoxTransferIssueV1::ConstructorCoverage'
require "$WINDOW" 'InstanceBoxSemanticOwner'
require "$LIFECYCLE" 'prepare_with_instance_box_transfers'
require "$RAW" 'is_brand_declared'

for file in "$TRANSFER" "$WINDOW" "$LIFECYCLE"; do
  lines="$(wc -l < "$file")"
  if (( lines >= 760 )); then
    echo "[script-instance-box-transfer] source split required: $file has $lines lines" >&2
    exit 1
  fi
done

CARGO_BUILD_JOBS=4 cargo test --profile quick -q -p nyash-rust \
  normal_script_instance_box_transfer --lib

echo "[script-instance-box-transfer] OK"

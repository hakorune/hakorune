#!/bin/bash
set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../apps/phase29cc_wsm_cargo_test_common.sh"
require_env || exit 2

require_p10_doc_keywords() {
  local smoke_name="$1"
  local doc="$2"
  shift 2

  if [ ! -f "$doc" ]; then
    test_fail "${smoke_name}: lock doc missing"
    exit 1
  fi

  local needle
  for needle in "$@"; do
    if ! grep -Fq "$needle" "$doc"; then
      test_fail "${smoke_name}: missing keyword in lock doc: $needle"
      exit 1
    fi
  done
}

run_p10_contract_tests() {
  local cmd
  local filter
  for cmd in "$@"; do
    if [[ "$cmd" =~ ^cargo[[:space:]]+test[[:space:]]+--features[[:space:]]+wasm-backend[[:space:]]+([^[:space:]]+)[[:space:]]+--[[:space:]]+--nocapture$ ]]; then
      filter="${BASH_REMATCH[1]}"
      run_wsm_targeted_contract_test "$filter"
    else
      eval "$cmd"
    fi
  done
}

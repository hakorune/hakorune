#!/bin/bash
# phase29cc_wsm_g3_canvas_contract_common_vm.sh
# Shared entrypoint for WSM-G3 canvas contract smokes.

set -euo pipefail

if [ "$#" -ne 4 ]; then
  echo "usage: $0 <smoke_name> <extern_test> <runtime_test> <pass_message>" >&2
  exit 2
fi

SMOKE_NAME="$1"
EXTERN_TEST="$2"
RUNTIME_TEST="$3"
PASS_MESSAGE="$4"

source "$(dirname "$0")/../../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../../lib/wasm_g3_contract.sh"
require_env || exit 2

run_wasm_g3_contract_smoke \
  "$SMOKE_NAME" \
  "$EXTERN_TEST" \
  "$RUNTIME_TEST" \
  "$PASS_MESSAGE"

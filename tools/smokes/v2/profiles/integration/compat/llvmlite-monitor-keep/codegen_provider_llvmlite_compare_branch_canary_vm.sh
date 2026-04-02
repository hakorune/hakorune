#!/usr/bin/env bash
set -euo pipefail
# Purpose: provider stop-line via llvmlite keep compiles a 2-block v1 compare/branch module

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
# shellcheck source=/dev/null
source "$SCRIPT_DIR/_llvmlite_provider_stopline_common.sh"

TMP_MIR="$(mktemp --suffix .json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

bash "$ROOT/tools/selfhost/examples/gen_v1_compare_branch.sh" >"$TMP_MIR"

run_llvmlite_provider_stopline_case \
  "compat/llvmlite-monitor-keep/codegen_provider_llvmlite_compare_branch_canary_vm" \
  "$TMP_MIR"

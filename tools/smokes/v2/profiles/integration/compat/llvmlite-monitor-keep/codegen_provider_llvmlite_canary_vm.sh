#!/usr/bin/env bash
set -euo pipefail
# Purpose: provider stop-line via llvmlite keep returns an existing .o path

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
# shellcheck source=/dev/null
source "$SCRIPT_DIR/_llvmlite_provider_stopline_common.sh"

INPUT_MIR="$ROOT/apps/tests/hello_simple_llvm_native_probe_v1.mir.json"

run_llvmlite_provider_stopline_case \
  "compat/llvmlite-monitor-keep/codegen_provider_llvmlite_canary_vm" \
  "$INPUT_MIR"

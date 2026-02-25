#!/bin/bash
# Verify LLVMCompareInstructionBox.lower_compare requires self and returns JSON

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

if [ "${SMOKES_ENABLE_LLVM_SELF_PARAM:-0}" != "1" ]; then
  test_skip "self_param_compare" "opt-in (set SMOKES_ENABLE_LLVM_SELF_PARAM=1)" && exit 0
fi

test_self_param_compare() {
  local src='
using "lang/src/llvm_ir/instructions/compare.hako" as C
static box Main { method main() {
  local s = new LLVMCompareInstructionBox().lower_compare("<", 1, 2, 3)
  if s.contains("\"op\":\"compare\"") && s.contains("\"operation\":\"<\"") { print(1) } else { print(0) }
} }
'
  local out
  out=$(HAKO_ALLOW_USING_FILE=1 NYASH_ALLOW_USING_FILE=1 run_nyash_vm -c "$src" 2>&1)
  check_exact "1" "$out" "self_param_compare"
}

run_test "self_param_compare" test_self_param_compare

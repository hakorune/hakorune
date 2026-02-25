#!/bin/bash
# Verify LLVMRetInstructionBox.lower_return requires self and returns JSON

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

if [ "${SMOKES_ENABLE_LLVM_SELF_PARAM:-0}" != "1" ]; then
  test_skip "self_param_ret" "opt-in (set SMOKES_ENABLE_LLVM_SELF_PARAM=1)" && exit 0
fi

test_self_param_ret() {
  local src='
using "lang/src/llvm_ir/instructions/ret.hako" as R
static box Main { method main() {
  local s = new LLVMRetInstructionBox().lower_return(7)
  if s.contains("\"op\":\"ret\"") && s.contains("\"value\":7") { print(1) } else { print(0) }
} }
'
  local out
  out=$(HAKO_ALLOW_USING_FILE=1 NYASH_ALLOW_USING_FILE=1 run_nyash_vm -c "$src" 2>&1)
  check_exact "1" "$out" "self_param_ret"
}

run_test "self_param_ret" test_self_param_ret

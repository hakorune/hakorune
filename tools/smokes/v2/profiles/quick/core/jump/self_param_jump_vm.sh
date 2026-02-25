#!/bin/bash
# Verify LLVMJumpInstructionBox.lower_jump requires self and returns JSON

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

if [ "${SMOKES_ENABLE_LLVM_SELF_PARAM:-0}" != "1" ]; then
  test_skip "self_param_jump" "opt-in (set SMOKES_ENABLE_LLVM_SELF_PARAM=1)" && exit 0
fi

test_self_param_jump() {
  local src='
using "lang/src/llvm_ir/instructions/jump.hako" as J
static box Main { method main() {
  local s = new LLVMJumpInstructionBox().lower_jump(5)
  if s.contains("\"op\":\"jump\"") && s.contains("\"target\":5") { print(1) } else { print(0) }
} }
'
  local out
  out=$(HAKO_ALLOW_USING_FILE=1 NYASH_ALLOW_USING_FILE=1 run_nyash_vm -c "$src" 2>&1)
  check_exact "1" "$out" "self_param_jump"
}

run_test "self_param_jump" test_self_param_jump

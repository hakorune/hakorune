#!/bin/bash
# Verify LLVMConstInstructionBox.lower_const requires self (implicit receiver) and returns JSON

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

if [ "${SMOKES_ENABLE_LLVM_SELF_PARAM:-0}" != "1" ]; then
  test_skip "self_param_const" "opt-in (set SMOKES_ENABLE_LLVM_SELF_PARAM=1)" && exit 0
fi

test_self_param_const() {
  local src='
using "lang/src/llvm_ir/instructions/const.hako" as ConstInst
static box Main {
  method main() {
    local v = new MapBox(); v.set("type","i64"); v.set("value",42)
    local s = new LLVMConstInstructionBox().lower_const(1, v)
    if s.contains("\"op\":\"const\"") && s.contains("\"i64\"") { print(1) } else { print(0) }
  }
}
'
  local out
  out=$(HAKO_ALLOW_USING_FILE=1 NYASH_ALLOW_USING_FILE=1 run_nyash_vm -c "$src" 2>&1)
  check_exact "1" "$out" "self_param_const"
}

run_test "self_param_const" test_self_param_const

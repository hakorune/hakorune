#!/bin/bash
# Verify LLVMPhiInstructionBox.lower_phi requires self (implicit receiver) and returns JSON

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

if [ "${SMOKES_ENABLE_LLVM_SELF_PARAM:-0}" != "1" ]; then
  test_skip "self_param_phi" "opt-in (set SMOKES_ENABLE_LLVM_SELF_PARAM=1)" && exit 0
fi

test_self_param_phi() {
  local src='
using "lang/src/llvm_ir/instructions/phi.hako" as PhiInst
static box Main {
  method main() {
    local incoming = new ArrayBox()
    local item = new MapBox(); item.set("value", 1); item.set("block", 0); incoming.push(item)
    local phi = new LLVMPhiInstructionBox().lower_phi(5, incoming)
    # 判定: 生成文字列に op":"phi が含まれるか（最小）
    if phi.contains("\"op\":\"phi\"") { print(1) } else { print(0) }
  }
}
'
  local out
  out=$(HAKO_ALLOW_USING_FILE=1 NYASH_ALLOW_USING_FILE=1 run_nyash_vm -c "$src" 2>&1)
  check_exact "1" "$out" "self_param_phi"
}

run_test "self_param_phi" test_self_param_phi

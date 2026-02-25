#!/bin/bash
# Bridge(JSON v0) → MIR Interpreter での loop+continue 検証
# 既定は SKIP（opt-in: SMOKES_ENABLE_LOOP_BRIDGE=1）

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

if [ "${SMOKES_ENABLE_LOOP_BRIDGE:-0}" != "1" ]; then
  test_skip "bridge_loop_continue_vm" "opt-in (set SMOKES_ENABLE_LOOP_BRIDGE=1)" && exit 0
fi

test_bridge_loop_continue() {
  local tmp_json="/tmp/nyash_bridge_loop_continue_$$.json"
  cat > "$tmp_json" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Local","name":"i","expr":{"type":"Int","value":0}},
  {"type":"Local","name":"sum","expr":{"type":"Int","value":0}},
  {"type":"Loop",
   "cond":{"type":"Compare","op":"<","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":5}},
   "body":[
     {"type":"Local","name":"i","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":1}}},
     {"type":"If","cond":{"type":"Compare","op":"==","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":3}},
      "then":[{"type":"Continue"}],"else":[]},
     {"type":"Local","name":"sum","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"sum"},"rhs":{"type":"Var","name":"i"}}}
   ]
  },
  {"type":"Extern","iface":"env.console","method":"log","args":[{"type":"Var","name":"sum"}]}
]}
JSON
  local output last
  output=$(NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
           NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 "$NYASH_BIN" --backend vm --json-file "$tmp_json" 2>&1)
  last=$(printf '%s\n' "$output" | awk '/^[0-9-]+$/ {v=$0} END{if(v) print v}')
  rm -f "$tmp_json"
  check_exact "12" "${last:-$output}" "bridge_loop_continue"
}

run_test "bridge_loop_continue" test_bridge_loop_continue

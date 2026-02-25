#!/bin/bash
# Bridge(JSON v0) → MIR Interpreter での loop+break 検証
# 既定は SKIP（opt-in: SMOKES_ENABLE_LOOP_BRIDGE=1）

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

if [ "${SMOKES_ENABLE_LOOP_BRIDGE:-0}" != "1" ]; then
  test_skip "bridge_loop_break_vm" "opt-in (set SMOKES_ENABLE_LOOP_BRIDGE=1)" && exit 0
fi

test_bridge_loop_break() {
  local tmp_json="/tmp/nyash_bridge_loop_break_$$.json"
  cat > "$tmp_json" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Local","name":"i","expr":{"type":"Int","value":0}},
  {"type":"Local","name":"result","expr":{"type":"Int","value":0}},
  {"type":"Loop",
   "cond":{"type":"Compare","op":"<","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":100}},
   "body":[
     {"type":"If","cond":{"type":"Compare","op":"==","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":5}},
      "then":[{"type":"Break"}],"else":[]},
     {"type":"Local","name":"result","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"result"},"rhs":{"type":"Var","name":"i"}}},
     {"type":"Local","name":"i","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":1}}}
   ]
  },
  {"type":"Extern","iface":"env.console","method":"log","args":[{"type":"Var","name":"result"}]}
]}
JSON
  local output last
  output=$(NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
           NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 "$NYASH_BIN" --backend vm --json-file "$tmp_json" 2>&1)
  last=$(printf '%s\n' "$output" | awk '/^[0-9-]+$/ {v=$0} END{if(v) print v}')
  rm -f "$tmp_json"
  check_exact "10" "${last:-$output}" "bridge_loop_break"
}

run_test "bridge_loop_break" test_bridge_loop_break

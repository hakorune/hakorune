#!/bin/bash
# Direct(Builder) vs Bridge(JSON v0) parity — loop + break
# 既定は SKIP（opt-in: SMOKES_ENABLE_LOOP_COMPARE=1）

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

if [ "${SMOKES_ENABLE_LOOP_COMPARE:-0}" != "1" ]; then
  test_skip "compare_loop_break_vm" "opt-in (set SMOKES_ENABLE_LOOP_COMPARE=1)" && exit 0
fi

test_compare_loop_break() {
  # Direct (Nyash source)
  local src='
local i, result
i = 0
result = 0
loop(i < 100) {
  if i == 5 { break }
  result = result + i
  i = i + 1
}
print(result)
'
  local direct_out
  direct_out=$(run_nyash_vm -c "$src" 2>&1)

  # Bridge (JSON v0)
  local tmp_json="/tmp/nyash_compare_loop_break_$$.json"
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
  local bridge_out
  bridge_out=$(NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 "$NYASH_BIN" --backend vm --json-file "$tmp_json" 2>&1 | filter_noise)
  rm -f "$tmp_json"

  check_exact "10" "$direct_out" "direct_loop_break"
  check_exact "10" "$bridge_out" "bridge_loop_break"

  if [ "$direct_out" != "$bridge_out" ]; then
    echo "[FAIL] compare_loop_break: direct != bridge" >&2
    echo "  direct: $direct_out" >&2
    echo "  bridge: $bridge_out" >&2
    return 1
  fi
  return 0
}

run_test "compare_loop_break" test_compare_loop_break

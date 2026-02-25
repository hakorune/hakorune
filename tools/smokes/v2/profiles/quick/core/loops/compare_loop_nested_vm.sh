#!/bin/bash
# Direct(Builder) vs Bridge(JSON v0) parity — nested loops
# 既定は SKIP（opt-in: SMOKES_ENABLE_LOOP_COMPARE=1）

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

if [ "${SMOKES_ENABLE_LOOP_COMPARE:-0}" != "1" ]; then
  test_skip "compare_loop_nested_vm" "opt-in (set SMOKES_ENABLE_LOOP_COMPARE=1)" && exit 0
fi

test_compare_loop_nested() {
  local src='
local i, j, count
i = 0
count = 0
loop(i < 3) {
  j = 0
  loop(j < 2) {
    count = count + 1
    j = j + 1
  }
  i = i + 1
}
print(count)
'
  local direct_out
  direct_out=$(run_nyash_vm -c "$src" 2>&1)

  local tmp_json="/tmp/nyash_compare_loop_nested_$$.json"
  cat > "$tmp_json" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Local","name":"i","expr":{"type":"Int","value":0}},
  {"type":"Local","name":"j","expr":{"type":"Int","value":0}},
  {"type":"Local","name":"count","expr":{"type":"Int","value":0}},
  {"type":"Loop",
   "cond":{"type":"Compare","op":"<","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":3}},
   "body":[
     {"type":"Local","name":"j","expr":{"type":"Int","value":0}},
     {"type":"Loop",
      "cond":{"type":"Compare","op":"<","lhs":{"type":"Var","name":"j"},"rhs":{"type":"Int","value":2}},
      "body":[
        {"type":"Local","name":"count","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"count"},"rhs":{"type":"Int","value":1}}},
        {"type":"Local","name":"j","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"j"},"rhs":{"type":"Int","value":1}}}
      ]
     },
     {"type":"Local","name":"i","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":1}}}
   ]},
  {"type":"Extern","iface":"env.console","method":"log","args":[{"type":"Var","name":"count"}]}
]}
JSON
  local bridge_out
  bridge_out=$(NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 "$NYASH_BIN" --backend vm --json-file "$tmp_json" 2>&1 | filter_noise)
  rm -f "$tmp_json"

  check_exact "6" "$direct_out" "direct_loop_nested"
  check_exact "6" "$bridge_out" "bridge_loop_nested"

  if [ "$direct_out" != "$bridge_out" ]; then
    echo "[FAIL] compare_loop_nested: direct != bridge" >&2
    echo "  direct: $direct_out" >&2
    echo "  bridge: $bridge_out" >&2
    return 1
  fi
  return 0
}

run_test "compare_loop_nested" test_compare_loop_nested

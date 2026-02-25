#!/bin/bash
# Direct(Builder) vs Bridge(JSON v0) parity — loop + continue
# 既定は SKIP（opt-in: SMOKES_ENABLE_LOOP_COMPARE=1）

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

if [ "${SMOKES_ENABLE_LOOP_COMPARE:-0}" != "1" ]; then
  test_skip "compare_loop_continue_vm" "opt-in (set SMOKES_ENABLE_LOOP_COMPARE=1)" && exit 0
fi

test_compare_loop_continue() {
  # Direct (Nyash source)
  local src='
local i, sum
i = 0
sum = 0
loop(i < 5) {
  i = i + 1
  if i == 3 { continue }
  sum = sum + i
}
print(sum)
'
  local direct_out
  direct_out=$(run_nyash_vm -c "$src" 2>&1)

  # Bridge (JSON v0)
  local tmp_json="/tmp/nyash_compare_loop_continue_$$.json"
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
  local bridge_out
  bridge_out=$(NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 "$NYASH_BIN" --backend vm --json-file "$tmp_json" 2>&1 | filter_noise)
  rm -f "$tmp_json"

  check_exact "12" "$direct_out" "direct_loop_continue"
  check_exact "12" "$bridge_out" "bridge_loop_continue"

  if [ "$direct_out" != "$bridge_out" ]; then
    echo "[FAIL] compare_loop_continue: direct != bridge" >&2
    echo "  direct: $direct_out" >&2
    echo "  bridge: $bridge_out" >&2
    return 1
  fi
  return 0
}

run_test "compare_loop_continue" test_compare_loop_continue

#!/bin/bash
# Direct(Builder) vs Bridge(JSON v0) parity — mixed continue+break and two carriers
# 既定は SKIP（opt-in: SMOKES_ENABLE_LOOP_COMPARE=1）

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

if [ "${SMOKES_ENABLE_LOOP_COMPARE:-0}" != "1" ]; then
  test_skip "compare_loop_mixed_vm" "opt-in (set SMOKES_ENABLE_LOOP_COMPARE=1)" && exit 0
fi

test_compare_loop_mixed() {
  local src='
local i, sum, prod
i = 0
sum = 0
prod = 1
loop(i < 10) {
  if i == 2 { i = i + 1; continue }
  if i == 6 { break }
  sum = sum + i
  prod = prod * (i + 1)
  i = i + 1
}
print(sum)
print(prod)
'
  local direct_out
  direct_out=$(run_nyash_vm -c "$src" 2>&1)

  # Bridge JSON v0 (等価)
  local tmp_json="/tmp/nyash_compare_loop_mixed_$$.json"
  cat > "$tmp_json" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Local","name":"i","expr":{"type":"Int","value":0}},
  {"type":"Local","name":"sum","expr":{"type":"Int","value":0}},
  {"type":"Local","name":"prod","expr":{"type":"Int","value":1}},
  {"type":"Loop",
   "cond":{"type":"Compare","op":"<","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":10}},
   "body":[
     {"type":"If","cond":{"type":"Compare","op":"==","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":2}},
      "then":[
        {"type":"Local","name":"i","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":1}}},
        {"type":"Continue"}
      ],"else":[]},
     {"type":"If","cond":{"type":"Compare","op":"==","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":6}},
      "then":[{"type":"Break"}],"else":[]},
     {"type":"Local","name":"sum","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"sum"},"rhs":{"type":"Var","name":"i"}}},
     {"type":"Local","name":"prod","expr":{"type":"Binary","op":"*","lhs":{"type":"Var","name":"prod"},"rhs":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":1}}}} ,
     {"type":"Local","name":"i","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":1}}}
   ]
  },
  {"type":"Extern","iface":"env.console","method":"log","args":[{"type":"Var","name":"sum"}]},
  {"type":"Extern","iface":"env.console","method":"log","args":[{"type":"Var","name":"prod"}]}
]}
JSON
  local bridge_out
  bridge_out=$(NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 "$NYASH_BIN" --backend vm --json-file "$tmp_json" 2>&1 | filter_noise)
  rm -f "$tmp_json"

  # Direct/Bridgeの末尾2行（sum, prod）をそれぞれ比較
  local d_last1 d_last2 b_last1 b_last2
  d_last1=$(printf '%s\n' "$direct_out" | tail -n 2 | head -n1)
  d_last2=$(printf '%s\n' "$direct_out" | tail -n 1)
  b_last1=$(printf '%s\n' "$bridge_out" | tail -n 2 | head -n1)
  b_last2=$(printf '%s\n' "$bridge_out" | tail -n 1)

  check_exact "$d_last1" "$b_last1" "compare_loop_mixed_sum" || return 1
  check_exact "$d_last2" "$b_last2" "compare_loop_mixed_prod" || return 1
  return 0
}

run_test "compare_loop_mixed" test_compare_loop_mixed

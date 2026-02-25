#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../../.." && pwd)
source "$SCRIPT_DIR/common.sh"

pass() { echo "✅ $1" >&2; }
fail() { echo "❌ $1" >&2; echo "$2" >&2; exit 1; }

run_exit_file() {
  local file="$1" code
  set +e
  pyvm_run_source_capture "$file" >/dev/null 2>&1
  code=$?
  set -e
  echo "$code"
}

# 1) String ops baseline
CODE=$(run_exit_file "$ROOT_DIR/apps/tests/string_ops_basic.hako")
[[ "$CODE" -eq 0 ]] && pass "PyVM: string ops baseline (exit=0)" || fail "PyVM: string ops baseline" "__EXIT_CODE__=$CODE"

# 2) me.method() baseline
CODE=$(run_exit_file "$ROOT_DIR/apps/tests/me_method_call.hako")
[[ "$CODE" -eq 0 ]] && pass "PyVM: me method baseline (exit=0)" || fail "PyVM: me method baseline" "__EXIT_CODE__=$CODE"

# 3) If/Loop + PHI baseline
CODE=$(run_exit_file "$ROOT_DIR/apps/tests/loop_if_phi.hako")
[[ "$CODE" -eq 0 ]] && pass "PyVM: loop/if/phi baseline (exit=0)" || fail "PyVM: loop/if/phi baseline" "__EXIT_CODE__=$CODE"

# 4) Ternary basic
CODE=$(run_exit_file "$ROOT_DIR/apps/tests/ternary_basic.hako")
[[ "$CODE" -eq 10 ]] && pass "PyVM: ternary basic (exit=10)" || fail "PyVM: ternary basic" "__EXIT_CODE__=$CODE"

# 5) Ternary nested
CODE=$(run_exit_file "$ROOT_DIR/apps/tests/ternary_nested.hako")
[[ "$CODE" -eq 0 ]] && pass "PyVM: ternary nested baseline (exit=0)" || fail "PyVM: ternary nested baseline" "__EXIT_CODE__=$CODE"

# 6) Match expr block
CODE=$(run_exit_file "$ROOT_DIR/apps/tests/peek_expr_block.hako")
[[ "$CODE" -eq 1 ]] && pass "PyVM: match expr block (exit=1)" || fail "PyVM: match expr block" "__EXIT_CODE__=$CODE"

# 7) Match guard (type)
CODE=$(run_exit_file "$ROOT_DIR/apps/tests/match_guard_type_basic.hako")
[[ "$CODE" -eq 0 ]] && pass "PyVM: match guard type baseline (exit=0)" || fail "PyVM: match guard type baseline" "__EXIT_CODE__=$CODE"

# 8) Match guard (literal)
CODE=$(run_exit_file "$ROOT_DIR/apps/tests/match_guard_lit_basic.hako")
[[ "$CODE" -eq 0 ]] && pass "PyVM: match guard lit baseline (exit=0)" || fail "PyVM: match guard lit baseline" "__EXIT_CODE__=$CODE"

echo "All PyVM Stage-2 smokes PASS" >&2

#!/usr/bin/env bash
set -euo pipefail
[[ "${NYASH_CLI_VERBOSE:-0}" == "1" ]] && set -x

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
BIN="$ROOT_DIR/target/release/hakorune"

if [[ ! -x "$BIN" ]]; then
  (cd "$ROOT_DIR" && cargo build --release >/dev/null)
fi

TMP="$ROOT_DIR/tmp"
mkdir -p "$TMP"

: "${NYASH_NY_COMPILER_TIMEOUT_MS:=10000}"
export NYASH_NY_COMPILER_TIMEOUT_MS

pass() { echo "✅ $1" >&2; }
fail() { echo "❌ $1" >&2; echo "$2" >&2; exit 1; }

run_case_expect() {
  local name="$1"; shift
  local src="$1"; shift
  local regex="$1"; shift
  local file="$TMP/selfhost_${name}.hako"
  printf "%s\n" "$src" > "$file"
  set +e
  OUT=$(NYASH_USE_NY_COMPILER=1 NYASH_NY_COMPILER_EMIT_ONLY=0 \
        "$BIN" --backend vm "$file" 2>&1)
  RC=$?
  set -e
  if echo "$OUT" | rg -q "$regex"; then
    pass "$name"
    return
  fi

  local expect_code=""
  if [[ "$regex" =~ true ]]; then
    expect_code=1
  elif [[ "$regex" =~ false ]]; then
    expect_code=0
  else
    local num
    num=$(printf '%s' "$regex" | sed -E 's/.*([^0-9]|^)([0-9]+)([^0-9]|$).*/\2/;t;d' || true)
    if [[ -n "$num" ]]; then
      expect_code="$num"
    fi
  fi
  if [[ -n "$expect_code" && "$RC" == "$expect_code" ]]; then
    pass "$name"
  else
    fail "$name" "$OUT"
  fi
}

# A) arithmetic
run_case_expect "arith" 'return 1+2*3' '^Result:\s*7\b'

# B) unary minus precedence (-3 + 5 -> 2)
run_case_expect "unary" 'return -3 + 5' '^Result:\s*2\b'

# C) logical AND
run_case_expect "and" 'return (1 < 2) && (2 < 3)' '^Result:\s*true\b'

# D) logical OR
run_case_expect "or" 'return (1 > 2) || (2 < 3)' '^Result:\s*true\b'

# E) compare eq
run_case_expect "eq" 'return (1 + 1) == 2' '^Result:\s*true\b'

# F) nested if with locals -> 200
cat > "$TMP/selfhost_nested_if.hako" <<'NY'
if 1 < 2 {
  if 2 < 1 {
    return 100
  } else {
    return 200
  }
} else {
  return 300
}
NY
set +e
OUT=$(NYASH_USE_NY_COMPILER=1 NYASH_NY_COMPILER_EMIT_ONLY=0 \
      "$BIN" --backend vm "$TMP/selfhost_nested_if.hako" 2>&1)
RC=$?
set -e
[[ "$RC" -eq 200 ]] && pass "nested if" || fail "nested if" "$OUT"

# G) if/else separated by newline
cat > "$TMP/selfhost_if_else_line.hako" <<'NY'
if 1 < 2 {
  return 10
}
else {
  return 20
}
NY
set +e
OUT=$(NYASH_USE_NY_COMPILER=1 NYASH_NY_COMPILER_EMIT_ONLY=0 \
      "$BIN" --backend vm "$TMP/selfhost_if_else_line.hako" 2>&1)
RC=$?
set -e
[[ "$RC" -eq 10 ]] && pass "if/else separation" || fail "if/else separation" "$OUT"

# J) ternary expression → 10
cat > "$TMP/selfhost_ternary_basic.hako" <<'NY'
return (1 < 2) ? 10 : 20
NY
set +e
NYASH_USE_NY_COMPILER=1 NYASH_NY_COMPILER_EMIT_ONLY=0 \
      "$BIN" --backend vm "$TMP/selfhost_ternary_basic.hako" >/dev/null 2>&1
CODE=$?
set -e
if [[ "$CODE" -eq 10 ]]; then
  pass "Ternary basic"
else
  fail "Ternary basic" "exit=$CODE"
fi

# K) peek expression → 1
cat > "$TMP/selfhost_peek_basic.hako" <<'NY'
local d = "dog"
local v = peek d {
  "cat" => { 0 }
  "dog" => { 1 }
  else => { 0 }
}
return v
NY
set +e
NYASH_USE_NY_COMPILER=1 NYASH_NY_COMPILER_EMIT_ONLY=0 \
      "$BIN" --backend vm "$TMP/selfhost_peek_basic.hako" >/dev/null 2>&1
CODE=$?
set -e
if [[ "$CODE" -eq 1 ]]; then
  pass "Peek basic"
else
  fail "Peek basic" "exit=$CODE"
fi

echo "All selfhost Stage-2 smokes PASS" >&2
exit 0

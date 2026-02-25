#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
BIN="$ROOT_DIR/target/release/hakorune"

if [[ ! -x "$BIN" ]]; then
  echo "[build] nyash (release) ..." >&2
  (cd "$ROOT_DIR" && cargo build --release >/dev/null)
fi

TMP_DIR="$ROOT_DIR/tmp"
mkdir -p "$TMP_DIR"

pass() { echo "✅ $1" >&2; }
fail() { echo "❌ $1" >&2; echo "$2" | sed -n '1,160p' >&2; exit 1; }

run_bridge() {
  # Use Stage-2 Python MVP parser → JSON v0 → bridge pipe
  local src="$1"
  local out code
  printf '%s\n' "$src" > "$TMP_DIR/stage2_tmp.ny"
  set +e
  out=$(python3 "$ROOT_DIR/tools/ny_parser_mvp.py" "$TMP_DIR/stage2_tmp.ny" | "$BIN" --ny-parser-pipe 2>&1)
  code=$?
  set -e
  printf '%s\n__EXIT_CODE__=%s\n' "$out" "$code"
}

# 1) AND: LHS false → RHS not evaluated
SRC=$'local c = new ConsoleBox()\nreturn (1>2) && (c.println("rhs") == 0)'
OUT=$(run_bridge "$SRC")
echo "$OUT" | rg -q '^__EXIT_CODE__=0$' \
  && ! echo "$OUT" | rg -q '^rhs$' \
  && pass "shortcircuit: AND skips RHS" || fail "shortcircuit: AND skips RHS" "$OUT"

# 2) OR: LHS true → RHS not evaluated
SRC=$'local c = new ConsoleBox()\nreturn (1<2) || (c.println("rhs") == 0)'
OUT=$(run_bridge "$SRC")
echo "$OUT" | rg -q '^__EXIT_CODE__=1$' \
  && ! echo "$OUT" | rg -q '^rhs$' \
  && pass "shortcircuit: OR skips RHS" || fail "shortcircuit: OR skips RHS" "$OUT"

echo "All Stage-2 short-circuit (skip RHS) smokes PASS" >&2

# Nested short-circuit (no side effects) via pipe bridge
SRC=$'return (1 < 2) && ((1 > 2) || (2 < 3))'
OUT=$(run_bridge "$SRC")
echo "$OUT" | rg -q '^__EXIT_CODE__=1$' \
  && pass "shortcircuit: nested AND/OR (pipe bridge)" || fail "shortcircuit: nested" "$OUT"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../.." && pwd)"
BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"

if [ ! -x "$BIN" ]; then
  echo "[SMOKE] build missing: $BIN" >&2
  echo "Run: cargo build --release" >&2
  exit 2
fi

tmp_hako="/tmp/argv_multiline_$$.hako"
cat >"$tmp_hako" <<'HAKO'
static box Main {
  method main(args) {
    // args: [<file>, <text>], or custom — we check presence of newline in any arg
    local n = args.size();
    local i = 0; local has_nl = 0
    while i < n {
      local s = args.get(i)
      if s.indexOf("\n") >= 0 { has_nl = 1 }
      i = i + 1
    }
    // return 0 if newline preserved, else 1
    return (has_nl == 1) ? 0 : 1
  }
}
HAKO

multiline="line1\nline2\nline3"

set +e
NYASH_JSON_ONLY=1 NYASH_FEATURES=stage3 "$BIN" --backend vm "$tmp_hako" -- --source-file "/dev/null" "$multiline"
rc=$?
set -e

rm -f "$tmp_hako"

if [ "$rc" -ne 0 ]; then
  echo "[SMOKE/FAIL] argv_multiline_roundtrip: rc=$rc" >&2
  exit 1
else
  echo "[SMOKE/OK] argv_multiline_roundtrip"
fi

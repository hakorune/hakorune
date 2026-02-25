#!/bin/bash
# string_vm_api_canary_vm.sh — Verify VM String API parity with Rust

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp="/tmp/string_vm_api_canary_$$.hako"
cat > "$tmp" <<'HAKO'
static box Main { method main(args) {
  local s = "hello"
  // substring(start,end)
  local a = s.substring(1,4)  // "ell"
  if (""+a) != "ell" { return 0 }

  // indexOf(search)
  local p1 = s.indexOf("l")
  if (""+p1) != "2" { return 0 }
  local p2 = s.indexOf("z")
  if (""+p2) != "-1" { return 0 }

  // charAt via substring(i,i+1)
  local c = s.substring(0,1)
  if (""+c) != "h" { return 0 }
  return 1
} }
HAKO

set +e
out="$(NYASH_PREINCLUDE=0 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 run_nyash_vm "$tmp" 2>&1)"; rc=$?
set -e
rm -f "$tmp" || true

if [ "$rc" -eq 1 ]; then echo "[PASS] string_vm_api_canary_vm"; exit 0; fi
echo "$out" >&2
echo "[FAIL] string_vm_api_canary_vm (rc=$rc)" >&2; exit 1

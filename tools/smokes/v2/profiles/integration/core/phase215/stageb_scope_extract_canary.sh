#!/usr/bin/env bash
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || (cd "$SCRIPT_DIR/../../../../../.." && pwd))"
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh" || true

require_env || { echo "[SKIP] env not ready"; exit 0; }

test_stageb_scope_extract() {
  local SRC='static box Main { method main(args) { local x = 0 { if (1==1) { x = 42 } } return x } }'
  local out
  set +e
  out=$(NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 "$NYASH_BIN" --backend vm "$ROOT_DIR/lang/src/compiler/entry/compiler_stageb.hako" -- --source "$SRC" 2>/dev/null)
  local rc=$?
  set -e
  if [[ $rc -ne 0 ]]; then
    echo "[FAIL] stageb_scope_extract_canary: runner rc=$rc"
    return 1
  fi
  echo "$out" | grep -q '"kind":"Program"' || { echo "[FAIL] stageb_scope_extract_canary: no Program JSON"; return 1; }
  echo "$out" | grep -q '42' || { echo "[FAIL] stageb_scope_extract_canary: literal 42 not found"; return 1; }
  echo "[PASS] stageb_scope_extract_canary"; return 0
}

run_test "stageb_scope_extract_canary" test_stageb_scope_extract

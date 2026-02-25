#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../../../../../../.." && pwd)
BIN="$ROOT_DIR/target/release/hakorune"

if [ ! -x "$BIN" ]; then
  cargo build --release >/dev/null
fi

# Activate prod profile (path using should error)
set +u
source "$ROOT_DIR/tools/dev_env.sh" prod >/dev/null 2>&1 || true
set -u

TMP=$(mktemp)
cat >"$TMP" <<'NY'
using "./basic_test.hako" as Basic
static box Main { method main(args) { return 0 } }
NY

set +e
OUT=$("$BIN" --backend vm "$TMP" 2>&1)
RC=$?
set -e

rm -f "$TMP" || true

if [ $RC -eq 0 ]; then
  echo "[FAIL] prod_disallow_using_canary_vm: path using did not fail" >&2
  echo "$OUT" >&2
  exit 1
fi

# Expect an error message about using/AST disabled or using not allowed
echo "$OUT" | grep -Eq "using: .*disabled|using.*not allowed|prelude merge is disabled|not found in nyash.toml|file paths are disallowed" || {
  echo "[FAIL] expected using error message; got:" >&2
  echo "$OUT" >&2
  exit 1
}

echo "[PASS] prod_disallow_using_canary_vm"

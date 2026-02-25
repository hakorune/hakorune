#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../../../../../../.." && pwd)
BIN="$ROOT_DIR/target/release/hakorune"

if [ ! -x "$BIN" ]; then
  cargo build --release >/dev/null
fi

TMP=$(mktemp)
cat >"$TMP" <<'NY'
using hv1.dispatch as Dispatcher
static box Main { method main(args) { return 0 } }
NY

# Minimal MIR v0: return 9
JSON='{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"int","value":9}},{"op":"ret","value":1}]}]}]}'

set +e
OUT=$(HAKO_VERIFY_PRIMARY=hakovm NYASH_VERIFY_JSON="$JSON" "$BIN" --backend vm "$TMP" 2>&1)
RC=$?
set -e

rm -f "$TMP" || true

if [ $RC -ne 9 ]; then
  echo "[FAIL] hv1_inline_alias_only_canary: expected rc=9, got rc=$RC" >&2
  echo "$OUT" >&2
  exit 1
fi

echo "$OUT" | grep -q 'UnifiedBoxRegistry' && {
  echo "[FAIL] hv1-inline alias: plugin init leaked" >&2
  echo "$OUT" >&2
  exit 1
}

echo "$OUT" | tail -n1 | grep -qx '9' || {
  echo "[FAIL] hv1-inline alias: stdout tail not '9'" >&2
  echo "$OUT" >&2
  exit 1
}

echo "[PASS] hv1_inline_alias_only_canary_vm"


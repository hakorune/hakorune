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

JSON='{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"int","value":33}},{"op":"ret","value":1}]}]}]}'

set +e
OUT0=$(HAKO_VERIFY_PRIMARY=hakovm NYASH_VERIFY_JSON="$JSON" "$BIN" --backend vm "$TMP" 2>&1)
RC0=$?
OUT1=$(HAKO_VERIFY_PRIMARY=hakovm HAKO_V1_DISPATCHER_FLOW=1 HAKO_V1_TYPED_IR_PRIMARY=1 HAKO_V1_TYPED_IR_SHADOW=1 NYASH_VERIFY_JSON="$JSON" "$BIN" --backend vm "$TMP" 2>&1)
RC1=$?
set -e

rm -f "$TMP" || true

if [ $RC0 -ne 33 ] || [ $RC1 -ne 33 ]; then
  echo "[FAIL] hv1_typed_ir_primary_inline_parity_alias_only: rc0=$RC0 rc1=$RC1" >&2
  echo "--- OUT0 ---" >&2
  echo "$OUT0" >&2
  echo "--- OUT1 ---" >&2
  echo "$OUT1" >&2
  exit 1
fi

echo "[PASS] hv1_typed_ir_primary_inline_parity_alias_only_canary_vm"


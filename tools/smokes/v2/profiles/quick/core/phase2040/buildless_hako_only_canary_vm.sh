#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../../../../../../.." && pwd)
BIN="$ROOT_DIR/target/release/hakorune"

if [ ! -x "$BIN" ]; then
  cargo build --release >/dev/null
fi

# Activate hako-only dev profile (buildless)
set +u
source "$ROOT_DIR/tools/dev_env.sh" hako-only >/dev/null 2>&1 || true
set -u

# Prepare minimal MIR v1/v0-ish JSON that returns 42 via hv1 direct route
JSON='{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"int","value":42}},{"op":"ret","value":1}]}]}]}'

set +e
OUT=$(HAKO_VERIFY_PRIMARY=hakovm NYASH_VERIFY_JSON="$JSON" "$BIN" --backend vm "$ROOT_DIR/basic_test.hako" 2>&1)
RC=$?
set -e

# Expect exit code equals 42
if [ $RC -ne 42 ]; then
  echo "[FAIL] buildless_hako_only_canary_vm: expected rc=42, got rc=$RC" >&2
  echo "$OUT" >&2
  exit 1
fi

# Expect clean output (no plugin init) and final line == 42
echo "$OUT" | grep -q 'UnifiedBoxRegistry' && {
  echo "[FAIL] plugin init leaked in hako-only profile" >&2
  echo "$OUT" >&2
  exit 1
}

tail_line=$(echo "$OUT" | tail -n1)
if [ "$tail_line" != "42" ]; then
  echo "[FAIL] expected '42', got '$tail_line'" >&2
  echo "$OUT" >&2
  exit 1
fi

echo "[PASS] buildless_hako_only_canary_vm"

#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../../../../../../.." && pwd)
BIN="$ROOT_DIR/target/release/hakorune"

if [ ! -x "$BIN" ]; then
  cargo build --release >/dev/null
fi

# Activate hybrid profile
set +u
source "$ROOT_DIR/tools/dev_env.sh" hybrid >/dev/null 2>&1 || true
set -u

# 1) hv1 direct small JSON → expect 7
JSON='{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"int","value":7}},{"op":"ret","value":1}]}]}]}'

set +e
OUT1=$(HAKO_VERIFY_PRIMARY=hakovm NYASH_VERIFY_JSON="$JSON" "$BIN" --backend vm "$ROOT_DIR/basic_test.hako" 2>&1)
RC1=$?
set -e

if [ $RC1 -ne 7 ] || [ "$(echo "$OUT1" | tail -n1)" != "7" ]; then
  echo "[FAIL] hybrid hv1-direct failed (rc=$RC1)" >&2
  echo "$OUT1" >&2
  exit 1
fi

echo "[PASS] hybrid_profile_smoke_vm"

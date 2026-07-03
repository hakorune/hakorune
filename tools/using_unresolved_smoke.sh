#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
HAKORUNE_BIN="$ROOT_DIR/target/release/hakorune"
LEGACY_NYASH_BIN="$ROOT_DIR/target/release/nyash"
if [ -x "$HAKORUNE_BIN" ]; then
  BIN="$HAKORUNE_BIN"
else
  BIN="$LEGACY_NYASH_BIN"
fi

if [ ! -x "$BIN" ]; then
  cargo build --release --features cranelift-jit >/dev/null
  BIN="$HAKORUNE_BIN"
fi

JSON=$(mktemp)
cat >"$JSON" <<'JSON'
{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Int","value":0}}]}
JSON

set +e
out=$(NYASH_CLI_VERBOSE=1 "$BIN" --backend vm --json-file "$JSON" --using "no.such.ns as X" 2>&1)
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  echo "FAIL: unresolved CLI using should remain non-fatal for JSON input (rc=$rc)" >&2
  echo "$out" >&2
  exit 1
fi
echo "$out" | rg -q 'ret %1|RC:[[:space:]]*0([[:space:]]|$)' || { echo "FAIL: JSON input did not reach execution/MIR output" >&2; echo "$out" >&2; exit 1; }
echo "PASS: using unresolved remains non-fatal (CLI JSON)" >&2
echo "All PASS" >&2

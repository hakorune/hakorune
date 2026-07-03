#!/bin/bash
# canonicalize_off_vm.sh — Bridge canonicalize OFF (opt-in)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_ENABLE_BRIDGE_CANON:-0}" != "1" ]; then
  echo "[SKIP] SMOKES_ENABLE_BRIDGE_CANON!=1; skipping bridge canonicalize(off)" >&2
  exit 0
fi

# Default-on: bridge canonicalize off

# Same v1 minimal JSON (const+ret) without toggles should still run (no mir_call involved)
json_path="/tmp/ny_v1_const_off_$$.json"
log_path="/tmp/bridge_canonicalize_off_$$.log"
trap 'rm -f "$json_path" "$log_path"' EXIT
BIN="$ROOT/target/release/hakorune"
LEGACY_NYASH_BIN="$ROOT/target/release/nyash"
if [ ! -x "$BIN" ] && [ -x "$LEGACY_NYASH_BIN" ]; then
  BIN="$LEGACY_NYASH_BIN"
fi
cat >"$json_path" <<'JSON'
{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"i64","value":3}},{"op":"ret","value":1}]}]}]}
JSON

set +e
NYASH_NYVM_V1_DOWNCONVERT=1 "$BIN" --json-file "$json_path" >"$log_path" 2>&1
rc=$?
set -e
if [ $rc -eq 0 ]; then
  echo "[PASS] bridge_canonicalize_off"
  exit 0
else
  cat "$log_path" >&2
  echo "[FAIL] bridge_canonicalize_off: expected rc=0, got $rc" >&2
  exit 1
fi

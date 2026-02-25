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
cat >"$json_path" <<'JSON'
{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"Integer","value":3}},{"op":"ret","value":1}]}]}]}
JSON

NYASH_NYVM_V1_DOWNCONVERT=1 "$ROOT/target/release/nyash" --json-file "$json_path" >/dev/null 2>&1
rc=$?
rm -f "$json_path"
if [ $rc -eq 0 ]; then
  echo "[PASS] bridge_canonicalize_off"
  exit 0
else
  echo "[FAIL] bridge_canonicalize_off: expected rc=0, got $rc" >&2
  exit 1
fi

#!/bin/bash
# canonicalize_on_vm.sh — Bridge canonicalize ON (opt-in)

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
  echo "[SKIP] SMOKES_ENABLE_BRIDGE_CANON!=1; skipping bridge canonicalize(on)" >&2
  exit 0
fi

# Default-on: bridge canonicalize on

# Minimal v1 JSON with only const/copy/ret (no mir_call), should run regardless
json_path="/tmp/ny_v1_const_$$.json"
cat >"$json_path" <<'JSON'
{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"Integer","value":7}},{"op":"ret","value":1}]}]}]}
JSON

HAKO_NYVM_V1_DOWNCONVERT=1 HAKO_BRIDGE_INJECT_SINGLETON=1 \
  "$ROOT/target/release/nyash" --json-file "$json_path" >/dev/null 2>&1
rc=$?
rm -f "$json_path"
if [ $rc -eq 0 ]; then
  echo "[PASS] bridge_canonicalize_on"
  exit 0
else
  echo "[FAIL] bridge_canonicalize_on: expected rc=0, got $rc" >&2
  exit 1
fi

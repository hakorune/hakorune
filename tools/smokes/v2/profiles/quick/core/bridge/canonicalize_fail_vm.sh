#!/bin/bash
# canonicalize_fail_vm.sh — Bridge canonicalize FAIL case (opt-in)

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
  echo "[SKIP] SMOKES_ENABLE_BRIDGE_CANON!=1; skipping bridge canonicalize(fail)" >&2
  exit 0
fi

# Default-on: bridge canonicalize fail

# v1 JSON with unsupported instruction to assert stable failure message
json_path="/tmp/ny_v1_fail_$$.json"
cat >"$json_path" <<'JSON'
{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"foo","dst":1},{"op":"ret"}]}]}]}
JSON

set +e
output=$(NYASH_NYVM_V1_DOWNCONVERT=1 "$ROOT/target/release/nyash" --json-file "$json_path" 2>&1)
rc=$?
set -e
rm -f "$json_path"

if [ $rc -ne 0 ] && echo "$output" | grep -q "unsupported instruction 'foo'"; then
  echo "[PASS] bridge_canonicalize_fail"
  exit 0
else
  echo "[FAIL] bridge_canonicalize_fail: expected failure with stable message" >&2
  echo "$output" >&2
  exit 1
fi

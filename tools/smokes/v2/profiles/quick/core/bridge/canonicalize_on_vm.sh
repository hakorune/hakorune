#!/bin/bash
# canonicalize_on_vm.sh — Retired singleton canonicalize route (opt-in)

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

# Singleton injection is retired; this smoke fixes the pre-mutation stop.

# Minimal v1 JSON with only const/copy/ret (no mir_call), should run regardless
json_path="/tmp/ny_v1_singleton_retired_$$.json"
trap 'rm -f "$json_path"' EXIT
cat >"$json_path" <<'JSON'
{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","mir_call":{"callee":{"type":"ModuleFunction","name":"LLVMPhiInstructionBox.lower_phi"},"args":[1,2]}},{"op":"ret"}]}]}]}
JSON

set +e
output=$(HAKO_NYVM_V1_DOWNCONVERT=1 HAKO_BRIDGE_INJECT_SINGLETON=1 \
  "$NYASH_BIN" --json-file "$json_path" 2>&1)
rc=$?
set -e
if [ $rc -ne 0 ] && echo "$output" | grep -q \
  "\[freeze:contract\]\[mir-json-bridge/singleton-injection-retired\]"; then
  echo "[PASS] bridge_singleton_retired"
  exit 0
else
  printf '%s\n' "$output" >&2
  echo "[FAIL] bridge_singleton_retired: expected typed rejection, got rc=$rc" >&2
  exit 1
fi

#!/usr/bin/env bash
# receiver_undefined_method_fails_vm.sh — Method call with undefined receiver should fail (no dev fallback)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

TMP_MIR="/tmp/mir_recv_undef_fail_$$.json"
trap 'rm -f "$TMP_MIR" || true' EXIT

# Minimal MIR(JSON v1) with a single call: ArrayBox.len(receiver=99)
# No NewBox/Copy defines value 99 in the block; VM should error when loading receiver.
cat >"$TMP_MIR" <<'JSON'
{
  "schema_version":"1.0",
  "functions":[
    {
      "name":"Main.main",
      "params":[],
      "blocks":[
        {
          "id":0,
          "instructions":[
            {"op":"mir_call",
             "dst":1,
             "mir_call":{
               "callee":{ "type":"Method", "box_name":"ArrayBox", "method":"len", "receiver": 99 },
               "args":[], "effects":[]
             }
            }
          ]
        }
      ]
    }
  ]
}
JSON

set +e
# Ensure dev safety toggles are OFF
NYASH_VM_RECV_ARG_FALLBACK=0 NYASH_VM_TOLERATE_VOID=0 "$NYASH_BIN" --mir-json-file "$TMP_MIR" >/dev/null 2>&1
RC=$?
set -e

if [ $RC -eq 0 ]; then
  echo "[FAIL] receiver_undefined_method_fails_vm: expected non-zero exit" >&2
  exit 1
fi

echo "[PASS] receiver_undefined_method_fails_vm"
exit 0


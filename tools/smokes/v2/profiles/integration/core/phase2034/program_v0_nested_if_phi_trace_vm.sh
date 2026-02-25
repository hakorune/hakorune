#!/bin/bash
# Program(JSON v0) → Core (--json-file) PHI trace: nested if
# Checks that PHI preds remain consistent under nested then/else.

set -euo pipefail

if [ "${SMOKES_ENABLE_DEBUG:-0}" != "1" ]; then
  echo "[SKIP] program_v0_nested_if_phi_trace_vm (enable with SMOKES_ENABLE_DEBUG=1)" >&2
  exit 0
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
enable_mirbuilder_dev_env

tmp_json="/tmp/prog_nested_if_phi_$$.json"

cat > "$tmp_json" <<'JSON'
{
  "version": 0,
  "kind": "Program",
  "body": [
    { "type":"Local", "name":"x", "expr": {"type":"Int","value":1} },
    { "type":"If",
      "cond": { "type":"Compare", "op": "<", "lhs": {"type":"Int","value":1}, "rhs": {"type":"Int","value":2} },
      "then": [
        { "type":"If",
          "cond": { "type":"Compare", "op": ">", "lhs": {"type":"Int","value":3}, "rhs": {"type":"Int","value":4} },
          "then": [ { "type":"Local", "name":"x", "expr": {"type":"Int","value": 10} } ],
          "else": [ { "type":"Local", "name":"x", "expr": {"type":"Int","value": 20} } ]
        }
      ],
      "else": [ { "type":"Local", "name":"x", "expr": {"type":"Int","value": 30} } ]
    },
    { "type":"Return", "expr": {"type":"Var","name":"x"} }
  ]
}
JSON

set +e
out="$(NYASH_VM_TRACE_PHI=1 "$NYASH_BIN" --json-file "$tmp_json" 2>&1)"; rc=$?
set -e
rm -f "$tmp_json" || true

if echo "$out" | grep -q "phi pred mismatch"; then
  echo "[FAIL] program_v0_nested_if_phi_trace_vm (phi pred mismatch)" >&2
  echo "$out" | sed -n '1,160p' >&2
  exit 1
fi

echo "[PASS] program_v0_nested_if_phi_trace_vm"
exit 0


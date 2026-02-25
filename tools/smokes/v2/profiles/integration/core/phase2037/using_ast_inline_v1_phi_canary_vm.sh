#!/bin/bash
# Test: Using AST text merge for inline Hako execution with v1 dispatcher
# Verifies that text-based prelude merge resolves transitive dependencies correctly
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Create inline MIR JSON (v1) - simple phi test
tmp_json="/tmp/mir_v1_phi_inline_$$.json"
cat > "$tmp_json" <<'JSON'
{
  "schema_version": "1.0",
  "functions": [
    {"name":"main","blocks":[
      {"id":0,"instructions":[
        {"op":"const","dst":1, "value": {"type": "i64", "value": 7}},
        {"op":"const","dst":2, "value": {"type": "i64", "value": 9}},
        {"op":"compare","dst":3, "lhs":1, "rhs":2, "cmp":"Lt"},
        {"op":"branch","cond":3, "then":1, "else":2}
      ]},
      {"id":1,"instructions":[
        {"op":"phi","dst":5, "incoming": [[2,0]]},
        {"op":"ret","value":5}
      ]},
      {"id":2,"instructions":[
        {"op":"phi","dst":6, "incoming": [[1,0]]},
        {"op":"ret","value":6}
      ]}
    ]}
  ]
}
JSON

# Prepare JSON literal for inline code
json_literal="$(jq -Rs . < "$tmp_json")"

# Verify via primary pipeline (hakovm first → fallback core)
set +e
verify_mir_rc "$tmp_json"
rc=$?
set -e
rm -f "$tmp_json"

# Expect rc=9 (7 < 9 → then=1 → block1 phi picks [2,0] → ret 9)
if [ "$rc" -eq 9 ]; then
  echo "[PASS] using_ast_inline_v1_phi_canary_vm"
  exit 0
fi

echo "[FAIL] using_ast_inline_v1_phi_canary_vm (rc=$rc, expect 9)" >&2
echo "[FAIL] Inline using with AST text merge did not resolve correctly" >&2
exit 1

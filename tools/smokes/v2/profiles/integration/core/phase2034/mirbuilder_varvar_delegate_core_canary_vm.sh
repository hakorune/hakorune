#!/bin/bash
# Verify Logical Var/Var via Delegate + Core direct (structure lock)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_varvar_delegate_core_$$.hako"
tmp_json="/tmp/mirbuilder_varvar_delegate_core_$$.json"
cat > "$tmp_hako" <<'HAKO'
static box Main { method main(args) {
  // Local b1=true; Local b2=false; return b1 && b2;
  local j = "{\"version\":0,\"kind\":\"Program\",\"body\":[" +
    "{\"type\":\"Local\",\"name\":\"b1\",\"expr\":{\"type\":\"Bool\",\"value\":true}}," +
    "{\"type\":\"Local\",\"name\":\"b2\",\"expr\":{\"type\":\"Bool\",\"value\":false}}," +
    "{\"type\":\"Return\",\"expr\":{\"type\":\"Logical\",\"op\":\"&&\",\"lhs\":{\"type\":\"Var\",\"name\":\"b1\"},\"rhs\":{\"type\":\"Var\",\"name\":\"b2\"}}}]}";
  // Call provider via extern directly (avoid MirBuilderBox toggles)
  local arr = new ArrayBox(); arr.push(j)
  local out = hostbridge.extern_invoke("env.mirbuilder", "emit", arr)
  if out == null { return 0 }
  print("" + out)
  return 1
} }
HAKO

set +e
out="$(out="$(HAKO_MIR_BUILDER_DELEGATE=1 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 run_nyash_vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
if [ "$rc" -ne 1 ]; then echo "$out" >&2; echo "[FAIL] varvar_delegate_core (emit)" >&2; rm -f "$tmp_hako" "$tmp_json"; exit 1; fi
# Be tolerant of pretty-printed JSON (multi-line). Validate and capture all.
echo "$out" | jq -e . > "$tmp_json" || { echo "$out" >&2; echo "[FAIL] varvar_delegate_core (no MIR)" >&2; rm -f "$tmp_hako" "$tmp_json"; exit 1; }

set +e
NYASH_GATE_C_CORE=1 HAKO_GATE_C_CORE=1 "$NYASH_BIN" --json-file "$tmp_json" >/dev/null 2>&1
rc2=$?
set -e
rm -f "$tmp_hako" "$tmp_json" || true
if [ "$rc2" -eq 0 ]; then
  echo "[FAIL] varvar_delegate_core rc=0 (expected non-zero for && with true,false)" >&2
  exit 1
fi
echo "[PASS] mirbuilder_varvar_delegate_core_canary_vm"
exit 0

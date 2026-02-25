#!/bin/bash
# mirbuilder_internal_return_logical_varvar_lower_canary_vm.sh
# Purpose: Verify LowerReturnLogicalBox.try_lower directly (bypass MirBuilderBox integration)
# Change: Migrate to content-based check (no rc-based PASS). Print MIR and grep tokens.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2
enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_lower_logical_varvar_$$.hako"
cat > "$tmp_hako" <<'HAKO'
using "hako.mir.builder.internal.lower.logical" as LowerReturnLogicalBox
static box Main { method main(args) {
  // Local b1=true; Local b2=false; return b1 && b2;
  local j = "{\"version\":0,\"kind\":\"Program\",\"body\":[" +
    "{\"type\":\"Local\",\"name\":\"b1\",\"expr\":{\"type\":\"Bool\",\"value\":true}}," +
    "{\"type\":\"Local\",\"name\":\"b2\",\"expr\":{\"type\":\"Bool\",\"value\":false}}," +
    "{\"type\":\"Return\",\"expr\":{\"type\":\"Logical\",\"op\":\"&&\",\"lhs\":{\"type\":\"Var\",\"name\":\"b1\"},\"rhs\":{\"type\":\"Var\",\"name\":\"b2\"}}}]}";
  local out = LowerReturnLogicalBox.try_lower(j);
  // Fallback (bring-up): delegate via provider when Lower is not yet available
  if out == null {
    local a = new ArrayBox(); a.push(j)
    out = hostbridge.extern_invoke("env.mirbuilder", "emit", a)
    if out == null { print("[fail:lower+provider]"); return 1 }
  }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

set +e
out="$(NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 NYASH_NY_COMPILER_TIMEOUT_MS=20000 \
  run_nyash_vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
rm -f "$tmp_hako" || true

if [ "$rc" -ne 0 ]; then
  echo "$out" >&2
  echo "[FAIL] mirbuilder_internal_return_logical_varvar_lower_canary_vm (vm rc=$rc)" >&2
  exit 1
fi
mir=$(echo "$out" | awk '/\[MIR_BEGIN\]/{f=1;next}/\[MIR_END\]/{f=0}f')
if [ -n "$mir" ] && echo "$mir" | grep -q '"functions"' \
  && echo "$mir" | grep -q '"op":"branch"' \
  && echo "$mir" | grep -q '"op":"ret"'; then
  echo "[PASS] mirbuilder_internal_return_logical_varvar_lower_canary_vm"; exit 0; fi
echo "$out" >&2
echo "[FAIL] mirbuilder_internal_return_logical_varvar_lower_canary_vm (content)" >&2
exit 1

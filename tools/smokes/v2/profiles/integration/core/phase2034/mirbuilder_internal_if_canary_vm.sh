#!/bin/bash
# mirbuilder_internal_if_canary_vm.sh — Program(JSON v0) If/Compare → MIR(JSON) internal box canary

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2
SMOKES_DEV_PREINCLUDE=1 enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_internal_if_$$.hako"
cat > "$tmp_hako" <<'HAKO'
static box Main { method main(args) {
  // Program(JSON v0): if (1 < 2) return 10; else return 20;
  local j = "{\"version\":0,\"kind\":\"Program\",\"body\":[{\"type\":\"If\",\"cond\":{\"type\":\"Compare\",\"op\":\"<\",\"lhs\":{\"type\":\"Int\",\"value\":1},\"rhs\":{\"type\":\"Int\",\"value\":2}},\"then\":[{\"type\":\"Return\",\"expr\":{\"type\":\"Int\",\"value\":10}}],\"else\":[{\"type\":\"Return\",\"expr\":{\"type\":\"Int\",\"value\":20}}]}]}";
  local a = new ArrayBox(); a.push(j)
  local out = hostbridge.extern_invoke("env.mirbuilder", "emit", a)
  if out == null { print("[fail:builder]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

set +e
out="$(run_nyash_vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
rm -f "$tmp_hako" 2>/dev/null || true

mir=$(echo "$out" | awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag')
if [ -n "$mir" ] \
  && echo "$mir" | grep -q '"op":"compare"' \
  && echo "$mir" | grep -q '"op":"branch"' \
  && echo "$mir" | grep -q '"op":"ret"'; then
  echo "[PASS] mirbuilder_internal_if_canary_vm"; exit 0; fi
echo "[FAIL] mirbuilder_internal_if_canary_vm" >&2; exit 1

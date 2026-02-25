#!/bin/bash
# mirbuilder_canary_vm.sh — Program(JSON v0) → MIR(JSON) box canary (delegate-first; SKIP when provider absent)

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

tmp_hako="/tmp/mirbuilder_canary_$$.hako"
cat > "$tmp_hako" <<'HAKO'
using hako.mir.builder as MirBuilderBox
static box Main { method main(args) {
  // Build minimal Program(JSON v0)
  local j = "{\"version\":0,\"kind\":\"Program\",\"body\":[{\"type\":\"Return\",\"expr\":{\"type\":\"Int\",\"value\":42}}]}";
  local out = MirBuilderBox.emit_from_program_json_v0(j, null);
  if out == null { print("[fail:builder]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

set +e
out="$(HAKO_MIR_BUILDER_DELEGATE=1 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
  $NYASH_BIN --backend vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
rm -f "$tmp_hako" 2>/dev/null || true

mir=$(echo "$out" | awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag')
if [ -n "$mir" ] && echo "$mir" | grep -q '"functions"' && echo "$mir" | grep -q '"blocks"'; then
  echo "[PASS] mirbuilder_canary_vm"; exit 0; fi
echo "[FAIL] mirbuilder_canary_vm" >&2; exit 1

#!/bin/bash
# mirbuilder_internal_canary_vm.sh — Program(JSON v0) → MIR(JSON) internal box canary

set -euo pipefail

# Default ON in quick: runs internal Return(Int) path with local toggle only for this test.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2
SMOKES_DEV_PREINCLUDE=1 enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_internal_$$.hako"
cat > "$tmp_hako" <<'HAKO'
using "hako.mir.builder" as MirBuilderBox
static box Main { method main(args) {
  // Minimal Program(JSON v0) with Return(Int)
  local j = "{\"version\":0,\"kind\":\"Program\",\"body\":[{\"type\":\"Return\",\"expr\":{\"type\":\"Int\",\"value\":7}}]}";
  local out = MirBuilderBox.emit_from_program_json_v0(j, null);
  if out == null { return 0 }
  local s = "" + out
  if s.indexOf("\"functions\"") >= 0 && s.indexOf("\"blocks\"") >= 0 && s.indexOf("\"value\":7") >= 0 { return 1 }
  return 0
} }
HAKO

set +e
out="$(HAKO_MIR_BUILDER_INTERNAL=1 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
  "$NYASH_BIN" --backend vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
rm -f "$tmp_hako" 2>/dev/null || true

if [ "$rc" -eq 1 ]; then
  echo "[PASS] mirbuilder_internal_canary_vm"
  exit 0
fi
echo "[FAIL] mirbuilder_internal_canary_vm (rc=$rc)" >&2; exit 1

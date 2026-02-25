#!/bin/bash
# Return(Binary Var/Int) with negative constants → const+const+binop+ret (structure)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_return_binop_varint_neg_$$.hako"
cat > "$tmp_hako" <<'HAKO'
using "hako.mir.builder" as MirBuilderBox
static box Main { method main(args) {
  // Local x=-2; return x + -3;
  local j = "{\"version\":0,\"kind\":\"Program\",\"body\":[{\"type\":\"Local\",\"name\":\"x\",\"expr\":{\"type\":\"Int\",\"value\":-2}},{\"type\":\"Return\",\"expr\":{\"type\":\"Binary\",\"op\":\"+\",\"lhs\":{\"type\":\"Var\",\"name\":\"x\"},\"rhs\":{\"type\":\"Int\",\"value\":-3}}}]}";
  local out = MirBuilderBox.emit_from_program_json_v0(j, null);
  if out == null { return 0 }
  local s = "" + out
  if s.indexOf("\"op\":\"binop\"") >= 0 && s.indexOf("\"op_kind\":\"Add\"") >= 0 { return 1 }
  return 0
} }
HAKO

set +e
out="$(HAKO_MIR_BUILDER_INTERNAL=1 "$NYASH_BIN" --backend vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
rm -f "$tmp_hako" || true

if [ "$rc" -eq 1 ]; then echo "[PASS] mirbuilder_internal_return_binop_varint_negative_canary_vm"; exit 0; fi
echo "[FAIL] mirbuilder_internal_return_binop_varint_negative_canary_vm (rc=$rc)" >&2; exit 1

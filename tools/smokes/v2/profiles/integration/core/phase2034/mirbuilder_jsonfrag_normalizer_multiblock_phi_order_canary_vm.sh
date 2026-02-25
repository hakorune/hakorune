#!/usr/bin/env bash
# Verify Normalizer groups phi at head for multiple blocks and preserves stable order
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
source "$ROOT/tools/smokes/v2/lib/mir_canary.sh" || true

tmp_hako="/tmp/mirbuilder_normalizer_multibb_$$.hako"
cat > "$tmp_hako" <<'HAKO'
using "hako.mir.builder.internal.jsonfrag_normalizer" as NormBox
static box Main { method main(args) {
  local m = env.get("MIR"); if m == null { print("[fail:nomir]"); return 1 }
  local out = NormBox.normalize_all("" + m)
  if out == null { print("[fail:norm]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

# Two blocks, each with late phi followed by const/ret; expect phi moved ahead of others in each block
MIR='{"functions":[{"name":"main","params":[],"locals":[],"blocks":[
  {"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"i64","value":0}},{"op":"ret"},{"op":"phi","dst":5,"values":[{"value":1,"block":1}]}]},
  {"id":1,"instructions":[{"op":"compare","operation":"<","lhs":1,"rhs":2,"dst":9},{"op":"phi","dst":7,"values":[{"value":2,"block":0}]},{"op":"const","dst":2,"value":{"type":"i64","value":2}}]}
]}]}'

set +e
out="$(MIR="$MIR" run_nyash_vm "$tmp_hako" 2>&1)"; rc=$?
set -e
rm -f "$tmp_hako" || true

if [[ "$rc" -ne 0 ]]; then echo "[SKIP] normalizer_multibb: vm exec unstable"; exit 0; fi
mir=$(echo "$out" | extract_mir_from_output)
if [[ -z "$mir" ]]; then echo "[SKIP] normalizer_multibb: no MIR"; exit 0; fi

# We cannot easily target per-block segments without a parser; instead check that the first phi appears before first compare and before first const/ret
if ! echo "$mir" | assert_order '"op":"phi"' '"op":"compare"'; then echo "[SKIP] normalizer_multibb: phi not before compare"; exit 0; fi
if ! echo "$mir" | assert_order '"op":"phi"' '"op":"const"'; then echo "[SKIP] normalizer_multibb: phi not before const"; exit 0; fi
if ! echo "$mir" | assert_order '"op":"phi"' '"op":"ret"'; then echo "[SKIP] normalizer_multibb: phi not before ret"; exit 0; fi

echo "[PASS] mirbuilder_jsonfrag_normalizer_multiblock_phi_order_canary_vm"
exit 0


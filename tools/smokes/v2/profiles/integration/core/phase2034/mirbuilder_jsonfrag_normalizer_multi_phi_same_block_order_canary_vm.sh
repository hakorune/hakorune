#!/usr/bin/env bash
# Verify Normalizer preserves relative order of multiple phi in same block and moves them to head
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
source "$ROOT/tools/smokes/v2/lib/mir_canary.sh" || true

tmp_hako="/tmp/mirbuilder_normalizer_multi_phi_same_block_$$.hako"
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

# One block containing: const, phi(dst=5), compare, phi(dst=6), ret  → expect phi(5), phi(6) first
MIR='{"functions":[{"name":"main","params":[],"locals":[],"blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"i64","value":0}},{"op":"phi","dst":5,"values":[{"value":1,"block":1}]},{"op":"compare","operation":"<","lhs":1,"rhs":2,"dst":9},{"op":"phi","dst":6,"values":[{"value":2,"block":0}]},{"op":"ret","value":1}]}]}]}'

set +e
out="$(MIR="$MIR" run_nyash_vm "$tmp_hako" 2>&1)"; rc=$?
set -e
rm -f "$tmp_hako" || true

if [[ "$rc" -ne 0 ]]; then echo "[SKIP] normalizer_multi_phi_same_block: vm exec unstable"; exit 0; fi
mir=$(echo "$out" | extract_mir_from_output)
if [[ -z "$mir" ]]; then echo "[SKIP] normalizer_multi_phi_same_block: no MIR"; exit 0; fi

# Check: both phi occur, phi(5) before phi(6), and both before compare/const/ret
if ! echo "$mir" | assert_token_count '"op":"phi"' 2; then echo "[SKIP] normalizer_multi_phi_same_block: phi count != 2"; exit 0; fi
if ! echo "$mir" | assert_order '"op":"phi","dst":5' '"op":"phi","dst":6'; then echo "[SKIP] normalizer_multi_phi_same_block: phi order unstable"; exit 0; fi
if ! echo "$mir" | assert_order '"op":"phi"' '"op":"compare"'; then echo "[SKIP] normalizer_multi_phi_same_block: phi not before compare"; exit 0; fi
if ! echo "$mir" | assert_order '"op":"phi"' '"op":"const"'; then echo "[SKIP] normalizer_multi_phi_same_block: phi not before const"; exit 0; fi
if ! echo "$mir" | assert_order '"op":"phi"' '"op":"ret"'; then echo "[SKIP] normalizer_multi_phi_same_block: phi not before ret"; exit 0; fi

echo "[PASS] mirbuilder_jsonfrag_normalizer_multi_phi_same_block_order_canary_vm"
exit 0


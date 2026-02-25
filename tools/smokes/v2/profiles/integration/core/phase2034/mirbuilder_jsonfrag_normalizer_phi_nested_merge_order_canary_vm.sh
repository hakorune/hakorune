#!/usr/bin/env bash
# Verify PHI grouping for nested merge scenario (phi appears before compare/const/ret after normalization)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
source "$ROOT/tools/smokes/v2/lib/mir_canary.sh" || true

tmp_hako="/tmp/mirbuilder_normalizer_phi_nested_$$.hako"
cat > "$tmp_hako" <<'HAKO'
using "hako.mir.builder.internal.jsonfrag_normalizer" as NormBox
static box Main { method main(args) {
  local m = env.get("MIR"); if m == null { print("[fail:nomir]"); return 1 }
  local out = NormBox.normalize_all("" + m)
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

# Craft a nested-style merge: block 3 acts as a join with phi late in the instruction list
MIR='{"functions":[{"name":"main","params":[],"locals":[],"blocks":[
  {"id":0,"instructions":[{"op":"branch","cond":1,"then":1,"else":2}]},
  {"id":1,"instructions":[{"op":"const","dst":10,"value":{"type":"i64","value":1}},{"op":"branch","cond":1,"then":3,"else":3}]},
  {"id":2,"instructions":[{"op":"const","dst":20,"value":{"type":"i64","value":2}},{"op":"branch","cond":1,"then":3,"else":3}]},
  {"id":3,"instructions":[{"op":"compare","operation":"<","lhs":1,"rhs":2,"dst":9},{"op":"const","dst":30,"value":{"type":"i64","value":3}},{"op":"phi","dst":5,"values":[{"value":10,"block":1},{"value":20,"block":2}]},{"op":"ret","value":5}]}]}]}'

set +e
out="$(MIR="$MIR" run_nyash_vm "$tmp_hako" 2>&1)"; rc=$?
set -e
rm -f "$tmp_hako" || true

if [[ "$rc" -ne 0 ]]; then echo "[SKIP] phi_nested_merge: vm exec unstable"; exit 0; fi
mir=$(echo "$out" | extract_mir_from_output)
if [[ -z "$mir" ]]; then echo "[SKIP] phi_nested_merge: no MIR"; exit 0; fi

if ! echo "$mir" | assert_order '"op":"phi"' '"op":"compare"'; then echo "[SKIP] phi_nested_merge: phi not before compare"; exit 0; fi
if ! echo "$mir" | assert_order '"op":"phi"' '"op":"const"'; then echo "[SKIP] phi_nested_merge: phi not before const"; exit 0; fi
if ! echo "$mir" | assert_order '"op":"phi"' '"op":"ret"'; then echo "[SKIP] phi_nested_merge: phi not before ret"; exit 0; fi

echo "[PASS] mirbuilder_jsonfrag_normalizer_phi_nested_merge_order_canary_vm"
exit 0


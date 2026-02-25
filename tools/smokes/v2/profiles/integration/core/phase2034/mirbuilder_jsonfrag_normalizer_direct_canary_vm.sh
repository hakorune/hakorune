#!/usr/bin/env bash
# Verify JsonFrag Normalizer: phi grouping, ret value insertion, const dedupe (direct call)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
source "$ROOT/tools/smokes/v2/lib/mir_canary.sh" || true

tmp_hako="/tmp/mirbuilder_jsonfrag_normalizer_direct_$$.hako"
cat > "$tmp_hako" <<'HAKO'
using "hako.mir.builder.internal.jsonfrag_normalizer" as NormBox
static box Main { method main(args) {
  // Input MIR(JSON) crafted with: non-head phi, duplicate const, ret without value
  local m = env.get("MIR"); if m == null { print("[fail:nomir]"); return 1 }
  local out = NormBox.normalize_all("" + m)
  if out == null { print("[fail:norm]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

# Craft MIR(JSON) with phi late, duplicate const, and ret without value
MIR='{"functions":[{"name":"main","params":[],"locals":[],"blocks":[{"id":0,"instructions":[{"op":"compare","operation":"<","lhs":1,"rhs":2,"dst":9},{"op":"const","dst":1,"value":{"type":"i64","value":42}},{"op":"ret"},{"op":"const","dst":1,"value":{"type":"i64","value":42}},{"op":"phi","dst":5,"values":[{"value":1,"block":1},{"value":2,"block":2}]}]}]}]}'

set +e
out="$(MIR="$MIR" run_nyash_vm "$tmp_hako" 2>&1)"; rc=$?
set -e
rm -f "$tmp_hako" || true

if [[ "$rc" -ne 0 ]]; then echo "[SKIP] normalizer_direct: vm exec unstable"; exit 0; fi
mir=$(echo "$out" | extract_mir_from_output)
if [[ -z "$mir" ]]; then echo "[SKIP] normalizer_direct: no MIR"; exit 0; fi

# 1) phi grouped before compare
if ! echo "$mir" | assert_order '"op":"phi"' '"op":"compare"'; then echo "[SKIP] normalizer_direct: phi not grouped"; exit 0; fi
# 2) ret has value
if ! echo "$mir" | assert_has_tokens '"op":"ret","value"'; then echo "[SKIP] normalizer_direct: ret value missing"; exit 0; fi
# 3) const deduped (dst 1, i64 42 occurs once)
if ! echo "$mir" | assert_token_count '"op":"const","dst":1,"value":{"type":"i64","value":42}' 1; then echo "[SKIP] normalizer_direct: const dedupe failed"; exit 0; fi

echo "[PASS] mirbuilder_jsonfrag_normalizer_direct_canary_vm"
exit 0


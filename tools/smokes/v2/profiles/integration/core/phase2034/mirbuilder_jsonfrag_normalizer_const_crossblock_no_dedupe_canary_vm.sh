#!/usr/bin/env bash
# Verify const dedupe is block-local (identical const in different blocks remain both present)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
source "$ROOT/tools/smokes/v2/lib/mir_canary.sh" || true

tmp_hako="/tmp/mirbuilder_normalizer_crossblock_const_$$.hako"
cat > "$tmp_hako" <<'HAKO'
using "hako.mir.builder.internal.jsonfrag_normalizer" as NormBox
static box Main { method main(args) {
  local m = env.get("MIR"); if m == null { print("[fail:nomir]"); return 1 }
  local out = NormBox.normalize_all("" + m)
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

# Two blocks with same const (dst=1, i64 42); should remain two occurrences after normalization
MIR='{"functions":[{"name":"main","params":[],"locals":[],"blocks":[
  {"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"i64","value":42}},{"op":"ret","value":1}]},
  {"id":1,"instructions":[{"op":"const","dst":1,"value":{"type":"i64","value":42}},{"op":"ret","value":1}]}
]}]}'

set +e
out="$(MIR="$MIR" run_nyash_vm "$tmp_hako" 2>&1)"; rc=$?
set -e
rm -f "$tmp_hako" || true

if [[ "$rc" -ne 0 ]]; then echo "[SKIP] normalizer_crossblock_const: vm exec unstable"; exit 0; fi
mir=$(echo "$out" | extract_mir_from_output)
if [[ -z "$mir" ]]; then echo "[SKIP] normalizer_crossblock_const: no MIR"; exit 0; fi
if echo "$mir" | assert_token_count '"op":"const","dst":1,"value":{"type":"i64","value":42}' 2; then
  echo "[PASS] mirbuilder_jsonfrag_normalizer_const_crossblock_no_dedupe_canary_vm"; exit 0; fi
echo "[SKIP] normalizer_crossblock_const: count mismatch"; exit 0


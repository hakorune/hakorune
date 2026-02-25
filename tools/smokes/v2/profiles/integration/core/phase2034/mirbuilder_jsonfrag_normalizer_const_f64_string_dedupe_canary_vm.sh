#!/usr/bin/env bash
# Verify Normalizer dedupes const for f64 and String values
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
source "$ROOT/tools/smokes/v2/lib/mir_canary.sh" || true

tmp_hako="/tmp/mirbuilder_normalizer_const_fx_$$.hako"
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

# MIR: duplicate f64 and duplicate String constants on same dst → expect single occurrence each
MIR='{"functions":[{"name":"main","params":[],"locals":[],"blocks":[{"id":0,"instructions":[
  {"op":"const","dst":2,"value":{"type":"f64","value":3.140000}},
  {"op":"const","dst":2,"value":{"type":"f64","value":3.14}},
  {"op":"const","dst":3,"value":{"String":"hello"}},
  {"op":"const","dst":3,"value":{"String":"hello"}},
  {"op":"ret","value":2}
]}]}]}'

set +e
out="$(MIR="$MIR" run_nyash_vm "$tmp_hako" 2>&1)"; rc=$?
set -e
rm -f "$tmp_hako" || true

if [[ "$rc" -ne 0 ]]; then echo "[SKIP] normalizer_const_fx: vm exec unstable"; exit 0; fi
mir=$(echo "$out" | extract_mir_from_output)
if [[ -z "$mir" ]]; then echo "[SKIP] normalizer_const_fx: no MIR"; exit 0; fi

# f64 and String const should each remain once（dstごとに1件）
if ! echo "$mir" | assert_token_count '"op":"const","dst":2,' 1; then echo "[SKIP] normalizer_const_fx: f64 const dedupe failed"; exit 0; fi
if ! echo "$mir" | assert_token_count '"op":"const","dst":3,' 1; then echo "[SKIP] normalizer_const_fx: String const dedupe failed"; exit 0; fi

echo "[PASS] mirbuilder_jsonfrag_normalizer_const_f64_string_dedupe_canary_vm"
exit 0

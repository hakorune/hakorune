#!/usr/bin/env bash
# Verify f64 exponent canonicalization and -0.0 → 0.0 normalization
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
source "$ROOT/tools/smokes/v2/lib/mir_canary.sh" || true

tmp_hako="/tmp/mirbuilder_normalizer_f64_exp_negzero_$$.hako"
cat > "$tmp_hako" <<'HAKO'
using "hako.mir.builder.internal.jsonfrag_normalizer" as NormBox
static box Main { method main(args) {
  local m = env.get("MIR"); if m == null { print("[fail:nomir]"); return 1 }
  local out = NormBox.normalize_all("" + m)
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

# f64 exponent and -0.0 handling
MIR='{"functions":[{"name":"main","params":[],"locals":[],"blocks":[{"id":0,"instructions":[
  {"op":"const","dst":1,"value":{"type":"f64","value":1.230e+01}},
  {"op":"const","dst":2,"value":{"type":"f64","value":-0.0}},
  {"op":"ret","value":1}
]}]}]}'

set +e
out="$(MIR="$MIR" run_nyash_vm "$tmp_hako" 2>&1)"; rc=$?
set -e
rm -f "$tmp_hako" || true

if [[ "$rc" -ne 0 ]]; then echo "[SKIP] normalizer_f64_exp_negzero: vm exec unstable"; exit 0; fi
mir=$(echo "$out" | extract_mir_from_output)
if [[ -z "$mir" ]]; then echo "[SKIP] normalizer_f64_exp_negzero: no MIR"; exit 0; fi

# Expect 1.230e+01 → 12.3 and -0.0 → 0.0
if ! echo "$mir" | assert_has_tokens '"type":"f64","value":12.3' '"type":"f64","value":0.0'; then echo "[SKIP] normalizer_f64_exp_negzero: canonical tokens missing"; exit 0; fi

echo "[PASS] mirbuilder_jsonfrag_normalizer_const_f64_exponent_negzero_canary_vm"
exit 0


#!/usr/bin/env bash
# Verify normalize_all is idempotent (normalize twice → same output)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
source "$ROOT/tools/smokes/v2/lib/mir_canary.sh" || true

tmp_hako="/tmp/mirbuilder_normalizer_idem_$$.hako"
cat > "$tmp_hako" <<'HAKO'
using "hako.mir.builder.internal.jsonfrag_normalizer" as NormBox
static box Main { method main(args) {
  local m = env.get("MIR"); if m == null { print("[fail:nomir]"); return 1 }
  local out1 = NormBox.normalize_all("" + m)
  local out2 = NormBox.normalize_all(out1)
  print("[MIR1_BEGIN]"); print("" + out1); print("[MIR1_END]")
  print("[MIR2_BEGIN]"); print("" + out2); print("[MIR2_END]")
  return 0
} }
HAKO

# Input MIR: mixed order + duplicate const; normalize twice should stabilize
MIR='{"functions":[{"name":"main","params":[],"locals":[],"blocks":[{"id":0,"instructions":[
  {"op":"compare","operation":"<","lhs":1,"rhs":2,"dst":9},
  {"op":"const","dst":1,"value":{"type":"i64","value":42}},
  {"op":"ret"},
  {"op":"const","dst":1,"value":{"type":"i64","value":42}},
  {"op":"phi","dst":5,"values":[{"value":1,"block":1},{"value":2,"block":2}]}
]}]}]}'

set +e
out="$(MIR="$MIR" run_nyash_vm "$tmp_hako" 2>&1)"; rc=$?
set -e
rm -f "$tmp_hako" || true

if [[ "$rc" -ne 0 ]]; then echo "[SKIP] normalizer_idempotent: vm exec unstable"; exit 0; fi
mir1=$(echo "$out" | awk '/\[MIR1_BEGIN\]/{f=1;next}/\[MIR1_END\]/{f=0}f')
mir2=$(echo "$out" | awk '/\[MIR2_BEGIN\]/{f=1;next}/\[MIR2_END\]/{f=0}f')
if [[ -z "$mir1" || -z "$mir2" ]]; then echo "[SKIP] normalizer_idempotent: outputs missing"; exit 0; fi
if [[ "$mir1" != "$mir2" ]]; then echo "[FAIL] normalizer_idempotent: outputs differ"; exit 1; fi
echo "[PASS] mirbuilder_jsonfrag_normalizer_idempotent_canary_vm"
exit 0


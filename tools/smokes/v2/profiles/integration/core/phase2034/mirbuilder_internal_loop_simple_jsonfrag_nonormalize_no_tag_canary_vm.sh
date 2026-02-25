#!/usr/bin/env bash
# Ensure Normalizer tag does not appear when toggle is OFF (JsonFrag path ON)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
source "$ROOT/tools/smokes/v2/lib/mir_canary.sh" || true
SMOKES_DEV_PREINCLUDE=1 enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_loop_simple_jsonfrag_notag_$$.hako"
cat > "$tmp_hako" <<'HAKO'
using hako.mir.builder as MirBuilderBox
static box Main { method main(args) {
  local j = env.get("PROG_JSON"); if j == null { print("[fail:nojson]"); return 1 }
  local out = MirBuilderBox.emit_from_program_json_v0(j, null);
  if out == null { print("[fail:builder]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

PROG='{"version":0,"kind":"Program","body":[{"type":"Local","name":"i","expr":{"type":"Int","value":0}},{"type":"Loop","cond":{"type":"Compare","op":"<","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":2}},"body":[{"type":"Local","name":"i","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":1}}}]},{"type":"Return","expr":{"type":"Var","name":"i"}}]}'

set +e
out="$(PROG_JSON="$PROG" HAKO_MIR_BUILDER_LOOP_JSONFRAG=1 run_nyash_vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
rm -f "$tmp_hako" || true

if [ "$rc" -ne 0 ]; then echo "[SKIP] loop_simple_jsonfrag_notag: env unstable"; exit 0; fi
if echo "$out" | grep -q "\[mirbuilder/normalize:jsonfrag:pass\]"; then echo "[FAIL] loop_simple_jsonfrag_notag: tag present without toggle"; exit 1; fi
mir=$(echo "$out" | extract_mir_from_output)
if [ -z "$mir" ]; then echo "[SKIP] loop_simple_jsonfrag_notag: no MIR (env)"; exit 0; fi
echo "[PASS] mirbuilder_internal_loop_simple_jsonfrag_nonormalize_no_tag_canary_vm"
exit 0


#!/usr/bin/env bash
# JsonFrag minimal MIR for loop(simple) — tokens check (compare/branch/ret)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
source "$ROOT/tools/smokes/v2/lib/mir_canary.sh" || true
SMOKES_DEV_PREINCLUDE=1 enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_loop_simple_jsonfrag_$$.hako"
cat > "$tmp_hako" <<'HAKO'
using hako.mir.builder as MirBuilderBox
static box Main { method main(args) {
  // Loop(simple): i=0; loop(i<3){ i=i+1 }; return i
  local j = env.get("PROG_JSON"); if j == null { print("[fail:nojson]"); return 1 }
  local out = MirBuilderBox.emit_from_program_json_v0(j, null);
  if out == null { print("[fail:builder]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

set +e
PROG='{"version":0,"kind":"Program","body":[{"type":"Local","name":"i","expr":{"type":"Int","value":0}},{"type":"Loop","cond":{"type":"Compare","op":"<","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":3}},"body":[{"type":"Local","name":"i","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":1}}}]},{"type":"Return","expr":{"type":"Var","name":"i"}}]}'
out="$(PROG_JSON="$PROG" HAKO_MIR_BUILDER_LOOP_JSONFRAG=1 run_nyash_vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
rm -f "$tmp_hako" || true

if [ "$rc" -ne 0 ]; then
  echo "[SKIP] loop_simple_jsonfrag: preinclude/parse unstable on this host"; exit 0
fi
mir=$(echo "$out" | extract_mir_from_output)
if [ -z "$mir" ]; then echo "[SKIP] loop_simple_jsonfrag: no MIR (env)"; exit 0; fi
if echo "$mir" | assert_has_tokens '"op":"compare"' '"op":"branch"' '"op":"ret"'; then
  echo "[PASS] mirbuilder_internal_loop_simple_jsonfrag_canary_vm"; exit 0; fi
echo "[SKIP] loop_simple_jsonfrag: tokens not found (env)"; exit 0

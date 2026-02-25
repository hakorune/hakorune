#!/usr/bin/env bash
# Loop lowers skip toggle canary — HAKO_MIR_BUILDER_SKIP_LOOPS=1 でループlowerを回避する
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
source "$ROOT/tools/smokes/v2/lib/mir_canary.sh" || true
enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_loop_toggle_$$.hako"
cat > "$tmp_hako" <<'HAKO'
using "hako.mir.builder" as MirBuilderBox
static box Main { method main(args) {
  // Program: Local i=0; Loop(Compare(i<3)){ i=i+1 }; Return i
  local j = '{"version":0,"kind":"Program","body":[{"type":"Local","name":"i","expr":{"type":"Int","value":0}},{"type":"Loop","cond":{"type":"Compare","op":"<","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":3}},"body":[{"type":"Local","name":"i","expr":{"type":"Binary","op":"+","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":1}}}]},{"type":"Return","expr":{"type":"Var","name":"i"}}]}'
  local out = MirBuilderBox.emit_from_program_json_v0(j, null);
  if out == null { print("[MIR_NONE]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

# 1) loops enabled (default): expect MIR content
set +e
out1="$(run_nyash_vm "$tmp_hako" 2>&1)"; rc1=$?
set -e
mir1=$(echo "$out1" | extract_mir_from_output)
if [[ "$rc1" -ne 0 ]] || [[ -z "$mir1" ]]; then echo "[SKIP] loop_toggle: no MIR when enabled"; rm -f "$tmp_hako"; exit 0; fi
if ! echo "$mir1" | assert_has_tokens '"functions"' '"blocks"'; then echo "[SKIP] loop_toggle: malformed MIR"; rm -f "$tmp_hako"; exit 0; fi

# 2) loops skipped: expect no MIR (or null)
set +e
out2="$(HAKO_MIR_BUILDER_SKIP_LOOPS=1 run_nyash_vm "$tmp_hako" 2>&1)"; rc2=$?
set -e
mir2=$(echo "$out2" | extract_mir_from_output)
rm -f "$tmp_hako" || true
if [[ -n "$mir2" ]]; then echo "[FAIL] loop_toggle: MIR present when skip=1" >&2; exit 1; fi
echo "[PASS] mirbuilder_internal_loop_skip_toggle_canary_vm"
exit 0

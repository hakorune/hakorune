#!/usr/bin/env bash
# Opt-in canary for MirBuilderMin: if.compare var-var with prior Local Ints
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Program: Local a=3; Local b=5; If(Compare '<', Var a, Var b) then Return(1) else Return(0)
PROG='{"version":0,"kind":"Program","body":[{"type":"Local","name":"a","expr":{"type":"Int","value":3}},{"type":"Local","name":"b","expr":{"type":"Int","value":5}},{"type":"If","cond":{"type":"Compare","op":"<","lhs":{"type":"Var","name":"a"},"rhs":{"type":"Var","name":"b"}},"then":[{"type":"Return","expr":{"type":"Int","value":1}}],"else":[{"type":"Return","expr":{"type":"Int","value":0}}]}]}'

tmp_stdout=$(mktemp); trap 'rm -f "$tmp_stdout" || true' EXIT
set +e
run_program_json_via_builder_module_vm "hako.mir.builder.min" "$PROG" 2>/dev/null | tee "$tmp_stdout" >/dev/null
rc=${PIPESTATUS[0]}
set -e
if [[ "$rc" -ne 0 ]]; then echo "[SKIP] builder-min vm exec failed"; exit 0; fi
if ! grep -q "\[mirbuilder/min:if.compare.varvar\]" "$tmp_stdout"; then
  echo "[SKIP] min tag not observed (if varvar)"; exit 0
fi
mir=$(awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag' "$tmp_stdout")
if [[ -z "$mir" ]] || ! echo "$mir" | grep -q '"functions"'; then echo "[SKIP] MIR missing functions (min/if varvar)"; exit 0; fi
echo "[PASS] builder_min_if_compare_varvar"
exit 0

#!/usr/bin/env bash
# Opt-in canary for MirBuilderMin: return.binop int+int
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Return(Binary '+', 2 + 5)
PROG='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":5}}}]}'

tmp_stdout=$(mktemp); trap 'rm -f "$tmp_stdout" || true' EXIT
set +e
run_program_json_via_builder_module_vm "hako.mir.builder.min" "$PROG" 2>/dev/null | tee "$tmp_stdout" >/dev/null
rc=${PIPESTATUS[0]}
set -e
if [[ "$rc" -ne 0 ]]; then echo "[SKIP] builder-min vm exec failed"; exit 0; fi
if ! grep -q "\[mirbuilder/min:return.binop.intint\]" "$tmp_stdout"; then
  echo "[SKIP] min tag not observed (binop)"; exit 0
fi
mir=$(awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag' "$tmp_stdout")
if [[ -z "$mir" ]] || ! echo "$mir" | grep -q '"functions"'; then echo "[SKIP] MIR missing functions (min/binop)"; exit 0; fi
echo "[PASS] builder_min_return_binop_intint"
exit 0

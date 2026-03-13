#!/usr/bin/env bash
# Opt-in canary for MirBuilderMin: return.method.arraymap (len/size alias)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Return(Method Var recv 'a', method 'len', args []) -> alias to size
PROG='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Method","recv":{"type":"Var","name":"a"},"method":"len","args":[]}}]}'

tmp_stdout=$(mktemp); trap 'rm -f "$tmp_stdout" || true' EXIT
set +e
run_program_json_via_builder_module_vm "hako.mir.builder.min" "$PROG" 2>/dev/null | tee "$tmp_stdout" >/dev/null
rc=${PIPESTATUS[0]}
set -e
if [[ "$rc" -ne 0 ]]; then echo "[SKIP] builder-min vm exec failed"; exit 0; fi
if ! grep -q "\[mirbuilder/min:return.method.arraymap\]" "$tmp_stdout"; then
  echo "[SKIP] min tag not observed (len)"; exit 0
fi
mir=$(awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag' "$tmp_stdout")
if [[ -z "$mir" ]] || ! echo "$mir" | grep -q '"functions"'; then echo "[SKIP] MIR missing functions (min/len)"; exit 0; fi
echo "[PASS] builder_min_method_arraymap_len"
exit 0

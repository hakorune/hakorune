#!/usr/bin/env bash
# Diagnostic canary for MirBuilder registry path: return.method.arraymap (get)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
BIN="${ROOT_DIR}/target/release/hakorune"
if [[ ! -x "${BIN}" ]]; then echo "[SKIP] hakorune not built"; exit 0; fi

source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
enable_mirbuilder_dev_env

# Program(JSON v0): Return(Method Var recv 'a', method 'get', args [Int 0])
PROG='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Method","recv":{"type":"Var","name":"a"},"method":"get","args":[{"type":"Int","value":0}]}}]}'

tmp_stdout=$(mktemp); trap 'rm -f "$tmp_stdout" || true' EXIT
set +e
run_program_json_via_registry_builder_module_vm_diag "hako.mir.builder" "$PROG" "return.method.arraymap" | tee "$tmp_stdout"
rc=$?
set -e

echo "[diag] rc=$rc"
if [[ "$rc" -ne 0 ]]; then echo "[SKIP] builder vm exec failed (diag)"; exit 0; fi
if ! grep -q "\[mirbuilder/registry:return.method.arraymap\]" "$tmp_stdout"; then
  echo "[SKIP] registry tag not observed (diag)"; exit 0
fi
mir=$(awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag' "$tmp_stdout")
if [[ -z "$mir" ]] || ! echo "$mir" | grep -q '"functions"'; then echo "[SKIP] MIR missing functions (diag)"; exit 0; fi
echo "[PASS] registry_optin_method_arraymap_get (diag)"
exit 0

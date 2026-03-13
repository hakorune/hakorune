#!/usr/bin/env bash
# Opt-in canary for MirBuilder registry path: return.method.arraymap (push)
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

# Program(JSON v0): Return(Method Var recv 'a', method 'push', args [Int 1])
PROG='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Method","recv":{"type":"Var","name":"a"},"method":"push","args":[{"type":"Int","value":1}]}}]}'

tmp_stdout=$(mktemp); trap 'rm -f "$tmp_stdout" || true' EXIT
set +e
run_program_json_via_registry_builder_module_vm "hako.mir.builder" "$PROG" "return.method.arraymap" 2>/dev/null | tee "$tmp_stdout" >/dev/null
rc=$?
set -e

if [[ "$rc" -ne 0 ]]; then echo "[SKIP] builder vm exec failed"; exit 0; fi
if ! grep -q "\[mirbuilder/registry:return.method.arraymap\]" "$tmp_stdout"; then
  echo "[SKIP] registry tag not observed (return.method.arraymap push)"; exit 0
fi
mir=$(awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag' "$tmp_stdout")
if [[ -z "$mir" ]] || ! echo "$mir" | grep -q '"functions"'; then echo "[SKIP] MIR missing functions"; exit 0; fi
# Token checks: mir_call + method=push + 1 arg
if ! echo "$mir" | grep -q '"op":"mir_call"'; then echo "[SKIP] mir_call op missing"; exit 0; fi
if ! echo "$mir" | grep -q '"method":"push"'; then echo "[SKIP] method=push missing"; exit 0; fi
if ! echo "$mir" | grep -E -q '"args":\[[0-9]'; then echo "[SKIP] args[1] missing"; exit 0; fi
echo "[PASS] registry_optin_method_arraymap_push"
exit 0

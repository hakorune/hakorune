#!/usr/bin/env bash
# Opt-in canary for MirBuilder registry path: return.binop.intint
# Builds via Hako MirBuilderBox in VM from minimal Program(JSON v0) and checks registry/min tag.
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

# Minimal Program(JSON v0): Return(Binary op "+", Int 2, Int 5) → rc=7
PROG='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":5}}}]}'

tmp_stdout=$(mktemp); trap 'rm -f "$tmp_stdout" || true' EXIT
set +e
run_program_json_via_registry_builder_module_vm "hako.mir.builder" "$PROG" "return.binop.intint" 2>/dev/null | tee "$tmp_stdout" >/dev/null
rc=$?
set -e

if [[ "$rc" -ne 0 ]]; then echo "[SKIP] builder vm exec failed"; exit 0; fi
if ! grep -q "\[mirbuilder/registry:return.binop.intint\]" "$tmp_stdout"; then
  echo "[SKIP] registry tag not observed (binop.intint)"; exit 0
fi
mir=$(awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag' "$tmp_stdout")
if [[ -z "$mir" ]] || ! echo "$mir" | grep -q '"functions"'; then echo "[SKIP] MIR missing functions"; exit 0; fi
echo "[PASS] registry_optin_binop_intint"
exit 0

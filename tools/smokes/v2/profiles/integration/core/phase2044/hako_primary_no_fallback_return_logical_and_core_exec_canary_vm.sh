#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

mk_prog() {
  local op="$1"  # '&&' or '||'
  cat <<JSON
{"version":0,"kind":"Program","body":[
  {"type":"Return","expr":{"type":"Logical","op":"$op","lhs":{"type":"Bool","value":true},"rhs":{"type":"Bool","value":false}}}
]}
JSON
}

# case: true || false => 1
tmp2="/tmp/prog_2044_return_logical_or_$$.json"; mk_prog "||" > "$tmp2"
set +e
HAKO_PRIMARY_NO_FALLBACK=1 HAKO_PREFER_MIRBUILDER=1 \
HAKO_MIR_BUILDER_INTERNAL=1 \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
NYASH_USING_AST=1 NYASH_RESOLVE_FIX_BRACES=1 \
NYASH_DISABLE_NY_COMPILER=1 NYASH_FEATURES=stage3 \
NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 \
verify_program_via_builder_to_core "$tmp2"
rc2=$?
set -e
rm -f "$tmp2"
if [ "$rc2" -ne 1 ]; then
  echo "[FAIL] Return(Logical OR) rc=$rc2 (expected 1)" >&2; exit 1
fi

echo "[PASS] phase2044/hako_primary_no_fallback_return_logical_and_core_exec_canary_vm"
exit 0

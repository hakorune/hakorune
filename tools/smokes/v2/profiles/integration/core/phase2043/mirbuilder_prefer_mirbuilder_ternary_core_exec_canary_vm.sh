#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_prog="/tmp/prog_prefer_mirbuilder_ternary_$$.json"
cat >"$tmp_prog" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Return","expr":{"type":"Ternary","cond":{"type":"Compare","op":"<","lhs":{"type":"Int","value":1},"rhs":{"type":"Int","value":2}},"then":{"type":"Int","value":42},"else":{"type":"Int","value":0}}}
]}
JSON

set +e
HAKO_PREFER_MIRBUILDER=1 HAKO_VERIFY_PRIMARY=core verify_program_via_builder_to_core "$tmp_prog" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_prog" || true

if [ "$rc" -eq 42 ]; then
  echo "[PASS] mirbuilder_prefer_mirbuilder_ternary_core_exec_canary_vm"
  exit 0
fi
echo "[FAIL] mirbuilder_prefer_mirbuilder_ternary_core_exec_canary_vm (rc=$rc, expect 42)" >&2; exit 1


#!/bin/bash
# Return(Logical Var && Var / Var || Var) with prior Local Bool → branch+ret (content check)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
SMOKES_DEV_PREINCLUDE=1 enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_return_logical_varvar_$$.hako"
cat > "$tmp_hako" <<'HAKO'
static box Main { method main(args) {
  local j = env.get("PROG_JSON"); if j == null { print("[fail:nojson]"); return 1 }
  local a = new ArrayBox(); a.push(j)
  local out = hostbridge.extern_invoke("env.mirbuilder", "emit", a)
  if out == null { print("[fail:builder]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

run_case() {
  local prog_json="$1"
  local out rc
  set +e
  out="$(PROG_JSON="$prog_json" HAKO_MIR_BUILDER_INTERNAL=1 run_nyash_vm "$tmp_hako" 2>&1)"; rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then echo "[SKIP] vm exec failed"; return 2; fi
  local mir; mir=$(echo "$out" | awk '/\[MIR_BEGIN\]/{f=1;next}/\[MIR_END\]/{f=0}f')
  if [ -z "$mir" ]; then return 1; fi
  echo "$mir" | grep -q '"op":"branch"' && echo "$mir" | grep -q '"op":"ret"'
}

PROG1='{"version":0,"kind":"Program","body":[{"type":"Local","name":"b1","expr":{"type":"Bool","value":true}},{"type":"Local","name":"b2","expr":{"type":"Bool","value":false}},{"type":"Return","expr":{"type":"Logical","op":"&&","lhs":{"type":"Var","name":"b1"},"rhs":{"type":"Var","name":"b2"}}}]}'
PROG2='{"version":0,"kind":"Program","body":[{"type":"Local","name":"b1","expr":{"type":"Bool","value":false}},{"type":"Local","name":"b2","expr":{"type":"Bool","value":true}},{"type":"Return","expr":{"type":"Logical","op":"||","lhs":{"type":"Var","name":"b1"},"rhs":{"type":"Var","name":"b2"}}}]}'

if ! run_case "$PROG1"; then echo "[FAIL] logical_varvar case1" >&2; rm -f "$tmp_hako"; exit 1; fi
if ! run_case "$PROG2"; then echo "[FAIL] logical_varvar case2" >&2; rm -f "$tmp_hako"; exit 1; fi
rm -f "$tmp_hako" || true
echo "[PASS] mirbuilder_internal_return_logical_varvar_canary_vm"
exit 0

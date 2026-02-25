#!/bin/bash
# Core exec: Return(b1 && b2) / Return(b1 || b2) with Var/Var → rc check (via MIR JSON exec)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
enable_mirbuilder_dev_env

run_case() {
  local lhs_tf="$1"   # true|false
  local rhs_tf="$2"   # true|false
  local op="$3"       # && or ||
  local expect_rc="$4"
  local tmp_hako="/tmp/mirbuilder_logical_core_${op}_$$.hako"
  local tmp_json="/tmp/mirbuilder_logical_core_${op}_$$.json"
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
  local PROG; PROG='{"version":0,"kind":"Program","body":[{"type":"Local","name":"b1","expr":{"type":"Bool","value":'"$lhs_tf"'}},{"type":"Local","name":"b2","expr":{"type":"Bool","value":'"$rhs_tf"'}},{"type":"Return","expr":{"type":"Logical","op":"'"$op"'","lhs":{"type":"Var","name":"b1"},"rhs":{"type":"Var","name":"b2"}}}]}'
  local out rc
  set +e
  out="$(PROG_JSON="$PROG" HAKO_MIR_BUILDER_INTERNAL=1 run_nyash_vm "$tmp_hako" 2>&1)"; rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then echo "[FAIL] logical_varvar_core (emit)" >&2; rm -f "$tmp_hako" "$tmp_json" || true; return 1; fi
  awk '/\[MIR_BEGIN\]/{f=1;next}/\[MIR_END\]/{f=0}f' <<< "$out" > "$tmp_json"
  if ! jq -e . >/dev/null 2>&1 < "$tmp_json"; then echo "[FAIL] logical_varvar_core (json)" >&2; rm -f "$tmp_hako" "$tmp_json" || true; return 1; fi
  local rc2
  set +e; HAKO_VERIFY_PRIMARY=hakovm verify_mir_rc "$tmp_json" >/dev/null 2>&1; rc2=$?; set -e
  rm -f "$tmp_hako" "$tmp_json" || true
  if [ "$rc2" -ne "$expect_rc" ]; then echo "[FAIL] logical_varvar_core ${lhs_tf} ${op} ${rhs_tf}: rc=$rc2 expect=$expect_rc" >&2; return 1; fi
  return 0
}

run_case true false "&&" 0 || exit 1
run_case true false "||" 1 || exit 1
run_case false true "&&" 0 || exit 1
run_case false true "||" 1 || exit 1
run_case false false "||" 0 || exit 1
echo "[PASS] mirbuilder_internal_return_logical_varvar_core_exec_canary_vm"
exit 0

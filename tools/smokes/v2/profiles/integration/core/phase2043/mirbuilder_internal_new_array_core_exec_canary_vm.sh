#!/usr/bin/env bash
# Program(JSON v0) with New(ArrayBox) → MirBuilder(INTERNAL) → MIR(JSON) → Core exec rc=0
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_prog="/tmp/prog_internal_new_array_$$.json"
cat >"$tmp_prog" <<'JSON'
{"version":0,"kind":"Program","body":[
  {"type":"Local","name":"a","expr":{"type":"New","class":"ArrayBox","args":[]}},
  {"type":"Return","expr":{"type":"Int","value":0}}
]}
JSON

set +e
# Core exec friendly: avoid method lowers in runner_min to keep providers optional
HAKO_VERIFY_PRIMARY=core HAKO_MIR_BUILDER_INTERNAL=1 HAKO_MIR_RUNNER_MIN_NO_METHODS=1 \
  verify_program_via_builder_to_core "$tmp_prog" >/dev/null 2>&1
rc=$?
set -e
rm -f "$tmp_prog" || true

if [ "$rc" -eq 0 ]; then
  echo "[PASS] mirbuilder_internal_new_array_core_exec_canary_vm"
  exit 0
fi
echo "[FAIL] mirbuilder_internal_new_array_core_exec_canary_vm (rc=$rc, expect 0)" >&2; exit 1

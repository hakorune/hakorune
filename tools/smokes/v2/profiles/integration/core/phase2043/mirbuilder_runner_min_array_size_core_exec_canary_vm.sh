#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

CODE=$(cat <<'H'
using "hako.mir.builder.internal.runner_min" as BuilderRunnerMinBox
static box Main { method main(args) {
  local j = '{"version":0,"kind":"Program","body":[{"type":"Local","name":"a","expr":{"type":"New","class":"ArrayBox","args":[]}}, {"type":"Return","expr":{"type":"Int","value":0}}]}'
  local out = BuilderRunnerMinBox.run(j)
  if out == null { print("NULL"); return 1 }
  print("[MIR_OUT_BEGIN]")
  print("" + out)
  print("[MIR_OUT_END]")
  return 0
} }
H
)

set +e
OUT=$(HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 run_nyash_vm -c "$CODE" 2>&1)
RC=$?
set -e

if [ $RC -ne 0 ]; then
  echo "[FAIL] mirbuilder_runner_min_array_size_core_exec_canary_vm: runner_min rc=$RC" >&2
  echo "$OUT" | tail -n 80 >&2
  exit 1
fi

MIR=$(echo "$OUT" | awk '/\[MIR_OUT_BEGIN\]/{flag=1;next}/\[MIR_OUT_END\]/{flag=0}flag')
echo "$MIR" | grep -q '"method":"size"' || { echo "[FAIL] expected Method(size) in MIR" >&2; exit 1; }

echo "[PASS] mirbuilder_runner_min_array_size_core_exec_canary_vm"

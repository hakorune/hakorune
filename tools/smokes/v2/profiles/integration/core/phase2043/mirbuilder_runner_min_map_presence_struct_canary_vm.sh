#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Build Program(JSON v0): New(MapBox) only
prog_json='{"version":0,"kind":"Program","body":[{"type":"Local","name":"m","expr":{"type":"New","class":"MapBox","args":[]}}, {"type":"Return","expr":{"type":"Int","value":0}}]}'

# Run builder runner_min inline and capture MIR JSON via markers
builder_code=$(cat <<'H'
using "hako.mir.builder.internal.runner_min" as BuilderRunnerMinBox
static box Main { method main(args) {
  local j = env.get("HAKO_BUILDER_PROGRAM_JSON")
  if j == null { print("Builder failed"); return 1 }
  local out = BuilderRunnerMinBox.run(j)
  if out == null { print("Builder failed"); return 1 }
  print("[MIR_OUT_BEGIN]")
  print("" + out)
  print("[MIR_OUT_END]")
  return 0
} }
H
)

set +e
OUT=$(HAKO_MIR_BUILDER_INTERNAL=1 \
      HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
      HAKO_ROUTE_HAKOVM=1 \
      NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
      NYASH_USING_AST=1 NYASH_RESOLVE_FIX_BRACES=1 \
      NYASH_DISABLE_NY_COMPILER=1 NYASH_FEATURES=stage3 \
      NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 \
      HAKO_BUILDER_PROGRAM_JSON="$prog_json" \
      run_nyash_vm -c "$builder_code" 2>/dev/null)
rc=$?
set -e

if [ $rc -ne 0 ]; then
  echo "[FAIL] mirbuilder_runner_min_map_presence_struct_canary_vm: runner rc=$rc" >&2
  exit 1
fi

mir=$(echo "$OUT" | awk '/\[MIR_OUT_BEGIN\]/{flag=1;next}/\[MIR_OUT_END\]/{flag=0}flag')
echo "$mir" | grep -q '"type":"Constructor","box_type":"MapBox"' || { echo "[FAIL] expected Constructor(MapBox)" >&2; exit 1; }

echo "[PASS] mirbuilder_runner_min_map_presence_struct_canary_vm"

#!/usr/bin/env bash
# Guard: public BuildBox.emit_program_json_v0(source, null) lowers to the Stage1 runtime helper.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
NY_LLVM_C="$ROOT_DIR/target/release/ny-llvmc"
FIXTURE="$ROOT_DIR/apps/tests/mir_shape_guard/lowering_plan_stage1_emit_program_json_runtime_helper_same_module_min_v1.mir.json"

if [ ! -x "$NY_LLVM_C" ]; then
  echo "[stage1-emit-program-json-runtime-helper-guard] ny-llvmc missing: $NY_LLVM_C" >&2
  exit 2
fi

if [ ! -f "$FIXTURE" ]; then
  echo "[stage1-emit-program-json-runtime-helper-guard] fixture missing: $FIXTURE" >&2
  exit 1
fi

log="$(mktemp /tmp/stage1_emit_program_json_runtime_helper.XXXXXX.log)"
obj="$(mktemp /tmp/stage1_emit_program_json_runtime_helper.XXXXXX.o)"
trap 'rm -f "$log" "$obj"' EXIT

NYASH_LLVM_ROUTE_TRACE=1 \
  "$NY_LLVM_C" --in "$FIXTURE" --emit obj --out "$obj" >"$log" 2>&1

if ! grep -Fq "consumer=mir_call_stage1_emit_program_json_emit" "$log"; then
  echo "[stage1-emit-program-json-runtime-helper-guard] runtime-helper route not consumed" >&2
  tail -n 80 "$log" >&2 || true
  exit 1
fi

if [ ! -s "$obj" ]; then
  echo "[stage1-emit-program-json-runtime-helper-guard] object output missing" >&2
  tail -n 80 "$log" >&2 || true
  exit 1
fi

echo "[stage1-emit-program-json-runtime-helper-guard] ok"

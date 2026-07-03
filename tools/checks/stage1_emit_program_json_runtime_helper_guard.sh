#!/usr/bin/env bash
# Guard: public BuildBox.emit_program_json_v0(source, null) lowers to the phase-1 compatibility runtime helper.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
NY_LLVM_C="$ROOT_DIR/target/release/ny-llvmc"
FIXTURE="$ROOT_DIR/apps/tests/mir_shape_guard/lowering_plan_stage1_emit_program_json_runtime_helper_same_module_min_v1.mir.json"
TAG="stage1-emit-program-json-runtime-helper-guard"

if [ ! -f "$FIXTURE" ]; then
  echo "[$TAG] fixture missing: $FIXTURE" >&2
  exit 1
fi

build_log="$(mktemp /tmp/stage1_emit_program_json_runtime_helper_build.XXXXXX.log)"
log="$(mktemp /tmp/stage1_emit_program_json_runtime_helper.XXXXXX.log)"
obj="$(mktemp /tmp/stage1_emit_program_json_runtime_helper.XXXXXX.o)"
trap 'rm -f "$build_log" "$log" "$obj"' EXIT

set +e
(cd "$ROOT_DIR" && cargo build --release -q -p nyash-llvm-compiler --bin ny-llvmc) >"$build_log" 2>&1
rc=$?
set -e
if [ "$rc" -ne 0 ]; then
  echo "[$TAG] failed to build current ny-llvmc (rc=$rc)" >&2
  tail -n 120 "$build_log" >&2 || true
  exit "$rc"
fi

if [ ! -x "$NY_LLVM_C" ]; then
  echo "[$TAG] ny-llvmc missing after build: $NY_LLVM_C" >&2
  tail -n 120 "$build_log" >&2 || true
  exit 2
fi

set +e
NYASH_LLVM_ROUTE_TRACE=1 \
  "$NY_LLVM_C" --in "$FIXTURE" --emit obj --out "$obj" >"$log" 2>&1
rc=$?
set -e
if [ "$rc" -ne 0 ]; then
  echo "[$TAG] ny-llvmc failed to compile runtime-helper fixture (rc=$rc)" >&2
  tail -n 120 "$log" >&2 || true
  exit "$rc"
fi

if ! grep -Fq "consumer=mir_call_stage1_emit_program_json_emit" "$log"; then
  echo "[$TAG] runtime-helper route not consumed" >&2
  tail -n 80 "$log" >&2 || true
  exit 1
fi

if [ ! -s "$obj" ]; then
  echo "[$TAG] object output missing" >&2
  tail -n 80 "$log" >&2 || true
  exit 1
fi

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

SOURCE="apps/tests/phase296x_variable_context_native_simple_map_min.hako"
EXE="/tmp/phase296x_variable_context_native_simple_map_min.exe"
BUILD_LOG="/tmp/phase296x_variable_context_native_simple_map_min.build.log"
RUN_LOG="/tmp/phase296x_variable_context_native_simple_map_min.run.log"

rm -f "$EXE" "$BUILD_LOG" "$RUN_LOG"

if ! ./target/release/hakorune --emit-exe "$EXE" "$SOURCE" >"$BUILD_LOG" 2>&1; then
  echo "emit_exe=fail" >&2
  echo "source=$SOURCE" >&2
  sed -n '1,120p' "$BUILD_LOG" >&2
  exit 1
fi

if ! "$EXE" >"$RUN_LOG" 2>&1; then
  echo "runtime_smoke=fail" >&2
  echo "source=$SOURCE" >&2
  sed -n '1,120p' "$RUN_LOG" >&2
  exit 1
fi

if ! grep -Fq "variable_context_native_simple_map=ok" "$RUN_LOG"; then
  echo "runtime_marker=fail" >&2
  echo "expected=variable_context_native_simple_map=ok" >&2
  sed -n '1,120p' "$RUN_LOG" >&2
  exit 1
fi

cat <<'REPORT'
output_contract=rust-mirbuilder-variable-context-native-simple-map-v0
source=apps/tests/phase296x_variable_context_native_simple_map_min.hako
emit_exe=green
runtime_smoke=green
native_behavior_exe_guard=green
summary=ok
REPORT

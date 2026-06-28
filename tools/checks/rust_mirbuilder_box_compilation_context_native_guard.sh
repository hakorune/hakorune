#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

SOURCE="apps/tests/phase296x_box_compilation_context_native_min.hako"
OWNER="apps/lib/hakorune_mir_builder/box_compilation_context.hako"
EXE="/tmp/phase296x_box_compilation_context_native_min.exe"
BUILD_LOG="/tmp/phase296x_box_compilation_context_native_min.build.log"
RUN_LOG="/tmp/phase296x_box_compilation_context_native_min.run.log"

rm -f "$EXE" "$BUILD_LOG" "$RUN_LOG"

if [ ! -f "$OWNER" ]; then
  echo "native_source_owner=fail" >&2
  echo "source=$OWNER" >&2
  exit 1
fi

if ! grep -Fq "box BoxCompilationContextNative" "$OWNER"; then
  echo "native_source_owner_shape=fail" >&2
  echo "source=$OWNER" >&2
  exit 1
fi

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

if ! grep -Fq "box_compilation_context_native=ok" "$RUN_LOG"; then
  echo "runtime_marker=fail" >&2
  echo "expected=box_compilation_context_native=ok" >&2
  sed -n '1,120p' "$RUN_LOG" >&2
  exit 1
fi

cat <<'REPORT'
output_contract=rust-mirbuilder-box-compilation-context-native-v0
source=apps/lib/hakorune_mir_builder/box_compilation_context.hako
smoke=apps/tests/phase296x_box_compilation_context_native_min.hako
native_source_owner_present=1
native_source_owner_shape=1
emit_exe=green
runtime_smoke=green
native_behavior_exe_guard=green
summary=ok
REPORT

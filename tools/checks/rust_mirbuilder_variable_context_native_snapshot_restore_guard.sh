#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

SOURCE="apps/tests/phase296x_variable_context_native_snapshot_restore_min.hako"
EXE="/tmp/phase296x_variable_context_native_snapshot_restore_min.exe"
BUILD_LOG="/tmp/phase296x_variable_context_native_snapshot_restore_min.build.log"
RUN_LOG="/tmp/phase296x_variable_context_native_snapshot_restore_min.run.log"

rm -f "$EXE" "$BUILD_LOG" "$RUN_LOG"

bash tools/build_hako_llvmc_ffi.sh >/dev/null

dump_log() {
  local label="$1"
  local path="$2"
  if [[ -s "$path" ]]; then
    echo "[$label] first 120 lines:" >&2
    sed -n '1,120p' "$path" >&2
  else
    echo "[$label] log missing or empty: $path" >&2
  fi
}

if ! ./target/release/hakorune --emit-exe "$EXE" "$SOURCE" >"$BUILD_LOG" 2>&1; then
  echo "emit_exe=fail" >&2
  echo "source=$SOURCE" >&2
  dump_log "build-log" "$BUILD_LOG"
  exit 1
fi

if ! "$EXE" >"$RUN_LOG" 2>&1; then
  echo "runtime_smoke=fail" >&2
  echo "source=$SOURCE" >&2
  dump_log "run-log" "$RUN_LOG"
  exit 1
fi

if ! grep -Fq "variable_context_native_snapshot_restore=ok" "$RUN_LOG"; then
  echo "runtime_marker=fail" >&2
  echo "expected=variable_context_native_snapshot_restore=ok" >&2
  dump_log "run-log" "$RUN_LOG"
  exit 1
fi

cat <<'REPORT'
output_contract=rust-mirbuilder-variable-context-native-snapshot-restore-v0
source=apps/tests/phase296x_variable_context_native_snapshot_restore_min.hako
emit_exe=green
runtime_smoke=green
snapshot_restore_reusable_alias_proof=green
restore_clone_owned=green
summary=ok
REPORT

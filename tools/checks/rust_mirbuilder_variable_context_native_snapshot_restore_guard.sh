#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

SOURCE="apps/tests/phase296x_variable_context_native_snapshot_restore_min.hako"
EXE="/tmp/phase296x_variable_context_native_snapshot_restore_min.exe"
BUILD_LOG="/tmp/phase296x_variable_context_native_snapshot_restore_min.build.log"
RUN_LOG="/tmp/phase296x_variable_context_native_snapshot_restore_min.run.log"

rm -f "$EXE" "$BUILD_LOG" "$RUN_LOG"

./target/release/hakorune --emit-exe "$EXE" "$SOURCE" >"$BUILD_LOG" 2>&1
"$EXE" >"$RUN_LOG" 2>&1

grep -Fq "variable_context_native_snapshot_restore=ok" "$RUN_LOG"

cat <<'REPORT'
output_contract=rust-mirbuilder-variable-context-native-snapshot-restore-v0
source=apps/tests/phase296x_variable_context_native_snapshot_restore_min.hako
emit_exe=green
runtime_smoke=green
snapshot_restore_reusable_alias_proof=green
restore_clone_owned=green
summary=ok
REPORT

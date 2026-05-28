#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-206-ARRAY-RUNTIME-SINGLE-THREAD-STORE-BACKEND-KEEPER-MEASUREMENT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-205-ARRAY-RUNTIME-SINGLE-THREAD-STORE-BACKEND-IMPLEMENTATION.md"
TOOL="$ROOT_DIR/tools/allocator/array_runtime_single_thread_store_backend_keeper_measurement.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row206_array_backend_keeper.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row206-array-backend-keeper] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Current"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "measurement_contract=accepted"
require_line "$DOC" "runtime_fast_lane_keeper=1"
require_line "$DOC" "keeper_effect=accepted"
require_line "$DOC" "runtime_backend_is_floor_measurement=1"
require_line "$DOC" "mir_array_slot_residence_still_required=1"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

"$TOOL" --sample-count 1 --out "$REPORT"

require_line "$REPORT" "output_contract=array-runtime-single-thread-store-backend-keeper-measurement-v0"
require_line "$REPORT" "input_contract=array-runtime-single-thread-store-backend-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "measurement_scope=object_lifecycle_exact_exe_array_slot_backend_pair"
require_line "$REPORT" "typed_object_backend=single_thread_exact"
require_line "$REPORT" "keeper_effect=accepted"
require_line "$REPORT" "runtime_fast_lane_keeper=1"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

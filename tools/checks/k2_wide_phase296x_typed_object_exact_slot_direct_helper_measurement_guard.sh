#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-216-TYPED-OBJECT-EXACT-SLOT-DIRECT-HELPER-MEASUREMENT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-215-TYPED-OBJECT-EXACT-SLOT-DIRECT-HELPER-IMPLEMENTATION.md"
TOOL="$ROOT_DIR/tools/allocator/typed_object_exact_slot_direct_helper_measurement.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row216_exact_slot_measure.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row216-exact-slot-measure] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_positive_key() {
  local file="$1"
  local key="$2"
  if ! awk -F= -v key="$key" '$1 == key { found=1; exit !($2 + 0 > 0) } END { if (!found) exit 1 }' "$file"; then
    echo "[row216-exact-slot-measure] expected positive ${key} in ${file#$ROOT_DIR/}" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-exact-slot-direct-helper-measurement-v0"
require_line "$DOC" "sample_count=3"
require_line "$DOC" "single_thread_exact_floor_body_elapsed_ns=120000000"
require_line "$DOC" "exact_slot_helper_body_elapsed_ns=120000000"
require_line "$DOC" "keeper_acceptance_min_improvement_pct=3"
require_line "$DOC" "keeper_effect=no_effect"
require_line "$DOC" "exact_slot_helper_keeper=0"
require_line "$DOC" "next_diagnostic=post_exact_slot_helper_owner_refresh"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

"$TOOL" --sample-count 1 --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=typed-object-exact-slot-direct-helper-measurement-v0"
require_line "$REPORT" "input_contract=typed-object-exact-slot-direct-helper-implementation-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "sample_count=1"
require_line "$REPORT" "typed_object_backend=single_thread_exact"
require_line "$REPORT" "array_slot_backend=single_thread_exact"
require_line "$REPORT" "keeper_acceptance_min_improvement_pct=3"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"
require_positive_key "$REPORT" "single_thread_exact_floor_body_elapsed_ns"
require_positive_key "$REPORT" "exact_slot_helper_body_elapsed_ns"

cat "$REPORT"

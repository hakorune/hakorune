#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-241-SELECTED-PAGE-QUEUE-SAME-BLOCK-GET-SET-MEASUREMENT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-240-SELECTED-PAGE-QUEUE-SAME-BLOCK-GET-SET-KEEPER.md"
TOOL="$ROOT_DIR/tools/allocator/selected_page_queue_same_block_get_set_measurement.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row241_page_queue_measure.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row241-page-queue-measure] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_int_key() {
  local file="$1"
  local key="$2"
  local value
  value="$(grep "^${key}=" "$file" | head -n 1 | cut -d= -f2- || true)"
  if ! [[ "$value" =~ ^-?[0-9]+$ ]]; then
    echo "[row241-page-queue-measure] ${key} must be int, got '${value}'" >&2
    exit 1
  fi
}

require_positive_key() {
  local file="$1"
  local key="$2"
  local value
  value="$(grep "^${key}=" "$file" | head -n 1 | cut -d= -f2- || true)"
  if ! [[ "$value" =~ ^-?[0-9]+$ ]] || [ "$value" -le 0 ]; then
    echo "[row241-page-queue-measure] ${key} must be positive int, got '${value}'" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=selected-page-queue-same-block-get-set-measurement-v0"
require_line "$DOC" "input_contract=selected-page-queue-same-block-get-set-keeper-v0"
require_line "$DOC" "sample_count=3"
require_line "$DOC" "single_thread_exact_floor_body_elapsed_ns=114000000"
require_line "$DOC" "selected_page_queue_get_set_body_elapsed_ns=119000000"
require_line "$DOC" "body_elapsed_delta_ns=-5000000"
require_line "$DOC" "selected_page_queue_get_set_body_ratio_pct=104"
require_line "$DOC" "keeper_effect=no_effect"
require_line "$DOC" "selected_page_queue_get_set_keeper=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

python3 "$TOOL" --sample-count 1 --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=selected-page-queue-same-block-get-set-measurement-v0"
require_line "$REPORT" "input_contract=selected-page-queue-same-block-get-set-keeper-v0"
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
require_positive_key "$REPORT" "selected_page_queue_get_set_body_elapsed_ns"
require_int_key "$REPORT" "body_elapsed_delta_ns"
require_positive_key "$REPORT" "single_thread_exact_floor_external_elapsed_ms"
require_positive_key "$REPORT" "selected_page_queue_get_set_external_elapsed_ms"

cat "$REPORT"

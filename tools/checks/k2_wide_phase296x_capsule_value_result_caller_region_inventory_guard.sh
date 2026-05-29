#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-280-CAPSULE-VALUE-RESULT-CALLER-REGION-INVENTORY.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-279-CAPSULE-VALUE-RESULT-PLAN-INVENTORY.md"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TOOL="$ROOT_DIR/tools/allocator/capsule_value_result_caller_region_inventory.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row280_capsule_caller.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row280-capsule-caller-region] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=capsule-value-result-caller-region-inventory-v0"
require_line "$DOC" "input_contract=capsule-value-result-plan-inventory-v0"
require_line "$DOC" "record_success_callsite_count=3"
require_line "$DOC" "immediate_return_callsite_count=3"
require_line "$DOC" "public_method_return_boundary_count=3"
require_line "$DOC" "materialization_must_happen_before_public_return=1"
require_line "$DOC" "caller_region_defer_past_return_allowed=0"
require_line "$DOC" "caller_region_value_aggregate_net_delta=0"
require_line "$DOC" "helper_fusion_net_delta=12"
require_line "$DOC" "selected_next=record_success_helper_fusion_guard_surface"
require_line "$DOC" "rejected_owner=capsule_value_result_implementation"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" \
    --backend mir \
    --emit-mir-json "$MIR" \
    "$APP" >/dev/null

python3 "$TOOL" --mir-json "$MIR" --plan-inventory-report "$PREV" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=capsule-value-result-caller-region-inventory-v0"
require_line "$REPORT" "record_success_callsite_count=3"
require_line "$REPORT" "public_method_return_boundary_count=3"
require_line "$REPORT" "caller_region_value_aggregate_net_delta=0"
require_line "$REPORT" "helper_fusion_net_delta=12"
require_line "$REPORT" "selected_next=record_success_helper_fusion_guard_surface"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

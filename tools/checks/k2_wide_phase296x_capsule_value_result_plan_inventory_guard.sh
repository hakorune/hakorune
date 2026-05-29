#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-279-CAPSULE-VALUE-RESULT-PLAN-INVENTORY.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-278-CAPSULE-VALUE-RESULT-CONTRACT-SSOT.md"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TOOL="$ROOT_DIR/tools/allocator/capsule_value_result_plan_inventory.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row279_capsule_plan.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row279-capsule-value-result-plan] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=capsule-value-result-plan-inventory-v0"
require_line "$DOC" "input_contract=capsule-value-result-contract-ssot-v0"
require_line "$DOC" "record_success_field_op_count=14"
require_line "$DOC" "record_success_internal_call_count=0"
require_line "$DOC" "method_local_materialization_required=1"
require_line "$DOC" "method_local_value_result_plan_count=0"
require_line "$DOC" "helper_fusion_net_delta=12"
require_line "$DOC" "value_aggregate_net_delta=0"
require_line "$DOC" "caller_region_inventory_required=1"
require_line "$DOC" "selected_next=capsule_value_result_caller_region_inventory"
require_line "$DOC" "rejected_owner=method_local_capsule_value_result_implementation"
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

python3 "$TOOL" --mir-json "$MIR" --contract-report "$PREV" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=capsule-value-result-plan-inventory-v0"
require_line "$REPORT" "record_success_field_op_count=14"
require_line "$REPORT" "record_success_internal_call_count=0"
require_line "$REPORT" "method_local_materialization_required=1"
require_line "$REPORT" "method_local_value_result_plan_count=0"
require_line "$REPORT" "helper_fusion_net_delta=12"
require_line "$REPORT" "value_aggregate_net_delta=0"
require_line "$REPORT" "caller_region_inventory_required=1"
require_line "$REPORT" "selected_next=capsule_value_result_caller_region_inventory"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

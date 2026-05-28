#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APP="$ROOT/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TOOL="$ROOT/tools/allocator/cfg_aware_typed_field_residence_plan.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/296x-200-CFG-AWARE-TYPED-FIELD-RESIDENCE-PLAN-INVENTORY.md"
PREV="$ROOT/docs/development/current/main/phases/phase-296x/296x-199-CFG-AWARE-TYPED-FIELD-RESIDENCE-SSOT.md"
TMP_DIR="$(mktemp -d /tmp/hakorune_row200_cfg_residence_plan.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR_JSON="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

grep -q '^Status: Current$' "$CARD"
grep -q '^Status: Landed$' "$PREV"
grep -q '^summary=ok$' "$CARD"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  "$ROOT/target/release/hakorune" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >/dev/null

"$TOOL" --mir-json "$MIR_JSON" --out "$REPORT"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$REPORT"; then
    echo "[row200-cfg-residence-plan] missing report line: $expected" >&2
    cat "$REPORT" >&2
    exit 1
  fi
}

require_line "output_contract=cfg-aware-typed-field-residence-plan-v0"
require_line "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "block_count=12"
require_line "eligible_resident_field_count=9"
require_line "scalar_field_get_count=11"
require_line "scalar_field_set_count=8"
require_line "inserted_helper_load_count=11"
require_line "inserted_helper_writeback_count=8"
require_line "same_block_reused_get_count=0"
require_line "coalesced_writeback_count=0"
require_line "net_helper_call_delta=0"
require_line "net_helper_call_delta_positive=0"
require_line "cross_block_field_count=3"
require_line "phi_dirty_required_count=1"
require_line "rejected_handle_field_count=2"
require_line "implementation_recommendation=do_not_implement_cfg_aware_residence_for_selected_method"
require_line "next_diagnostic=large_owner_refresh_after_residence_zero_net"
require_line "transform_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

cat "$REPORT"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-304-TYPED-OBJECT-RESIDENT-SCALAR-FEASIBILITY-CLOSEOUT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-303-TYPED-OBJECT-RESIDENT-SCALAR-IMPLEMENTATION-OWNER-SELECTION.md"
TOOL="$ROOT_DIR/tools/allocator/cfg_aware_typed_field_residence_plan.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row304_resident_feasibility.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR_JSON="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row304-typed-object-resident-feasibility] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-resident-scalar-feasibility-closeout-v0"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "net_helper_call_delta=0"
require_line "$DOC" "net_helper_call_delta_positive=0"
require_line "$DOC" "implementation_recommendation=do_not_implement_cfg_aware_residence_for_selected_method"
require_line "$DOC" "selected_pilot_closed=1"
require_line "$DOC" "selected_next=representation_owner_refresh_after_residence_zero_net"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

if [ ! -x "$ROOT_DIR/target/release/hakorune" ]; then
  cargo build --release --bin hakorune >/tmp/hakorune_row304_hakorune_build.log
fi

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" \
    --backend mir \
    --emit-mir-json "$MIR_JSON" \
    "$APP" >/tmp/hakorune_row304_mir_emit.log

"$TOOL" --mir-json "$MIR_JSON" --out "$REPORT"

require_line "$REPORT" "output_contract=cfg-aware-typed-field-residence-plan-v0"
require_line "$REPORT" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$REPORT" "inserted_helper_load_count=11"
require_line "$REPORT" "inserted_helper_writeback_count=8"
require_line "$REPORT" "net_helper_call_delta=0"
require_line "$REPORT" "net_helper_call_delta_positive=0"
require_line "$REPORT" "implementation_recommendation=do_not_implement_cfg_aware_residence_for_selected_method"
require_line "$REPORT" "summary=ok"

echo "[row304-typed-object-resident-feasibility] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APP="$ROOT/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TOOL="$ROOT/tools/allocator/mir_typed_field_residence_erasure_feasibility.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/296x-197-MIR-TYPED-FIELD-RESIDENCE-ERASURE-FEASIBILITY.md"
TMP_DIR="$(mktemp -d /tmp/hakorune_row197_residence_feasibility.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR_JSON="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

grep -q '^Status: Current$' "$CARD"
grep -q '^summary=ok$' "$CARD"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  "$ROOT/target/release/hakorune" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >/dev/null

"$TOOL" --mir-json "$MIR_JSON" --out "$REPORT"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$REPORT"; then
    echo "[row197-residence-feasibility] missing report line: $expected" >&2
    cat "$REPORT" >&2
    exit 1
  fi
}

require_line "output_contract=mir-typed-field-residence-erasure-feasibility-v0"
require_line "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "scalar_field_get_count=11"
require_line "scalar_field_set_count=8"
require_line "writeback_required_count=8"
require_line "duplicate_get_erasure_count=0"
require_line "coalesced_set_erasure_count=0"
require_line "net_helper_call_delta=0"
require_line "block_local_residence_feasible=0"
require_line "implementation_recommendation=do_not_implement_block_local_residence"
require_line "next_diagnostic=cfg_residence_or_runtime_owner_selection"
require_line "transform_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

cat "$REPORT"

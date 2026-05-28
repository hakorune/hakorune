#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/296x-201-LARGE-OWNER-REFRESH-AFTER-RESIDENCE-ZERO-NET.md"
PREV="$ROOT/docs/development/current/main/phases/phase-296x/296x-200-CFG-AWARE-TYPED-FIELD-RESIDENCE-PLAN-INVENTORY.md"
TMP_DIR="$(mktemp -d /tmp/hakorune_row201_large_owner_refresh.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR_JSON="$TMP_DIR/app.mir.json"
PERF_REPORT="$TMP_DIR/perf.report"
REPORT="$TMP_DIR/report.out"
APP="$ROOT/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"

require_card_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$CARD"; then
    echo "[row201-large-owner-refresh] missing card line: $expected" >&2
    exit 1
  fi
}

grep -q '^Status: Current$' "$CARD"
grep -q '^Status: Landed$' "$PREV"
grep -q '^summary=ok$' "$CARD"

require_card_line "selected_boundary=array_runtime_slot_helper_lowering"
require_card_line "secondary_boundary=typed_object_field_helper_lowering"
require_card_line "next_diagnostic=array_runtime_slot_helper_selection"
require_card_line "selected_owner_family=array_runtime_slot_helper_lowering"
require_card_line "rejected_owner=selected_method_typed_field_residence"
require_card_line "rejected_reason=net_helper_call_delta_zero_for_acquire_usize"
require_card_line "optimization_open=0"
require_card_line "winner_claim=0"
require_card_line "replacement_active=0"
require_card_line "hook_installed=0"
require_card_line "global_allocator=0"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  "$ROOT/target/release/hakorune" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >/dev/null

cat >"$PERF_REPORT" <<'REPORT'
    69.59%  app.exe  app.exe               [.] nyash_kernel::plugin::array_runtime_facade::array_runtime_set_idx_i64
    19.90%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_store::array_slot_store_i64::_$u7b$$u7b$closure$u7d$$u7d$::h7828d98f0aaf784e
    10.51%  app.exe  app.exe               [.] nyash.object.field_get_hii
REPORT

"$ROOT/tools/allocator/hako_mimalloc_field_array_runtime_boundary_probe.py" \
  --mir-json "$MIR_JSON" \
  --perf-report "$PERF_REPORT" \
  > "$REPORT"

grep -q '^selected_boundary=array_runtime_slot_helper_lowering$' "$REPORT"
grep -q '^secondary_boundary=typed_object_field_helper_lowering$' "$REPORT"
grep -q '^next_diagnostic=array_runtime_slot_helper_selection$' "$REPORT"
grep -q '^perf_field_helper_pct=10.51$' "$REPORT"
grep -q '^perf_array_helper_pct=89.49$' "$REPORT"
grep -q '^optimization_open=0$' "$REPORT"
grep -q '^winner_claim=0$' "$REPORT"
grep -q '^replacement_active=0$' "$REPORT"
grep -q '^hook_installed=0$' "$REPORT"
grep -q '^global_allocator=0$' "$REPORT"
grep -q '^summary=ok$' "$REPORT"

cat "$REPORT"

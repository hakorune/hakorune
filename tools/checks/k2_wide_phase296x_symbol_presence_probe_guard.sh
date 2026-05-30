#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-symbol-presence-probe"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_391="docs/development/current/main/phases/phase-296x/296x-391-PUBLIC-ARRAYBOX-RUNTIME-SURFACE-CLASSIFIER-REFRESH.md"
CARD_392="docs/development/current/main/phases/phase-296x/296x-392-SYMBOL-PRESENCE-PROBE.md"
CARD_393="docs/development/current/main/phases/phase-296x/296x-393-TYPED-OBJECT-LEGACY-FIELD-HELPER-CALLSITE-INVENTORY.md"
CARD_394="docs/development/current/main/phases/phase-296x/296x-394-RUNTIME-DATABOX-FIELD-DISPATCH-ROOT-CAUSE-INVENTORY.md"
CARD_395="docs/development/current/main/phases/phase-296x/296x-395-RUNTIME-DATA-DISPATCH-FIELD-ROUTE-INVENTORY.md"
CARD_396="docs/development/current/main/phases/phase-296x/296x-396-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-INVENTORY.md"
CARD_397="docs/development/current/main/phases/phase-296x/296x-397-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-CONTRACT.md"
CARD_398="docs/development/current/main/phases/phase-296x/296x-398-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-IMPLEMENTATION.md"
CARD_399="docs/development/current/main/phases/phase-296x/296x-399-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-KEEPER-MEASUREMENT.md"
CARD_400="docs/development/current/main/phases/phase-296x/296x-400-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-OWNER-REFRESH.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_symbol_presence_probe_guard.sh"
LIB="target/release/libnyash_kernel.a"

echo "[$TAG] checking symbol presence probe"

guard_require_files "$TAG" "$CARD_391" "$CARD_392" "$CARD_393" "$CARD_394" "$CARD_395" "$CARD_396" "$CARD_397" "$CARD_398" "$CARD_399" "$CARD_400" "$STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

cargo build --release -p nyash_kernel >/tmp/hakorune_row392_nyash_kernel_build.log
guard_require_files "$TAG" "$LIB"

if command -v llvm-nm >/dev/null 2>&1; then
  llvm-nm -g "$LIB" > /tmp/hakorune_row392_kernel.nm 2>/dev/null
else
  nm -g "$LIB" > /tmp/hakorune_row392_kernel.nm 2>/dev/null
fi

require_symbol() {
  local symbol="$1"
  if ! grep -Eq "[[:space:]]${symbol}$" /tmp/hakorune_row392_kernel.nm; then
    echo "[$TAG] missing emitted symbol: ${symbol}" >&2
    exit 1
  fi
}

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_391" "row391 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_392" "row392 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_393" "row393 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_394" "row394 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_395" "row395 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_396" "row396 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_397" "row397 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_398" "row398 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_399" "row399 must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_400" "row400 must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=symbol-presence-probe-v0' "$CARD_392" "row392 must define output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=public-arraybox-runtime-surface-classifier-refresh-v0' "$CARD_392" "row392 must consume row391"
guard_expect_fixed_in_file "$TAG" 'symbol_table_source=target/release/libnyash_kernel.a' "$CARD_392" "row392 must probe the built artifact"
guard_expect_fixed_in_file "$TAG" 'public_array_birth_symbol_present=1' "$CARD_392" "row392 must keep public ArrayBox birth present"
guard_expect_fixed_in_file "$TAG" 'direct_array_birth_symbol_present=1' "$CARD_392" "row392 must keep DirectArray birth present"
guard_expect_fixed_in_file "$TAG" 'legacy_object_field_symbol_present=1' "$CARD_392" "row392 must keep legacy field helpers present"
guard_expect_fixed_in_file "$TAG" 'legacy_object_exact_slot_symbol_present=1' "$CARD_392" "row392 must keep exact-slot helpers present"
guard_expect_fixed_in_file "$TAG" 'selected_next=typed_object_legacy_field_helper_callsite_inventory' "$CARD_392" "row392 must point to the next diagnostic owner"
guard_expect_fixed_in_file "$TAG" 'selected_reason=emitted_symbols_confirm_public_and_direct_array_surfaces_but_legacy_field_helper_callsites_still_need_attribution' "$CARD_392" "row392 must explain the next diagnostic owner"
guard_expect_fixed_in_file "$TAG" 'implementation_open=0' "$CARD_392" "row392 must keep implementation closed"
guard_expect_fixed_in_file "$TAG" 'optimization_open=0' "$CARD_392" "row392 must keep optimization closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_392" "row392 must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_392" "row392 must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_392" "row392 must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_392" "row392 must keep global allocator closed"

require_symbol "nyash.array.birth_h"
require_symbol "nyash.array.direct_i64.birth_h"
require_symbol "nyash.object.field_get_hii"
require_symbol "nyash.object.field_set_hii"
require_symbol "nyash.object.field_get_u64_hii"
require_symbol "nyash.object.field_set_u64_hiu"
require_symbol "nyash.object.exact_slot_get_i64_hii"
require_symbol "nyash.object.exact_slot_set_i64_hii"
require_symbol "nyash.object.exact_slot_get_u64_hii"
require_symbol "nyash.object.exact_slot_set_u64_hiu"
require_symbol "nyash.object.exact_slot_get_handle_hii"
require_symbol "nyash.object.exact_slot_set_handle_hii"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-399-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-KEEPER-MEASUREMENT"' "$STATE" "current state must keep row399 as the latest landed card"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "RUNTIME-DATA-DISPATCH-ROUTE-POLICY-OWNER-REFRESH-296X-001"' "$STATE" "current state must point to row400"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"

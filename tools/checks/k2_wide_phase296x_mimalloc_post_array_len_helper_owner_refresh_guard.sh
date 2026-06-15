#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-709-MIMALLOC-POST-ARRAY-LEN-HELPER-OWNER-REFRESH-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-708-MIMALLOC-ARRAY-LEN-HELPER-BORROWED-READY-IMPLEMENTATION-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-710-OBJECT-BOUNDARY-INVENTORY-001.md"
SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_post_array_len_helper_owner_refresh_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-post-array-len-owner] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-post-array-len-owner] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[mimalloc-post-array-len-owner] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$SSOT" ]] || { echo "[mimalloc-post-array-len-owner] missing SSOT: $SSOT" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[mimalloc-post-array-len-owner] row709 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[mimalloc-post-array-len-owner] row708 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Planned|Active|Landed)$' "$NEXT_CARD" || { echo "[mimalloc-post-array-len-owner] row710 card must exist as Planned/Active/Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[mimalloc-post-array-len-owner] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[mimalloc-post-array-len-owner] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-mimalloc-post-array-len-helper-owner-refresh-v0"
require_line_in_file "$CARD" "source_evidence=296x-708"
require_line_in_file "$CARD" "target_symbol=nyash_array_length_h"
require_line_in_file "$CARD" "body_elapsed_ns=53000000"
require_line_in_file "$CARD" "top_symbol_percent=69.72"
require_line_in_file "$CARD" "remaining_owner=handle_registry_typed_handle_boundary"
require_line_in_file "$CARD" "remaining_owner_confidence=high"
require_line_in_file "$CARD" "implementation_allowed=0"
require_line_in_file "$CARD" "closed_world_route_required=1"
require_line_in_file "$CARD" "object_substrate_required=1"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "source_hako_changed=0"
require_line_in_file "$CARD" "compiler_lowering_changed=0"
require_line_in_file "$CARD" "runtime_object_changed=0"
require_line_in_file "$CARD" "winner_claim=0"
require_line_in_file "$CARD" "next_task=OBJECT-BOUNDARY-INVENTORY-001"
require_line_in_file "$CARD" "summary=ok"

require_line_in_file "$SSOT" "mirbuilder_object_management_enabled=0"
require_line_in_file "$SSOT" "object_storage_plan_is_representation_truth=1"
require_line_in_file "$NEXT_CARD" "Task: OBJECT-BOUNDARY-INVENTORY-001"

echo "[mimalloc-post-array-len-owner] ok"

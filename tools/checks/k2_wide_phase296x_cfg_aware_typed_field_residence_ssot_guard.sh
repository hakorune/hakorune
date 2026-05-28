#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/296x-199-CFG-AWARE-TYPED-FIELD-RESIDENCE-SSOT.md"
PREV="$ROOT/docs/development/current/main/phases/phase-296x/296x-198-CFG-RESIDENCE-OR-RUNTIME-OWNER-SELECTION.md"
SSOT="$ROOT/docs/development/current/main/design/cfg-aware-typed-field-residence-ssot.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row199-cfg-residence-ssot] missing line in ${file#$ROOT/}: $expected" >&2
    exit 1
  fi
}

require_text() {
  local file="$1"
  local expected="$2"
  if ! grep -Fq "$expected" "$file"; then
    echo "[row199-cfg-residence-ssot] missing text in ${file#$ROOT/}: $expected" >&2
    exit 1
  fi
}

require_line "$CARD" "Status: Current"
require_line "$PREV" "Status: Landed"
require_line "$CARD" "owner=cfg_aware_typed_field_residence"
require_line "$CARD" "runtime_helper_abi=fallback"
require_line "$CARD" "block_local_retry=0"
require_line "$CARD" "transform_open=0"
require_line "$CARD" "by_name_special_case=0"
require_line "$CARD" "generic_cse=0"
require_line "$CARD" "winner_claim=0"
require_line "$CARD" "replacement_active=0"
require_line "$CARD" "hook_installed=0"
require_line "$CARD" "global_allocator=0"
require_line "$CARD" "summary=ok"
require_line "$CARD" "cfg_aware_typed_field_residence_ssot=accepted"
require_line "$CARD" "next_inventory_required=1"

require_line "$SSOT" "owner=cfg_aware_typed_field_residence"
require_line "$SSOT" "primary_goal=erase_exported_typed_object_field_helpers"
require_line "$SSOT" "runtime_helper_abi=fallback"
require_line "$SSOT" "transform_open=0"
require_line "$SSOT" "by_name_special_case=0"
require_line "$SSOT" "generic_cse=0"
require_text "$SSOT" "net_helper_call_delta > 0"
require_text "$SSOT" "loop-carried residence requires explicit value and dirty PHIs"
require_text "$SSOT" "flush dirty resident fields before the call"
require_text "$SSOT" "The typed-object helper ABI remains the fallback"

echo "[row199-cfg-residence-ssot] ok"

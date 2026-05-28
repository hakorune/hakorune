#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/296x-204-ARRAY-RUNTIME-SINGLE-THREAD-STORE-BACKEND-SSOT.md"
PREV="$ROOT/docs/development/current/main/phases/phase-296x/296x-203-ARRAY-RUNTIME-SLOT-HELPER-COST-PROBE.md"
SSOT="$ROOT/docs/development/current/main/design/array-runtime-single-thread-store-backend-ssot.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row204-array-store-ssot] missing line in ${file#$ROOT/}: $expected" >&2
    exit 1
  fi
}

require_text() {
  local file="$1"
  local expected="$2"
  if ! grep -Fq "$expected" "$file"; then
    echo "[row204-array-store-ssot] missing text in ${file#$ROOT/}: $expected" >&2
    exit 1
  fi
}

require_line "$CARD" "Status: Current"
require_line "$PREV" "Status: Landed"
require_line "$CARD" "array_runtime_single_thread_store_backend_ssot=accepted"
require_line "$CARD" "default_array_storage_backend=SafeRwLockArrayBox"
require_line "$CARD" "selected_diagnostic_backend=SingleThreadExactArrayStore"
require_line "$CARD" "selection_env=HAKO_ARRAY_SLOT_STORE"
require_line "$CARD" "exported_array_helper_abi=unchanged"
require_line "$CARD" "visible_arraybox_semantics=unchanged"
require_line "$CARD" "optimization_open=0"
require_line "$CARD" "winner_claim=0"
require_line "$CARD" "replacement_active=0"
require_line "$CARD" "hook_installed=0"
require_line "$CARD" "global_allocator=0"
require_line "$CARD" "summary=ok"

require_line "$SSOT" "default_array_storage_backend=SafeRwLockArrayBox"
require_line "$SSOT" "selected_diagnostic_backend=SingleThreadExactArrayStore"
require_line "$SSOT" "selection_env=HAKO_ARRAY_SLOT_STORE"
require_line "$SSOT" "allowed_values=safe_rwlock|single_thread_exact"
require_line "$SSOT" "exported_array_helper_abi=unchanged"
require_line "$SSOT" "visible_arraybox_semantics=unchanged"
require_text "$SSOT" "ArrayBox:"
require_text "$SSOT" "items: Arc<RwLock<ArrayStorage>>"
require_text "$SSOT" "array_runtime_set_idx_i64(handle, idx, value)"
require_text "$SSOT" "unknown value:"
require_text "$SSOT" "fail-fast"
require_text "$SSOT" "hako_alloc_by_name_array_special_case:"

echo "[row204-array-store-ssot] ok"

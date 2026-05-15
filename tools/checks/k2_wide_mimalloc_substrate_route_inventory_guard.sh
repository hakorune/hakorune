#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-substrate-route-inventory"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-293x/293x-390-MIMAP-SUBSTRATE-CONC-002-ROUTE-INVENTORY-GUARD.md"
BOUNDARY="docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md"
METADATA="docs/reference/mir/metadata-facts-ssot.md"
SUBSTRATE="docs/reference/runtime/substrate-capabilities.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
CLOSEOUT="tools/checks/k2_wide_mimalloc_allocator_substrate_closeout_guard.sh"

echo "[$TAG] checking MIMAP substrate route inventory"

fail() {
  echo "[$TAG] ERROR: $*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "missing file: $path"
}

require_text() {
  local file="$1"
  local needle="$2"
  rg -F -q "$needle" "$file" || fail "missing text in $file: $needle"
}

require_file "$CARD"
require_file "$BOUNDARY"
require_file "$METADATA"
require_file "$SUBSTRATE"
require_file "$TASKBOARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$CLOSEOUT"

require_text "$CARD" "Status: landed"
require_text "$CARD" "## Route Inventory"
require_text "$CARD" "backend lowering uses MIR-owned route facts"
require_text "$BOUNDARY" '`MIMAP-SUBSTRATE-CONC-002` | landed'
require_text "$TASKBOARD" '| `MIMAP-SUBSTRATE-CONC-002` | landed |'
require_text "$INDEX" "tools/checks/k2_wide_mimalloc_substrate_route_inventory_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_mimalloc_substrate_route_inventory_guard.sh"

symbols=(
  "hako_mem_alloc"
  "hako_mem_free"
  "hako_osvm_reserve_bytes_i64"
  "hako_osvm_commit_bytes_i64"
  "hako_osvm_decommit_bytes_i64"
  "hako_tls_cache_slot_get_i64"
  "hako_tls_cache_slot_set_i64"
  "hako_atomic_slot_cas_i64"
  "hako_atomic_slot_load_i64"
  "hako_atomic_slot_store_i64"
  "hako_atomic_slot_fetch_add_i64"
  "hako_atomic_ptr_store_ordered"
  "hako_atomic_ptr_load_ordered"
  "hako_atomic_ptr_cas_ordered"
)

for symbol in "${symbols[@]}"; do
  require_text "$CARD" "$symbol"
  require_text "$METADATA" "$symbol"
done

runtime_decl_only=(
  "hako_mem_realloc"
  "hako_osvm_page_size_i64"
)

for symbol in "${runtime_decl_only[@]}"; do
  require_text "$CARD" "$symbol"
  require_text "$METADATA" "$symbol"
done

guards=(
  "tools/checks/k2_wide_hako_mem_extern_pure_first_guard.sh"
  "tools/checks/k2_wide_hako_mem_runtime_decl_guard.sh"
  "tools/checks/k2_wide_mimalloc_osvm_page_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_tls_cache_slot_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_atomic_cas_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_atomic_load_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_atomic_store_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_atomic_fetch_add_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_ptr_atomic_store_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_ptr_atomic_load_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_ptr_atomic_cas_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_allocator_substrate_closeout_guard.sh"
)

for guard in "${guards[@]}"; do
  require_file "$guard"
  require_text "$CARD" "$guard"
  require_text "$INDEX" "$guard"
  require_text "$DEV_GATE" "$guard"
done

require_text "$CARD" "No new extern route row is introduced by this card."
require_text "$CARD" "No source-level"
require_text "$CARD" "No provider hook or host allocator replacement"

echo "[$TAG] ok"

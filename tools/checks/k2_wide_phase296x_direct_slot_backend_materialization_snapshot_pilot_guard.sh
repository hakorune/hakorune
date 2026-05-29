#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-328-DIRECT-SLOT-BACKEND-MATERIALIZATION-SNAPSHOT-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-327-DIRECT-SLOT-BACKEND-MATERIALIZATION-POLICY-SELECTION.md"
STORE="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object_store.rs"
ARENA="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row328-direct-slot-backend-materialization-snapshot-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row328-direct-slot-backend-materialization-snapshot-pilot] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-backend-materialization-snapshot-pilot-v0"
require_line "$DOC" "input_contract=direct-slot-backend-materialization-policy-selection-v0"
require_line "$DOC" "implemented_bridge=direct_slot_object_v0_to_typed_slot_object_snapshot"
require_line "$DOC" "materialization_trigger=explicit_only"
require_line "$DOC" "materialization_view_lifetime=snapshot"
require_line "$DOC" "sync_direction=direct_cell_to_typed_slot_snapshot"
require_line "$DOC" "direct_cell_primary_storage=1"
require_line "$DOC" "dual_truth_allowed=0"
require_line "$DOC" "implicit_sync_on_every_direct_write=0"
require_line "$DOC" "supported_storage_tags=i64,u64,handle"
require_line "$DOC" "unsupported_storage_tag_policy=none_not_silent_fallback"
require_line "$DOC" "direct_slot_snapshot_smoke=ok"
require_line "$DOC" "generic_helper_route_to_direct_backend=0"
require_line "$DOC" "exact_slot_helper_route_to_direct_backend=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "summary=ok"

require_pattern "$ARENA" "pub(crate) fn materialize_typed_object_snapshot"
require_pattern "$ARENA" "fn to_typed_slot"
require_pattern "$STORE" "pub(crate) fn materialize_direct_slot_snapshot"
require_pattern "$STORE" "fn direct_slot_exact_materializes_typed_slot_snapshot_explicitly"

HAKO_TYPED_OBJECT_STORE=direct_slot_exact \
  cargo test -p nyash_kernel direct_slot_exact_materializes_typed_slot_snapshot_explicitly -- --nocapture

echo "[row328-direct-slot-backend-materialization-snapshot-pilot] ok"

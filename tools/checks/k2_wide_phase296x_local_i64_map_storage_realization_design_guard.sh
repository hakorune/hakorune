#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-storage-realization-design"
CARD="docs/development/current/main/phases/phase-296x/296x-905-LOCAL-I64-MAP-STORAGE-REALIZATION-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-904-MAP-HASH-OWNER-REFRESH-AFTER-LOCAL-FASTPATH-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_storage_realization_design_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX"; do
  [[ -f "$file" ]] || { echo "[$TAG] missing file: $file" >&2; exit 1; }
done

grep -q '^Status: Landed$' "$CARD" || {
  echo "[$TAG] card must be Landed" >&2
  exit 1
}

grep -F -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[$TAG] check index missing guard entry" >&2
  exit 1
}

require_card_line() {
  local expected="$1"
  if ! grep -F -x -q "$expected" "$CARD"; then
    echo "[$TAG] missing card line: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-local-i64-map-storage-realization-design-v0" \
  "source_evidence=296x-904" \
  "row_kind=design" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "selected_owner=exact_aot_local_i64_map_storage_realization" \
  "selected_plan_owner=src/mir/map_repr_plan.rs" \
  "selected_backend_owner=src/llvm_py/instructions/mir_call/collection_method_call.py" \
  "selected_runtime_boundary=crates/nyash_kernel/src/plugin/map_slot_load.rs" \
  "before_publication_representation=local_i64_key_map" \
  "publication_materialization_required=1" \
  "after_publication_representation=product_mapbox" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "first_allowed_slice=passive_plan_and_guard_surface" \
  "backend_lowering_enabled=0" \
  "runtime_helper_enabled=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-STORAGE-REALIZATION-GUARD-SURFACE-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for text in \
  "change the representation used before publication, not mutate product" \
  "published object is Box-compatible" \
  "unpublished local object may use a faster internal representation" \
  "does not lower differently yet" \
  "publication_materialization_sites_known=1" \
  "no product \`HashMap\` hasher swap" \
  "no MIRBuilder map storage ownership"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing invariant text: $text" >&2
    exit 1
  }
done

grep -F -q "selected_next=LOCAL-I64-MAP-STORAGE-REALIZATION-DESIGN-001" "$PREV_CARD" || {
  echo "[$TAG] previous owner refresh does not hand off to design" >&2
  exit 1
}

echo "[$TAG] ok"

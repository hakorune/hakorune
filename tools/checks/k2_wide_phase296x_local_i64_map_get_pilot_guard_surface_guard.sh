#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-get-pilot-guard-surface"
CARD="docs/development/current/main/phases/phase-296x/296x-890-LOCAL-I64-MAP-GET-PILOT-GUARD-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-889-LOCAL-I64-MAP-STORAGE-SHADOW-001.md"
BACKEND="src/llvm_py/instructions/mir_call/collection_method_call.py"
LOADER="src/llvm_py/builders/function_metadata.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_get_pilot_guard_surface_guard.sh"

for file in "$CARD" "$PREV_CARD" "$BACKEND" "$LOADER" "$INDEX"; do
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
  "output_contract=hako-local-i64-map-get-pilot-guard-surface-v0" \
  "source_evidence=296x-889" \
  "row_kind=guard_surface" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "selected_consumer=llvm_py_collection_method_call" \
  "selected_metadata=map_repr.local_i64_key_map_shadow" \
  "selected_route_kind=map_load_scalar_i64" \
  "allowed_backend_change=shadow_metadata_get_consumer_only" \
  "allowed_helper_target=local_i64_map_get_pilot_helper" \
  "post_local_i64_map_shadow_get_consumer_enabled=1" \
  "post_product_mapbox_storage_changed=0" \
  "post_product_hasher_swap=0" \
  "post_sidecar_storage=0" \
  "post_mirbuilder_map_storage_ownership=0" \
  "post_benchmark_name_branch=0" \
  "post_helper_name_inference=0" \
  "implementation_allowed=1" \
  "next_task=LOCAL-I64-MAP-GET-PILOT-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCAL-I64-MAP-GET-PILOT-GUARD-SURFACE-001" "$PREV_CARD" || {
  echo "[$TAG] shadow row does not hand off to guard surface" >&2
  exit 1
}

grep -F -q "map_repr_plans_by_site" "$LOADER" || {
  echo "[$TAG] metadata loader must expose map_repr_plans_by_site" >&2
  exit 1
}

grep -F -q "_lower_map_get_collection_method_call" "$BACKEND" || {
  echo "[$TAG] selected backend consumer seam missing" >&2
  exit 1
}

for text in \
  "driven by MIR metadata at the call site" \
  "not by benchmark names" \
  "do not change product \`MapBox\` storage" \
  "do not swap product hasher" \
  "do not add sidecar storage" \
  "do not move map storage ownership into MIRBuilder" \
  "do not infer from \`nyash.map.scalar_load_hi\`" \
  "do not branch on benchmark name, method name alone, or receiver variable name"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing guard text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"

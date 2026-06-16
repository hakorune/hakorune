#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-storage-shadow"
CARD="docs/development/current/main/phases/phase-296x/296x-889-LOCAL-I64-MAP-STORAGE-SHADOW-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-888-LOCAL-I64-MAP-FRONT-SELECTION-001.md"
PLAN="src/mir/map_repr_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_storage_shadow_guard.sh"

for file in "$CARD" "$PREV_CARD" "$PLAN" "$INDEX"; do
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
  "output_contract=hako-local-i64-map-storage-shadow-v0" \
  "source_evidence=296x-888" \
  "row_kind=passive_shadow" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "map_repr_shadow_kind=local_i64_key_map_shadow" \
  "map_repr_shadow_owner=src/mir/map_repr_plan.rs" \
  "map_repr_shadow_source=generic_method_routes" \
  "local_i64_key_map_shadow_requires_i64_set=1" \
  "local_i64_key_map_shadow_requires_scalar_i64_get=1" \
  "local_i64_key_map_shadow_rejects_disallowed_route=1" \
  "fixture_shadow_route_count=4" \
  "backend_lowering_enabled=0" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap_allowed=0" \
  "sidecar_storage_allowed=0" \
  "mirbuilder_map_storage_ownership=0" \
  "implementation_allowed=0" \
  "next_task=LOCAL-I64-MAP-GET-PILOT-GUARD-SURFACE-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCAL-I64-MAP-STORAGE-SHADOW-001" "$PREV_CARD" || {
  echo "[$TAG] front selection does not hand off to shadow row" >&2
  exit 1
}

for code_text in \
  "LocalI64KeyMapShadow" \
  "\"local_i64_key_map_shadow\"" \
  "fn local_i64_key_map_shadow_receivers" \
  "fn is_i64_map_set_route" \
  "fn is_scalar_i64_map_get_route" \
  "refresh_function_map_repr_plans_emits_local_i64_key_map_shadow_rows"; do
  grep -F -q "$code_text" "$PLAN" || {
    echo "[$TAG] missing code evidence: $code_text" >&2
    exit 1
  }
done

for text in \
  "metadata only; it is not a backend lowering instruction" \
  "no backend lowering from shadow metadata" \
  "no product MapBox i64-only storage" \
  "no product hasher swap" \
  "no sidecar storage" \
  "no MIRBuilder map storage ownership" \
  "no benchmark-name / helper-name / variable-name special case"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing shadow decision text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"

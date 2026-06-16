#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-get-pilot"
CARD="docs/development/current/main/phases/phase-296x/296x-891-LOCAL-I64-MAP-GET-PILOT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-890-LOCAL-I64-MAP-GET-PILOT-GUARD-SURFACE-001.md"
BACKEND="src/llvm_py/instructions/mir_call/collection_method_call.py"
TEST="src/llvm_py/tests/test_collection_method_call.py"
KERNEL="crates/nyash_kernel/src/plugin/map_aliases.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_get_pilot_guard.sh"
RETIRE_CARD="docs/development/current/main/phases/phase-296x/296x-911-LOCAL-I64-MAP-LEGACY-SHADOW-CONSUMER-RETIRE-001.md"
RETIRE_GUARD="tools/checks/k2_wide_phase296x_local_i64_map_legacy_shadow_consumer_retire_guard.sh"

for file in "$CARD" "$PREV_CARD" "$BACKEND" "$TEST" "$KERNEL" "$INDEX"; do
  [[ -f "$file" ]] || { echo "[$TAG] missing file: $file" >&2; exit 1; }
done

if [[ -f "$RETIRE_CARD" ]]; then
  bash "$RETIRE_GUARD"
  echo "[$TAG] superseded by 296x-911 legacy shadow consumer retire"
  exit 0
fi

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
  "output_contract=hako-local-i64-map-get-pilot-v0" \
  "source_evidence=296x-890" \
  "row_kind=implementation" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "local_i64_map_shadow_get_consumer_enabled=1" \
  "selected_metadata=map_repr.local_i64_key_map_shadow" \
  "selected_route_kind=map_load_scalar_i64" \
  "selected_backend=src/llvm_py/instructions/mir_call/collection_method_call.py" \
  "selected_helper=nyash.map.local_i64_get_hi" \
  "helper_implementation=delegates_to_map_scalar_load_i64" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "benchmark_name_branch=0" \
  "helper_name_inference=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-GET-PILOT-VALIDATION-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCAL-I64-MAP-GET-PILOT-001" "$PREV_CARD" || {
  echo "[$TAG] guard surface does not hand off to implementation" >&2
  exit 1
}

for backend_text in \
  "def _current_local_i64_map_shadow_get_plan" \
  "map_repr.local_i64_key_map_shadow" \
  "map_load_scalar_i64" \
  "nyash.map.local_i64_get_hi" \
  "local_i64_map_get_hi"; do
  grep -F -q "$backend_text" "$BACKEND" || {
    echo "[$TAG] missing backend evidence: $backend_text" >&2
    exit 1
  }
done

for kernel_text in \
  "nyash.map.local_i64_get_hi" \
  "nyash_map_local_i64_get_hi_alias" \
  "map_scalar_load_i64(handle, key_i64)"; do
  grep -F -q "$kernel_text" "$KERNEL" || {
    echo "[$TAG] missing kernel evidence: $kernel_text" >&2
    exit 1
  }
done

grep -F -q "test_mapbox_local_i64_shadow_get_uses_metadata_pilot_helper" "$TEST" || {
  echo "[$TAG] missing Python test" >&2
  exit 1
}

for text in \
  "This row proves the metadata consumer seam only." \
  "does not change product \`MapBox\` storage" \
  "no product MapBox storage change" \
  "no product hasher swap" \
  "no sidecar storage" \
  "no MIRBuilder map storage ownership" \
  "no benchmark-name / helper-name / variable-name special case" \
  "no Hako-vs-C winner claim"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing implementation text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"

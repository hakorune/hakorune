#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-fastpath-fact-producer-selection"
CARD="docs/development/current/main/phases/phase-296x/296x-902-LOCAL-FASTPATH-FACT-PRODUCER-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-901-LOCAL-FASTPATH-FACT-METADATA-SURFACE-001.md"
PRODUCER="src/mir/map_repr_plan.rs"
METADATA_STRUCT="src/mir/function/metadata.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_fastpath_fact_producer_selection_guard.sh"

for file in "$CARD" "$PREV_CARD" "$PRODUCER" "$METADATA_STRUCT" "$INDEX"; do
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
  "output_contract=hako-local-fastpath-fact-producer-selection-v0" \
  "source_evidence=296x-901" \
  "row_kind=producer_selection_and_minimal_implementation" \
  "selected_producer=map_repr_scalar_i64_no_publication_get" \
  "producer_owner=src/mir/map_repr_plan.rs" \
  "producer_input=MapReprPlan(route_id=map_repr.generic_hash_runtime,source_route_kind=map_load_scalar_i64,publication_policy=no_publication,return_shape=scalar_i64_or_missing_zero)" \
  "producer_output=FunctionMetadata.local_fastpath_facts" \
  "producer_positive_fact_only=1" \
  "producer_fallback_evidence_enabled=0" \
  "producer_observation_export_enabled=0" \
  "alias_class_source=v0_receiver_value_placeholder" \
  "route_plan_id_source=v0_map_repr_plan_index" \
  "storage_plan_id_source=v0_map_repr_plan_index" \
  "full_alias_analysis_enabled=0" \
  "full_object_storage_plan_enabled=0" \
  "hosthandle_bypass_enabled=0" \
  "direct_storage_enabled=0" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "winner_claim=0" \
  "next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCAL-FASTPATH-FACT-PRODUCER-SELECTION-001" "$PREV_CARD" || {
  echo "[$TAG] previous metadata surface does not hand off to producer selection" >&2
  exit 1
}

for text in \
  "build_local_fastpath_facts_from_map_repr_plans" \
  "LocalFastPathFact::known_receiver_direct_call" \
  'plan.route_id() != "map_repr.generic_hash_runtime"' \
  'plan.source_route_kind() != "map_load_scalar_i64"' \
  'plan.publication_policy_tag() != Some("no_publication")' \
  'plan.return_shape_tag() != Some("scalar_i64_or_missing_zero")' \
  "function.metadata.local_fastpath_facts =" \
  "AliasClassId(plan.receiver_value().as_u32())" \
  "RoutePlanId(index as u32)" \
  "ObjectStoragePlanId(index as u32)" \
  "refresh_function_map_repr_plans_emits_local_fastpath_facts_for_scalar_no_publication_get"; do
  grep -F -q "$text" "$PRODUCER" || {
    echo "[$TAG] missing producer evidence: $text" >&2
    exit 1
  }
done

grep -F -q "pub local_fastpath_facts: Vec<LocalFastPathFact>" "$METADATA_STRUCT" || {
  echo "[$TAG] FunctionMetadata missing local_fastpath_facts" >&2
  exit 1
}

for text in \
  "It does not inspect helper symbols or source variable names" \
  "Unknown/missing proof still means no fact." \
  "no fallback Fact producer"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing card invariant: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"

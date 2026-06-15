#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-811-LOCAL-FIRST-OBJECT-MODEL-SSOT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-810-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-FACT-BOUNDARY-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_first_object_model_ssot_guard.sh"

[[ -f "$CARD" ]] || { echo "[local-first-object-model-ssot] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[local-first-object-model-ssot] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[local-first-object-model-ssot] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[local-first-object-model-ssot] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[local-first-object-model-ssot] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[local-first-object-model-ssot] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-local-first-object-model-ssot-v0" \
  "decision=local_first_unpublished_default_for_exact_aot" \
  "source_evidence=296x-810,296x-809,296x-731,object_storage_plan_vocabulary" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "record_source_semantics=value" \
  "box_source_semantics=identity_behavior_lifecycle" \
  "product_default_runtime_changed=0" \
  "product_default_box_world_preserved=1" \
  "exact_aot_local_unpublished_default=1" \
  "published_object_is_box_world=1" \
  "arc_is_post_publication_ownership_form=1" \
  "host_handle_is_boundary_representation=1" \
  "mirbuilder_object_management_enabled=0" \
  "mirbuilder_representation_owner=0" \
  "routeplan_call_execution_truth=1" \
  "objectplan_representation_truth=1" \
  "objectplan_publication_sites_truth=1" \
  "standalone_publication_plan_enabled=0" \
  "publish_site_detection_is_conservative_escape_analysis=1" \
  "unknown_publication_forces_generic_fallback=1" \
  "publish_point_precision_requires_ssa_value_boundary=1" \
  "array_receiver_residence_chain_extended=0" \
  "array_receiver_residence_fact_from_fallback_enabled=0" \
  "direct_residence_fact_implementation_selected=0" \
  "local_direct_pilot_requires_perf_measurement=1" \
  "next_task=OBJECT-PLAN-LOCAL-FIRST-000" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "do not extend ArrayReceiverResidenceInput into fallback Fact" \
  "do not implement DirectResidenceFact before local-first inventory" \
  "do not create standalone PublicationPlan before ObjectPlan proves too large" \
  "do not move object representation ownership into MIRBuilder" \
  "do not change product default runtime behavior" \
  "do not infer representation from helper names" \
  "A correct plan with no measurable win is still a non-keeper."; do
  grep -F -q "$expected" "$CARD" || {
    echo "[local-first-object-model-ssot] missing stop line: $expected" >&2
    exit 1
  }
done

echo "[local-first-object-model-ssot] ok"

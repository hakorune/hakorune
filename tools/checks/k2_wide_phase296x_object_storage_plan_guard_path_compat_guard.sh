#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="object-storage-plan-guard-path-compat"
CARD="docs/development/current/main/phases/phase-296x/296x-990-OBJECT-STORAGE-PLAN-GUARD-PATH-COMPAT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-989-OBJECT-STORAGE-PLAN-MODULE-SPLIT-001.md"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_publication_site_generic_inventory.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_storage_plan_guard_path_compat_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$TOOL"; do
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
  "output_contract=hako-object-storage-plan-guard-path-compat-v0" \
  "source_evidence=296x-989" \
  "row_kind=guard_compat" \
  "facade_and_module_tree_checked=1" \
  "legacy_guard_source_path_compat_fixed=1" \
  "publication_inventory_tool_reads_split_modules=1" \
  "behavior_changed=0" \
  "public_api_reexport_preserved=1" \
  "vocabulary_merge_count=0" \
  "backend_lowering_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=OBJECT-STORAGE-PLAN-VOCAB-AUDIT-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=OBJECT-STORAGE-PLAN-VOCAB-AUDIT-001" "$PREV_CARD" || {
  echo "[$TAG] previous module split card must hand off to vocab audit" >&2
  exit 1
}

grep -F -q "source_dir.glob" "$TOOL" || {
  echo "[$TAG] publication inventory tool must read split module tree" >&2
  exit 1
}

for guard in \
  tools/checks/k2_wide_phase296x_object_storage_plan_ssot_guard.sh \
  tools/checks/k2_wide_phase296x_local_publication_classifier_guard.sh \
  tools/checks/k2_wide_phase296x_local_alias_class_mvp_guard.sh \
  tools/checks/k2_wide_phase296x_local_publication_inventory_v2_guard.sh \
  tools/checks/k2_wide_phase296x_local_known_receiver_direct_call_shadow_guard.sh \
  tools/checks/k2_wide_phase296x_object_plan_local_first_guard.sh \
  tools/checks/k2_wide_phase296x_objectplan_passive_unify_guard.sh \
  tools/checks/k2_wide_phase296x_routeplan_objectplan_handoff_guard.sh \
  tools/checks/k2_wide_phase296x_backend_plan_consumer_guard.sh \
  tools/checks/k2_wide_phase296x_publication_site_generic_inventory_guard.sh \
  tools/checks/k2_wide_phase296x_exact_object_flattened_nested_field_layout_guard.sh \
  tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_candidate_inventory_guard.sh \
  tools/checks/k2_wide_phase296x_compiler_object_shape_closeout_guard.sh \
  tools/checks/k2_wide_phase296x_compiler_object_shape_closeout_followup_guard.sh; do
  bash "$guard" >/tmp/"$TAG"."$(basename "$guard")".out
done

echo "[$TAG] ok"

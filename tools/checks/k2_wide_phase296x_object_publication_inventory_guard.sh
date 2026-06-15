#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-813-OBJECT-PUBLICATION-INVENTORY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-812-OBJECT-PLAN-LOCAL-FIRST-000.md"
TOOL="tools/allocator/hako_object_publication_inventory.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_publication_inventory_guard.sh"

[[ -f "$CARD" ]] || { echo "[object-publication-inventory] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[object-publication-inventory] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[object-publication-inventory] missing tool: $TOOL" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[object-publication-inventory] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[object-publication-inventory] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[object-publication-inventory] check index missing guard entry" >&2
  exit 1
}
grep -q "$TOOL" "$INDEX" || {
  echo "[object-publication-inventory] check index missing tool entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[object-publication-inventory] missing line in $file: $expected" >&2
    exit 1
  fi
}

tmp_dir="$(mktemp -d /tmp/hakorune_object_publication_inventory.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

for expected in \
  "output_contract=hako-object-publication-inventory-v0" \
  "source_evidence=296x-812,296x-811" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "source_file=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako" \
  "inventory_kind=source_body_conservative" \
  "new_box_count=0" \
  "local_binding_count=9" \
  "local_object_candidate_count=1" \
  "preexisting_published_field_alias_count=2" \
  "publication_site_count=2" \
  "publication_reason_host_handle_required_count=2" \
  "publication_reason_plugin_or_extern_count=0" \
  "publication_reason_dynamic_array_or_map_count=0" \
  "publication_reason_task_future_channel_boundary_count=0" \
  "publication_reason_return_as_dynamic_box_count=0" \
  "publication_reason_unknown_count=0" \
  "record_last_alloc_page_call_count=2" \
  "page_local_candidate_count=1" \
  "page_publication_site_count=2" \
  "pre_publication_page_direct_call_count=3" \
  "array_length_direct_candidate_count=0" \
  "array_length_direct_candidate_reason=not_in_facade_body" \
  "unknown_publication_forces_generic_fallback=1" \
  "object_plan_execution_enabled=0" \
  "backend_consumes_object_plan=0" \
  "product_default_changed=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
  require_line_in_file "$report" "$expected"
done

for expected in \
  "do not lower page direct calls from this row" \
  "do not implement local object storage from this row" \
  "do not implement array length direct route from this row" \
  "do not infer array length direct candidacy from helper names"; do
  grep -F -q "$expected" "$CARD" || {
    echo "[object-publication-inventory] missing stop line: $expected" >&2
    exit 1
  }
done

echo "[object-publication-inventory] ok"

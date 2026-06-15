#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-817-LOCAL-PAGE-RECEIVER-CANDIDATE-PROBE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-816-LOCAL-FIRST-DIRECT-PILOT-SELECTION-001.md"
TOOL="tools/allocator/hako_local_page_receiver_candidate_probe.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_page_receiver_candidate_probe_guard.sh"

[[ -f "$CARD" ]] || { echo "[local-page-receiver-candidate-probe] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[local-page-receiver-candidate-probe] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[local-page-receiver-candidate-probe] missing tool: $TOOL" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[local-page-receiver-candidate-probe] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[local-page-receiver-candidate-probe] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[local-page-receiver-candidate-probe] check index missing guard entry" >&2
  exit 1
}
grep -q "$TOOL" "$INDEX" || {
  echo "[local-page-receiver-candidate-probe] check index missing tool entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[local-page-receiver-candidate-probe] missing line in $file: $expected" >&2
    exit 1
  fi
}

tmp_dir="$(mktemp -d /tmp/hakorune_local_page_receiver_candidate_probe.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

for expected in \
  "output_contract=hako-local-page-receiver-candidate-probe-v0" \
  "source_evidence=296x-816,296x-814,296x-813" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "facade_source_file=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako" \
  "queue_source_file=lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako" \
  "page_source_file=lang/src/hako_alloc/memory/page_box.hako" \
  "probe_kind=source_body_conservative" \
  "page_local_binding_count=1" \
  "page_birth_in_body=0" \
  "page_from_queue_selection=1" \
  "page_from_queue_selection_count=2" \
  "page_select_single_fast_path_assignment_count=1" \
  "page_select_page_assignment_count=1" \
  "page_selector_return_type_known_count=2" \
  "page_type_known=1" \
  "page_method_surface_known_count=2" \
  "page_acquire_usize_call_count=2" \
  "page_reuse_call_count=1" \
  "page_pre_publication_call_count=3" \
  "page_publication_site_count=2" \
  "page_call_after_publication_count=0" \
  "page_dynamic_api_required_count=0" \
  "page_plugin_or_extern_escape_count=0" \
  "page_task_boundary_escape_count=0" \
  "page_storage_direct_required=0" \
  "page_hosthandle_bypass_required=0" \
  "closed_world_direct_call_proof_count=0" \
  "routeplan_backend_consumable_proof_count=0" \
  "candidate_probe_open=1" \
  "guard_surface_required=1" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
  require_line_in_file "$report" "$expected"
done

for expected in \
  "do not implement direct call from this probe" \
  "do not treat page as body-local new object" \
  "do not open storage direct route" \
  "do not bypass HostHandle" \
  "do not infer backend-consumable RoutePlan proof from source method presence" \
  "do not special-case page receiver name" \
  "do not special-case acquire_usize or reuse" \
  "do not change product default runtime behavior"; do
  grep -F -q "$expected" "$CARD" || {
    echo "[local-page-receiver-candidate-probe] missing stop line: $expected" >&2
    exit 1
  }
done

echo "[local-page-receiver-candidate-probe] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-814-LOCAL-OBJECT-SHADOW-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-813-OBJECT-PUBLICATION-INVENTORY-001.md"
INV_TOOL="tools/allocator/hako_object_publication_inventory.py"
SHADOW_TOOL="tools/allocator/hako_local_object_shadow.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_object_shadow_guard.sh"

[[ -f "$CARD" ]] || { echo "[local-object-shadow] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[local-object-shadow] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$INV_TOOL" ]] || { echo "[local-object-shadow] missing inventory tool: $INV_TOOL" >&2; exit 1; }
[[ -f "$SHADOW_TOOL" ]] || { echo "[local-object-shadow] missing shadow tool: $SHADOW_TOOL" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[local-object-shadow] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[local-object-shadow] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[local-object-shadow] check index missing guard entry" >&2
  exit 1
}
grep -q "$SHADOW_TOOL" "$INDEX" || {
  echo "[local-object-shadow] check index missing tool entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[local-object-shadow] missing line in $file: $expected" >&2
    exit 1
  fi
}

tmp_dir="$(mktemp -d /tmp/hakorune_local_object_shadow.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
inv_report="$tmp_dir/publication.out"
shadow_report="$tmp_dir/shadow.out"
python3 "$INV_TOOL" --out "$inv_report"
python3 "$SHADOW_TOOL" --publication-report "$inv_report" --out "$shadow_report"

for expected in \
  "output_contract=hako-local-object-shadow-v0" \
  "source_evidence=296x-813,296x-812" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "local_object_candidate_count=1" \
  "local_identity_object_candidate_count=1" \
  "local_scalar_candidate_count=0" \
  "local_struct_candidate_count=0" \
  "published_fallback_candidate_count=1" \
  "publication_site_count=2" \
  "pre_publication_direct_call_count=3" \
  "array_length_direct_candidate_count=0" \
  "local_direct_array_len_pilot_open=0" \
  "shadow_plan_behavior_changed=0" \
  "object_plan_execution_enabled=0" \
  "backend_consumes_object_plan=0" \
  "product_default_changed=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
  require_line_in_file "$shadow_report" "$expected"
done

for expected in \
  "do not implement local page direct lowering from this row" \
  "do not implement Array.length direct lowering from this row" \
  "do not claim LOCAL-DIRECT-ARRAY-LEN-PILOT is open" \
  "do not infer array length candidacy from nyash_array_length_h"; do
  grep -F -q "$expected" "$CARD" || {
    echo "[local-object-shadow] missing stop line: $expected" >&2
    exit 1
  }
done

echo "[local-object-shadow] ok"

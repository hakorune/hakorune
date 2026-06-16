#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-front-selection"
CARD="docs/development/current/main/phases/phase-296x/296x-888-LOCAL-I64-MAP-FRONT-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-887-MAP-HASH-OWNER-INVENTORY-001.md"
SOURCE="benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_front_selection_guard.sh"

for file in "$CARD" "$PREV_CARD" "$SOURCE" "$INDEX"; do
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
  "output_contract=hako-local-i64-map-front-selection-v0" \
  "source_evidence=296x-887" \
  "row_kind=front_selection" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "target_source=benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako" \
  "local_map_birth_count=1" \
  "init_i64_set_count=3" \
  "hot_loop_i64_get_count=1" \
  "post_loop_i64_get_count=1" \
  "dynamic_unknown_key_get_count=0" \
  "text_key_write_count=0" \
  "keys_values_json_use_count=0" \
  "plugin_extern_publication_count=0" \
  "map_return_escape_count=0" \
  "selected_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "selected_storage_plan=LocalI64KeyMap" \
  "selected_scope=shadow_only" \
  "implementation_allowed=0" \
  "next_task=LOCAL-I64-MAP-STORAGE-SHADOW-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "selected_next=LOCAL-I64-MAP-FRONT-SELECTION-001" "$PREV_CARD" || {
  echo "[$TAG] previous card does not hand off to front selection" >&2
  exit 1
}

for source_text in \
  "local map = new MapBox()" \
  "map.set(0, 1)" \
  "map.set(1, 2)" \
  "map.set(2, 3)" \
  "local k = i % 3" \
  "local v = map.get(k)" \
  "return sum + map.get(1)"; do
  grep -F -q "$source_text" "$SOURCE" || {
    echo "[$TAG] missing source evidence: $source_text" >&2
    exit 1
  }
done

for forbidden in \
  "map.keys" \
  "map.values" \
  "toJSON" \
  "plugin" \
  "extern"; do
  if grep -F -q "$forbidden" "$SOURCE"; then
    echo "[$TAG] source contains forbidden publication evidence: $forbidden" >&2
    exit 1
  fi
done

for text in \
  "Use this front as the first \`LocalI64KeyMap\` shadow candidate." \
  "This row does not authorize lowering." \
  "no product MapBox i64-only storage" \
  "no product hasher swap" \
  "no sidecar storage" \
  "no MIRBuilder map storage ownership" \
  "no backend lowering from front-selection evidence alone" \
  "no benchmark-name / helper-name / variable-name special case"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing decision text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"

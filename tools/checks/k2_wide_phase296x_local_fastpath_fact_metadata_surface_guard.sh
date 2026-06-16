#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-fastpath-fact-metadata-surface"
CARD="docs/development/current/main/phases/phase-296x/296x-901-LOCAL-FASTPATH-FACT-METADATA-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-900-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001.md"
METADATA_STRUCT="src/mir/function/metadata.rs"
JSON_EMIT="src/runner/mir_json_emit/plan_metadata.rs"
JSON_TEST="src/runner/mir_json_emit/tests/map_repr_plans.rs"
PY_METADATA="src/llvm_py/builders/function_metadata.py"
PY_TEST="src/llvm_py/tests/test_fastmem_metadata_loader.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_fastpath_fact_metadata_surface_guard.sh"

for file in "$CARD" "$PREV_CARD" "$METADATA_STRUCT" "$JSON_EMIT" "$JSON_TEST" "$PY_METADATA" "$PY_TEST" "$INDEX"; do
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
  "output_contract=hako-local-fastpath-fact-metadata-surface-v0" \
  "source_evidence=296x-900" \
  "row_kind=transport_surface" \
  "function_metadata_local_fastpath_facts=1" \
  "mir_json_emits_local_fastpath_facts=1" \
  "json_site_id_preserved=1" \
  "json_block_instruction_index_preserved=1" \
  "python_loader_indexes_by_block_instruction=1" \
  "backend_reads_positive_fact_only=1" \
  "automatic_fact_producer_enabled=0" \
  "fallback_fact_enabled=0" \
  "fallback_evidence_exported=0" \
  "helper_symbol_inference=0" \
  "source_variable_name_inference=0" \
  "hosthandle_bypass_enabled=0" \
  "direct_storage_enabled=0" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "winner_claim=0" \
  "next_task=LOCAL-FASTPATH-FACT-PRODUCER-SELECTION-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001" "$PREV_CARD" || {
  echo "[$TAG] previous pilot handoff changed unexpectedly" >&2
  exit 1
}

for text in \
  "pub local_fastpath_facts: Vec<LocalFastPathFact>" \
  "positive facts only" \
  "Fallback evidence"; do
  grep -F -q "$text" "$METADATA_STRUCT" || {
    echo "[$TAG] missing FunctionMetadata evidence: $text" >&2
    exit 1
  }
done

for text in \
  '"local_fastpath_facts"' \
  '"site_id": fact.site_id.0' \
  '"block": fact.block_id.0' \
  '"instruction_index": fact.instruction_index.0' \
  '"fallback_reason": serde_json::Value::Null'; do
  grep -F -q "$text" "$JSON_EMIT" || {
    echo "[$TAG] missing JSON emit evidence: $text" >&2
    exit 1
  }
done

for text in \
  "build_mir_json_root_emits_local_fastpath_facts" \
  "LocalFastPathFact::known_receiver_direct_call" \
  "ObjectBasicBlockId" \
  "ObjectInstructionIndex" \
  'fact["fallback_reason"].is_null()'; do
  grep -F -q "$text" "$JSON_TEST" || {
    echo "[$TAG] missing JSON test evidence: $text" >&2
    exit 1
  }
done

for text in \
  "local_fastpath_facts" \
  '"site_id"' \
  "local_fastpath_facts_by_site"; do
  grep -F -q "$text" "$PY_METADATA" "$PY_TEST" || {
    echo "[$TAG] missing Python loader/test evidence: $text" >&2
    exit 1
  }
done

for text in \
  'Do not derive backend call-site lookup' \
  'from `site_id`.' \
  "This row intentionally does not create facts automatically." \
  "no fallback Fact producer"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing card invariant: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-known-receiver-direct-call-pilot"
CARD="docs/development/current/main/phases/phase-296x/296x-900-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-899-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001.md"
BACKEND="src/llvm_py/instructions/mir_call/collection_method_call.py"
METADATA="src/llvm_py/builders/function_metadata.py"
LOWER="src/llvm_py/builders/function_lower.py"
CONTEXT="src/llvm_py/context/function_lower_context.py"
RESOLVER="src/llvm_py/resolver.py"
TEST="src/llvm_py/tests/test_collection_method_call.py"
METADATA_TEST="src/llvm_py/tests/test_fastmem_metadata_loader.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_known_receiver_direct_call_pilot_guard.sh"

for file in "$CARD" "$PREV_CARD" "$BACKEND" "$METADATA" "$LOWER" "$CONTEXT" "$RESOLVER" "$TEST" "$METADATA_TEST" "$INDEX"; do
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
  "output_contract=hako-local-known-receiver-direct-call-pilot-v0" \
  "source_evidence=296x-899" \
  "row_kind=implementation" \
  "backend_reads_local_fastpath_fact=1" \
  "backend_reads_fallback_evidence=0" \
  "backend_reads_helper_symbol=0" \
  "backend_reads_source_variable_name=0" \
  "selected_backend=src/llvm_py/instructions/mir_call/collection_method_call.py" \
  "selected_fact_route=local_fastpath.known_receiver_direct_call" \
  "selected_backend_kind=known_receiver_direct_call" \
  "selected_route_plan=map_repr.generic_hash_runtime" \
  "selected_helper=nyash.map.local_i64_get_hi" \
  "local_fastpath_metadata_loader_enabled=1" \
  "local_fastpath_metadata_field=local_fastpath_facts" \
  "function_lower_loads_local_fastpath_facts=1" \
  "fallback_reason_blocks_fastpath=1" \
  "hosthandle_bypass_enabled=0" \
  "direct_storage_enabled=0" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "winner_claim=0" \
  "next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001" "$PREV_CARD" || {
  echo "[$TAG] shadow row does not hand off to pilot" >&2
  exit 1
}

for backend_text in \
  "LOCAL_FASTPATH_FACT_ROUTE" \
  "local_fastpath.known_receiver_direct_call" \
  "local_fastpath_facts_by_site" \
  "def _current_local_fastpath_known_receiver_direct_call_fact" \
  "fallback_reason" \
  "local_fastpath_map_get_hi"; do
  grep -F -q "$backend_text" "$BACKEND" || {
    echo "[$TAG] missing backend evidence: $backend_text" >&2
    exit 1
  }
done

for metadata_text in \
  "def _load_local_fastpath_fact_metadata" \
  "local_fastpath_facts" \
  "local_fastpath_facts_by_site" \
  "receiver_value" \
  "key_value" \
  "alias_class"; do
  grep -F -q "$metadata_text" "$METADATA" || {
    echo "[$TAG] missing metadata loader evidence: $metadata_text" >&2
    exit 1
  }
done

grep -F -q "_load_local_fastpath_fact_metadata(builder, func_data)" "$LOWER" || {
  echo "[$TAG] function lowering does not load local fastpath facts" >&2
  exit 1
}

for state_file in "$CONTEXT" "$RESOLVER"; do
  grep -F -q "local_fastpath_facts_by_site" "$state_file" || {
    echo "[$TAG] missing resolver state evidence in $state_file" >&2
    exit 1
  }
done

for test_text in \
  "test_mapbox_local_fastpath_fact_get_uses_known_receiver_direct_call_helper" \
  "test_mapbox_local_fastpath_fact_get_ignores_fallback_reason" \
  "_seed_local_fastpath_known_receiver_direct_call_fact"; do
  grep -F -q "$test_text" "$TEST" || {
    echo "[$TAG] missing test evidence: $test_text" >&2
    exit 1
  }
done

grep -F -q "test_local_fastpath_fact_loader_indexes_sites" "$METADATA_TEST" || {
  echo "[$TAG] missing metadata loader test" >&2
  exit 1
}

for text in \
  "\`resolver.local_fastpath_facts_by_site\`" \
  "\`metadata.local_fastpath_facts\`" \
  "fallback_reason=none" \
  "changes the backend proof source, not product MapBox storage" \
  "no HostHandle bypass" \
  "no direct storage enablement"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing card text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"

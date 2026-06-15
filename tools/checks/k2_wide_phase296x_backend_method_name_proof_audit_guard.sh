#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-834-BACKEND-METHOD-NAME-PROOF-AUDIT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-833-COMPILER-OBJECT-SHAPE-CLOSEOUT-FOLLOWUP-001.md"
FINAL_SSOT="docs/development/current/main/design/compiler-object-final-shape-ssot.md"
FLATTENED="src/llvm_py/instructions/flattened_nested_fields.py"
METHOD_CALL="src/llvm_py/instructions/mir_call/method_call.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_backend_method_name_proof_audit_guard.sh"

[[ -f "$CARD" ]] || { echo "[backend-method-name-proof-audit] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[backend-method-name-proof-audit] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$FINAL_SSOT" ]] || { echo "[backend-method-name-proof-audit] missing final SSOT: $FINAL_SSOT" >&2; exit 1; }
[[ -f "$FLATTENED" ]] || { echo "[backend-method-name-proof-audit] missing flattened consumer: $FLATTENED" >&2; exit 1; }
[[ -f "$METHOD_CALL" ]] || { echo "[backend-method-name-proof-audit] missing method call consumer: $METHOD_CALL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || {
  echo "[backend-method-name-proof-audit] card must be Landed" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[backend-method-name-proof-audit] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[backend-method-name-proof-audit] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[backend-method-name-proof-audit] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-backend-method-name-proof-audit-v0" \
  "source_evidence=296x-831,296x-833" \
  "flattened_nested_method_tables_classified=1" \
  "flattened_nested_read_method_map_count=4" \
  "flattened_nested_write_method_map_count=3" \
  "guarded_flattened_nested_method_semantic_map_allowed=1" \
  "generic_backend_method_name_route_inference_count=0" \
  "backend_method_name_special_case_scope=generic_backend_route_inference" \
  "backend_method_name_special_case_selfproof_scope_clarified=1" \
  "flattened_nested_receiver_guard_required=1" \
  "flattened_nested_objectplan_consumer_required=1" \
  "method_call_route_enabled_is_existing_flattened_nested_consumer=1" \
  "backend_lowering_changed=0" \
  "implementation_started=0" \
  "product_default_changed=0" \
  "selected_next=ARRAY-RECEIVER-RESIDENCE-PROOF-CHAIN-RETIRE-INVENTORY-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "backend_method_name_special_case_enabled=0" \
  "backend_method_name_special_case_scope=generic_backend_route_inference" \
  "guarded_flattened_nested_method_semantic_map_allowed=1"; do
  require_line_in_file "$FINAL_SSOT" "$expected"
done

for token in \
  "READ_METHOD_TO_FIELD = {" \
  "\"requested\": \"last_requested\"" \
  "\"normalized\": \"last_normalized\"" \
  "\"reason\": \"last_reason\"" \
  "\"supported\": \"last_supported\"" \
  "WRITE_METHODS = {" \
  "\"recordFailure\"" \
  "\"recordSuccess\"" \
  "\"reset\"" \
  "is_flattened_nested_view(receiver)" \
  "receiver.get(\"nested_object\") != NESTED_OBJECT" \
  "owner_h = receiver.get(\"owner_handle\")"; do
  grep -F -q "$token" "$FLATTENED" || {
    echo "[backend-method-name-proof-audit] missing flattened semantic-map guard token: $token" >&2
    exit 1
  }
done

grep -F -q "if _flattened_nested_fields.is_flattened_nested_view(recv_val):" "$METHOD_CALL" || {
  echo "[backend-method-name-proof-audit] method-call consumer must guard on flattened nested view" >&2
  exit 1
}

for stop_line in \
  "do not remove READ_METHOD_TO_FIELD / WRITE_METHODS as a drive-by cleanup" \
  "do not treat guarded flattened-nested semantic maps as generic route inference" \
  "do not introduce new method-name route selection outside an ObjectPlan consumer" \
  "do not change flattened_nested_fields.py in this audit row"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[backend-method-name-proof-audit] missing stop line: $stop_line" >&2
    exit 1
  }
done

echo "[backend-method-name-proof-audit] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-818-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-GUARD-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-817-LOCAL-PAGE-RECEIVER-CANDIDATE-PROBE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_known_receiver_direct_call_guard_surface_guard.sh"

[[ -f "$CARD" ]] || { echo "[local-known-receiver-direct-call-guard-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[local-known-receiver-direct-call-guard-surface] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[local-known-receiver-direct-call-guard-surface] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[local-known-receiver-direct-call-guard-surface] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[local-known-receiver-direct-call-guard-surface] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[local-known-receiver-direct-call-guard-surface] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-local-known-receiver-direct-call-guard-surface-v0" \
  "source_evidence=296x-817,296x-816" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "selected_shape=local_known_receiver_direct_call" \
  "first_target_receiver=page" \
  "first_target_methods=acquire_usize,reuse" \
  "first_target_call_count=3" \
  "guard_receiver_pre_publication_required=1" \
  "guard_receiver_type_known_required=1" \
  "guard_method_surface_known_required=1" \
  "guard_dynamic_api_absent_required=1" \
  "guard_plugin_or_extern_absent_required=1" \
  "guard_task_boundary_absent_required=1" \
  "guard_page_call_after_publication_required_zero=1" \
  "storage_direct_required=0" \
  "storage_direct_enabled=0" \
  "hosthandle_bypass_enabled=0" \
  "arc_retirement_enabled=0" \
  "page_specific_rule_enabled=0" \
  "method_name_special_case_enabled=0" \
  "helper_symbol_inference_enabled=0" \
  "routeplan_backend_consumable_proof_required_before_implementation=1" \
  "shadow_allowed=1" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "do not implement direct call from this row" \
  "do not bypass HostHandle" \
  "do not open storage direct route" \
  "do not retire Arc" \
  "do not special-case page receiver name" \
  "do not special-case acquire_usize or reuse" \
  "do not infer from helper symbol" \
  "do not change product default runtime behavior"; do
  grep -F -q "$expected" "$CARD" || {
    echo "[local-known-receiver-direct-call-guard-surface] missing stop line: $expected" >&2
    exit 1
  }
done

echo "[local-known-receiver-direct-call-guard-surface] ok"

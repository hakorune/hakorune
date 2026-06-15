#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-761-CALL-OPERAND-ROUTE-CARRIER-RECEIVER-ARG-SPLIT-PROBE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-760-CALL-OPERAND-ROUTE-CARRIER-REVALIDATION-GUARD-SURFACE-001.md"
PROBE="tools/allocator/mir_local_ssa_copy_position_probe.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_call_operand_route_carrier_receiver_arg_split_probe_guard.sh"

[[ -f "$CARD" ]] || { echo "[call-operand-route-carrier-receiver-arg-split-probe] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[call-operand-route-carrier-receiver-arg-split-probe] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$PROBE" ]] || { echo "[call-operand-route-carrier-receiver-arg-split-probe] missing probe: $PROBE" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[call-operand-route-carrier-receiver-arg-split-probe] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[call-operand-route-carrier-receiver-arg-split-probe] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[call-operand-route-carrier-receiver-arg-split-probe] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[call-operand-route-carrier-receiver-arg-split-probe] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-call-operand-route-carrier-receiver-arg-split-probe-v0" \
  "source_evidence=296x-760" \
  "probe_tool=tools/allocator/mir_local_ssa_copy_position_probe.py" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "copy_count=51" \
  "backend_route_carrier_copy_count=19" \
  "route_carrier_residual_copy_count=13" \
  "call_operand_route_carrier_copy_count=13" \
  "call_operand_receiver_route_carrier_copy_count=2" \
  "call_operand_arg_route_carrier_copy_count=11" \
  "call_operand_receiver_route_carrier_sample_count=2" \
  "call_operand_arg_route_carrier_sample_count=11" \
  "dominant_call_operand_surface=arg" \
  "receiver_post_target=0" \
  "receiver_post_target_met=0" \
  "arg_forwarding_enabled=0" \
  "arg_forwarding_policy=closed_until_explicit_arg_owner_selection" \
  "selected_next_action=call_operand_receiver_residue_classification" \
  "implementation_allowed=0" \
  "measurement_required=0" \
  "winner_claim=0" \
  "startup_lane_reopened=0" \
  "source_hako_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "product_default_changed=0" \
  "provider_active=0" \
  "replacement_active=0" \
  "hook_installed=0" \
  "global_allocator=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "call_operand_receiver" \
  "call_operand_arg" \
  "call_operand_receiver_route_carrier_sample_count" \
  "call_operand_arg_route_carrier_sample_count"; do
  grep -F -q "$expected" "$PROBE" || {
    echo "[call-operand-route-carrier-receiver-arg-split-probe] probe missing token: $expected" >&2
    exit 1
  }
done

grep -F -q "CALL-OPERAND-RECEIVER-RESIDUE-CLASSIFICATION-001:" "$CARD" || {
  echo "[call-operand-route-carrier-receiver-arg-split-probe] next receiver classification row is not documented" >&2
  exit 1
}

echo "[call-operand-route-carrier-receiver-arg-split-probe] ok"

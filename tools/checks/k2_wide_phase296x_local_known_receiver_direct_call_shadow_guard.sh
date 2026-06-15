#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-819-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-818-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-GUARD-SURFACE-001.md"
PROBE_TOOL="tools/allocator/hako_local_page_receiver_candidate_probe.py"
SHADOW_TOOL="tools/allocator/hako_local_known_receiver_direct_call_shadow.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_known_receiver_direct_call_shadow_guard.sh"

[[ -f "$CARD" ]] || { echo "[local-known-receiver-direct-call-shadow] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[local-known-receiver-direct-call-shadow] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$PROBE_TOOL" ]] || { echo "[local-known-receiver-direct-call-shadow] missing probe tool: $PROBE_TOOL" >&2; exit 1; }
[[ -f "$SHADOW_TOOL" ]] || { echo "[local-known-receiver-direct-call-shadow] missing shadow tool: $SHADOW_TOOL" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[local-known-receiver-direct-call-shadow] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[local-known-receiver-direct-call-shadow] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[local-known-receiver-direct-call-shadow] check index missing guard entry" >&2
  exit 1
}
grep -q "$SHADOW_TOOL" "$INDEX" || {
  echo "[local-known-receiver-direct-call-shadow] check index missing tool entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[local-known-receiver-direct-call-shadow] missing line in $file: $expected" >&2
    exit 1
  fi
}

tmp_dir="$(mktemp -d /tmp/hakorune_local_known_receiver_direct_call_shadow.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
probe_report="$tmp_dir/probe.out"
shadow_report="$tmp_dir/shadow.out"
python3 "$PROBE_TOOL" --out "$probe_report"
python3 "$SHADOW_TOOL" --probe-report "$probe_report" --out "$shadow_report"

for expected in \
  "output_contract=hako-local-known-receiver-direct-call-shadow-v0" \
  "source_evidence=296x-818,296x-817" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "shadow_kind=report_only" \
  "selected_shape=local_known_receiver_direct_call" \
  "first_target_receiver=page" \
  "shadow_guard_satisfied=1" \
  "shadow_direct_call_candidate_count=3" \
  "shadow_page_acquire_usize_count=2" \
  "shadow_page_reuse_count=1" \
  "shadow_route_kind=pre_publication_known_receiver_method_call" \
  "shadow_rule_source=objectplan_pre_publication_plus_known_receiver_surface" \
  "receiver_name_rule_enabled=0" \
  "method_name_rule_enabled=0" \
  "helper_symbol_inference_enabled=0" \
  "storage_direct_count=0" \
  "hosthandle_bypass_count=0" \
  "arc_retirement_count=0" \
  "routeplan_backend_consumable_proof_required_before_lowering=1" \
  "shadow_plan_behavior_changed=0" \
  "product_default_changed=0" \
  "pilot_implementation_candidate=1" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
  require_line_in_file "$shadow_report" "$expected"
done

for expected in \
  "do not implement direct call from shadow alone" \
  "do not special-case page receiver name" \
  "do not special-case acquire_usize or reuse" \
  "do not infer from helper symbol" \
  "do not bypass HostHandle" \
  "do not open storage direct route" \
  "do not retire Arc" \
  "do not change product default runtime behavior" \
  "stop for design consultation if no such seam exists"; do
  grep -F -q "$expected" "$CARD" || {
    echo "[local-known-receiver-direct-call-shadow] missing stop line: $expected" >&2
    exit 1
  }
done

echo "[local-known-receiver-direct-call-shadow] ok"

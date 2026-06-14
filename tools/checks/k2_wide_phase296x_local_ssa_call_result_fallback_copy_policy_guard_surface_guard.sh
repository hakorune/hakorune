#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-680-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-GUARD-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-679-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_ssa_call_result_fallback_copy_policy_guard_surface_guard.sh"

[[ -f "$CARD" ]] || { echo "[local-ssa-call-result-policy-guard-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[local-ssa-call-result-policy-guard-surface] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[local-ssa-call-result-policy-guard-surface] row680 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[local-ssa-call-result-policy-guard-surface] row679 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[local-ssa-call-result-policy-guard-surface] check index missing guard entry" >&2; exit 1; }

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$CARD"; then
    echo "[local-ssa-call-result-policy-guard-surface] missing card line: $expected" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-local-ssa-call-result-fallback-copy-policy-guard-surface-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "source_evidence=296x-679"
require_line "pre_candidate_result_copy_count=14"
require_line "pre_terminal_compare_operand_count=4"
require_line "post_terminal_compare_operand_target=0"
require_line "post_candidate_result_copy_count_upper_bound=4"
require_line "allowed_use_kind=CompareOperand"
require_line "arg_forwarding_enabled=0"
require_line "helper_name_special_case=0"
require_line "variable_map_semantics_changed=0"
require_line "phi_lifecycle_changed=0"
require_line "implementation_started=0"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "summary=ok"

grep -q 'do not require full 14-copy removal for first keeper' "$CARD" || {
  echo "[local-ssa-call-result-policy-guard-surface] card must avoid full-chain overclaim" >&2
  exit 1
}
grep -q 'do not broaden LocalSSA copy coalescing' "$CARD" || {
  echo "[local-ssa-call-result-policy-guard-surface] card must forbid broad coalescing" >&2
  exit 1
}
grep -q 'do not enable Arg forwarding' "$CARD" || {
  echo "[local-ssa-call-result-policy-guard-surface] card must keep Arg forwarding closed" >&2
  exit 1
}

echo "[local-ssa-call-result-policy-guard-surface] ok"

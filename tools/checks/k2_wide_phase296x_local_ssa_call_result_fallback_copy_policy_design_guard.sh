#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-679-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-678-PAGE-HOTPATH-HELPER-RESULT-EMISSION-OWNER-REFRESH-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_ssa_call_result_fallback_copy_policy_design_guard.sh"
TOOL="tools/allocator/hako_mimalloc_local_ssa_call_result_fallback_copy_policy_design.py"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"

[[ -f "$CARD" ]] || { echo "[local-ssa-call-result-policy-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[local-ssa-call-result-policy-design] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[local-ssa-call-result-policy-design] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[local-ssa-call-result-policy-design] row679 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[local-ssa-call-result-policy-design] row678 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[local-ssa-call-result-policy-design] check index missing guard entry" >&2; exit 1; }
grep -q 'do not broaden LocalSSA copy coalescing' "$CARD" || { echo "[local-ssa-call-result-policy-design] card must forbid broad coalescing" >&2; exit 1; }
grep -q 'do not forward arbitrary Call results' "$CARD" || { echo "[local-ssa-call-result-policy-design] card must forbid arbitrary Call forwarding" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_local_ssa_call_result_policy.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
report="$tmp_dir/report.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$TOOL" \
  --mir-json "$mir_json" \
  --owner-refresh "$PREV_CARD" \
  --out "$report"

require_report_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$report"; then
    echo "[local-ssa-call-result-policy-design] missing report line: $expected" >&2
    cat "$report" >&2
    exit 1
  fi
}

require_card_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$CARD"; then
    echo "[local-ssa-call-result-policy-design] missing card line: $expected" >&2
    exit 1
  fi
}

require_report_line "output_contract=hako-mimalloc-local-ssa-call-result-fallback-copy-policy-design-v0"
require_report_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_report_line "source_evidence=296x-678"
require_report_line "candidate_result_copy_count=14"
require_report_line "first_hop_call_result_copy_count=4"
require_report_line "chain_internal_copy_count=10"
require_report_line "terminal_compare_operand_count=4"
require_report_line "terminal_compare_covered_by_same_block_call_root_count=4"
require_report_line "uncovered_terminal_compare_operand_count=0"
require_report_line "post_candidate_result_copy_count_upper_bound=4"
require_report_line "selected_policy_shape=same_block_call_result_root_for_compare_operand"
require_report_line "selected_policy_owner=LocalSSA::ensure_fallback_copy"
require_report_line "selected_owner_confidence=medium"
require_report_line "next_task=local_ssa_call_result_fallback_copy_policy_guard_surface"
require_report_line "allowed_use_kind=CompareOperand"
require_report_line "arg_forwarding_enabled=0"
require_report_line "helper_name_special_case=0"
require_report_line "variable_map_semantics_changed=0"
require_report_line "phi_lifecycle_changed=0"
require_report_line "implementation_started=0"
require_report_line "optimization_open=0"
require_report_line "winner_claim=0"
require_report_line "summary=ok"

require_card_line "output_contract=hako-mimalloc-local-ssa-call-result-fallback-copy-policy-design-v0"
require_card_line "selected_policy_shape=same_block_call_result_root_for_compare_operand"
require_card_line "selected_policy_owner=LocalSSA::ensure_fallback_copy"
require_card_line "post_candidate_result_copy_count_upper_bound=4"
require_card_line "helper_name_special_case=0"
require_card_line "variable_map_semantics_changed=0"
require_card_line "phi_lifecycle_changed=0"

echo "[local-ssa-call-result-policy-design] ok"

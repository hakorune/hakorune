#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-681-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-IMPLEMENTATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-680-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-GUARD-SURFACE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_ssa_call_result_fallback_copy_policy_implementation_guard.sh"
TOOL="tools/allocator/hako_mimalloc_local_ssa_call_result_fallback_copy_policy_post_probe.py"
LOCAL_SSA="src/mir/builder/ssa/local.rs"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"

[[ -f "$CARD" ]] || { echo "[local-ssa-call-result-policy-impl] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[local-ssa-call-result-policy-impl] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[local-ssa-call-result-policy-impl] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[local-ssa-call-result-policy-impl] row681 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[local-ssa-call-result-policy-impl] row680 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[local-ssa-call-result-policy-impl] check index missing guard entry" >&2; exit 1; }
grep -q 'fn can_forward_same_block_call_result_to_compare_operand' "$LOCAL_SSA" || {
  echo "[local-ssa-call-result-policy-impl] missing LocalKind policy method" >&2
  exit 1
}
grep -q 'fn same_block_call_result_root' "$LOCAL_SSA" || {
  echo "[local-ssa-call-result-policy-impl] missing same-block Call root helper" >&2
  exit 1
}
grep -q 'matches!(self, LocalKind::CompareOperand)' "$LOCAL_SSA" || {
  echo "[local-ssa-call-result-policy-impl] policy must stay CompareOperand-only" >&2
  exit 1
}
if grep -q 'PAGE_HOTPATH_HELPERS\\|acquire_usize\\|selectSinglePageFastPath\\|reuse' "$LOCAL_SSA"; then
  echo "[local-ssa-call-result-policy-impl] LocalSSA must not special-case helper names" >&2
  exit 1
fi

tmp_dir="$(mktemp -d /tmp/hakorune_local_ssa_call_result_impl.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
report="$tmp_dir/report.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$TOOL" \
  --mir-json "$mir_json" \
  --guard-surface "$PREV_CARD" \
  --out "$report"

require_report_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$report"; then
    echo "[local-ssa-call-result-policy-impl] missing report line: $expected" >&2
    cat "$report" >&2
    exit 1
  fi
}

require_card_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$CARD"; then
    echo "[local-ssa-call-result-policy-impl] missing card line: $expected" >&2
    exit 1
  fi
}

require_report_line "output_contract=hako-mimalloc-local-ssa-call-result-fallback-copy-policy-implementation-v0"
require_report_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_report_line "source_evidence=296x-680"
require_report_line "pre_candidate_result_copy_count=14"
require_report_line "pre_terminal_compare_operand_count=4"
require_report_line "post_terminal_compare_operand_count=0"
require_report_line "post_candidate_result_copy_count=0"
require_report_line "post_candidate_result_copy_count_upper_bound=4"
require_report_line "allowed_use_kind=CompareOperand"
require_report_line "arg_forwarding_enabled=0"
require_report_line "helper_name_special_case=0"
require_report_line "variable_map_semantics_changed=0"
require_report_line "phi_lifecycle_changed=0"
require_report_line "implementation_started=1"
require_report_line "optimization_open=0"
require_report_line "winner_claim=0"
require_report_line "summary=ok"

require_card_line "output_contract=hako-mimalloc-local-ssa-call-result-fallback-copy-policy-implementation-v0"
require_card_line "post_terminal_compare_operand_count=0"
require_card_line "post_candidate_result_copy_count=0"
require_card_line "arg_forwarding_enabled=0"
require_card_line "helper_name_special_case=0"
require_card_line "winner_claim=0"

echo "[local-ssa-call-result-policy-impl] ok"

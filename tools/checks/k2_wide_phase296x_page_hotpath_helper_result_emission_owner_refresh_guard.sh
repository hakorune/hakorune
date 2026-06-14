#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-678-PAGE-HOTPATH-HELPER-RESULT-EMISSION-OWNER-REFRESH-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-677-PAGE-HOTPATH-HELPER-RESULT-COPY-CHAIN-NARROWING-IMPLEMENTATION-001.md"
DESIGN_CARD="docs/development/current/main/phases/phase-296x/296x-675-PAGE-HOTPATH-HELPER-RESULT-COPY-CHAIN-NARROWING-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_page_hotpath_helper_result_emission_owner_refresh_guard.sh"
TOOL="tools/allocator/hako_mimalloc_page_hotpath_helper_result_emission_owner_refresh.py"
LOCAL_SSA="src/mir/builder/ssa/local.rs"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"

[[ -f "$CARD" ]] || { echo "[page-hotpath-result-emission-owner] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[page-hotpath-result-emission-owner] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$DESIGN_CARD" ]] || { echo "[page-hotpath-result-emission-owner] missing design card: $DESIGN_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[page-hotpath-result-emission-owner] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[page-hotpath-result-emission-owner] row678 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[page-hotpath-result-emission-owner] row677 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[page-hotpath-result-emission-owner] check index missing guard entry" >&2; exit 1; }
grep -q 'LocalKind::CompareOperand' "$LOCAL_SSA" || { echo "[page-hotpath-result-emission-owner] LocalSSA compare operand seam missing" >&2; exit 1; }
grep -q 'builder.emit_instruction(MirInstruction::Copy { dst: loc, src: v })' "$LOCAL_SSA" || {
  echo "[page-hotpath-result-emission-owner] LocalSSA fallback Copy seam missing" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_page_hotpath_result_emission_owner.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
report="$tmp_dir/report.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$TOOL" \
  --mir-json "$mir_json" \
  --design "$DESIGN_CARD" \
  --out "$report"

require_report_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$report"; then
    echo "[page-hotpath-result-emission-owner] missing report line: $expected" >&2
    cat "$report" >&2
    exit 1
  fi
}

require_card_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$CARD"; then
    echo "[page-hotpath-result-emission-owner] missing card line: $expected" >&2
    exit 1
  fi
}

require_report_line "output_contract=hako-mimalloc-page-hotpath-helper-result-emission-owner-refresh-v0"
require_report_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_report_line "source_evidence=296x-677"
require_report_line "candidate_result_copy_count=14"
require_report_line "terminal_consumer_rewrite_candidate_count=4"
require_report_line "local_ssa_terminal_rewrite_owner_rejected=1"
require_report_line "first_hop_call_result_copy_count=4"
require_report_line "chain_internal_copy_count=10"
require_report_line "terminal_compare_operand_count=4"
require_report_line "terminal_compare_first_hop_root_count=4"
require_report_line "dominant_emission_owner=LocalSSA::ensure_fallback_copy"
require_report_line "selected_next_owner=local_ssa_call_result_fallback_copy_policy"
require_report_line "selected_owner_confidence=medium"
require_report_line "next_task=local_ssa_call_result_fallback_copy_policy_design"
require_report_line "implementation_started=0"
require_report_line "optimization_open=0"
require_report_line "winner_claim=0"
require_report_line "summary=ok"

require_card_line "output_contract=hako-mimalloc-page-hotpath-helper-result-emission-owner-refresh-v0"
require_card_line "dominant_emission_owner=LocalSSA::ensure_fallback_copy"
require_card_line "selected_next_owner=local_ssa_call_result_fallback_copy_policy"
require_card_line "next_task=local_ssa_call_result_fallback_copy_policy_design"
require_card_line "implementation_started=0"
require_card_line "winner_claim=0"

echo "[page-hotpath-result-emission-owner] ok"

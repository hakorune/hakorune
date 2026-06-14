#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-675-PAGE-HOTPATH-HELPER-RESULT-COPY-CHAIN-NARROWING-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-674-PAGE-HOTPATH-HELPER-RESULT-MATERIALIZATION-INVENTORY-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_page_hotpath_helper_result_copy_chain_design_guard.sh"
DESIGN="tools/allocator/hako_mimalloc_page_hotpath_helper_result_copy_chain_design.py"

[[ -f "$CARD" ]] || { echo "[page-hotpath-result-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[page-hotpath-result-design] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$DESIGN" ]] || { echo "[page-hotpath-result-design] missing design tool: $DESIGN" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[page-hotpath-result-design] row675 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[page-hotpath-result-design] row674 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[page-hotpath-result-design] check index missing guard entry" >&2; exit 1; }
grep -q 'output_contract=hako-mimalloc-page-hotpath-helper-result-copy-chain-narrowing-design-v0' "$CARD" || { echo "[page-hotpath-result-design] card missing output contract evidence" >&2; exit 1; }
grep -q 'candidate_result_copy_count=14' "$CARD" || { echo "[page-hotpath-result-design] card missing candidate count" >&2; exit 1; }
grep -q 'safe_candidate_count=14' "$CARD" || { echo "[page-hotpath-result-design] card missing safe count" >&2; exit 1; }
grep -q 'unsafe_candidate_count=0' "$CARD" || { echo "[page-hotpath-result-design] card missing unsafe count" >&2; exit 1; }
grep -q 'selected_keeper_shape=same_block_call_result_terminal_consumer_rewrite' "$CARD" || { echo "[page-hotpath-result-design] card missing keeper shape" >&2; exit 1; }
grep -q 'selected_keeper_owner=LocalSSA::ensure_call_result_alias_to_consumer' "$CARD" || { echo "[page-hotpath-result-design] card missing keeper owner" >&2; exit 1; }
grep -q 'do not broaden LocalSSA copy coalescing' "$CARD" || { echo "[page-hotpath-result-design] card must forbid broad coalescing" >&2; exit 1; }
grep -q 'do not forward arbitrary call results' "$CARD" || { echo "[page-hotpath-result-design] card must forbid arbitrary call-result forwarding" >&2; exit 1; }
grep -q 'implementation_started=0' "$CARD" || { echo "[page-hotpath-result-design] card must be design-only" >&2; exit 1; }
grep -q 'winner_claim=0' "$CARD" || { echo "[page-hotpath-result-design] card must keep winner claim closed" >&2; exit 1; }

echo "[page-hotpath-result-design] ok"

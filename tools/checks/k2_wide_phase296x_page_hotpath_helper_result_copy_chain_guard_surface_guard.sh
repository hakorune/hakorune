#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-676-PAGE-HOTPATH-HELPER-RESULT-COPY-CHAIN-NARROWING-GUARD-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-675-PAGE-HOTPATH-HELPER-RESULT-COPY-CHAIN-NARROWING-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_page_hotpath_helper_result_copy_chain_guard_surface_guard.sh"

[[ -f "$CARD" ]] || { echo "[page-hotpath-result-guard-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[page-hotpath-result-guard-surface] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[page-hotpath-result-guard-surface] row676 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[page-hotpath-result-guard-surface] row675 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[page-hotpath-result-guard-surface] check index missing guard entry" >&2; exit 1; }
grep -q 'output_contract=hako-mimalloc-page-hotpath-helper-result-copy-chain-guard-surface-v0' "$CARD" || { echo "[page-hotpath-result-guard-surface] card missing output contract evidence" >&2; exit 1; }
grep -q 'pre_candidate_result_copy_count=14' "$CARD" || { echo "[page-hotpath-result-guard-surface] card missing pre candidate count" >&2; exit 1; }
grep -q 'pre_terminal_consumer_rewrite_candidate_count=4' "$CARD" || { echo "[page-hotpath-result-guard-surface] card missing terminal target" >&2; exit 1; }
grep -q 'post_terminal_consumer_target=0' "$CARD" || { echo "[page-hotpath-result-guard-surface] card missing post terminal target" >&2; exit 1; }
grep -q 'post_candidate_result_copy_count_upper_bound=10' "$CARD" || { echo "[page-hotpath-result-guard-surface] card missing post copy upper bound" >&2; exit 1; }
grep -q 'do not require full 14-copy removal for first keeper' "$CARD" || { echo "[page-hotpath-result-guard-surface] card must avoid overclaiming full chain removal" >&2; exit 1; }
grep -q 'do not broaden LocalSSA copy coalescing' "$CARD" || { echo "[page-hotpath-result-guard-surface] card must forbid broad LocalSSA coalescing" >&2; exit 1; }
grep -q 'implementation_started=0' "$CARD" || { echo "[page-hotpath-result-guard-surface] card must be guard-only" >&2; exit 1; }
grep -q 'winner_claim=0' "$CARD" || { echo "[page-hotpath-result-guard-surface] card must keep winner claim closed" >&2; exit 1; }

echo "[page-hotpath-result-guard-surface] ok"

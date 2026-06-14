#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-671-CROSS-BLOCK-FIELD-GET-ALIAS-FORWARDING-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-670-FIELD-GET-DIRECT-CONSUMER-FORWARDING-REFRESH-002.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_cross_block_field_get_alias_design_guard.sh"
DESIGN="tools/allocator/hako_mimalloc_cross_block_field_get_alias_design_probe.py"

[[ -f "$CARD" ]] || { echo "[cross-block-field-get-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[cross-block-field-get-design] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$DESIGN" ]] || { echo "[cross-block-field-get-design] missing design probe: $DESIGN" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[cross-block-field-get-design] row671 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[cross-block-field-get-design] row670 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[cross-block-field-get-design] check index missing guard entry" >&2; exit 1; }
grep -q 'output_contract=hako-mimalloc-cross-block-field-get-alias-forwarding-design-v0' "$CARD" || { echo "[cross-block-field-get-design] card missing output contract evidence" >&2; exit 1; }
grep -q 'keeper_shape=dominance_alias' "$CARD" || { echo "[cross-block-field-get-design] card missing keeper shape" >&2; exit 1; }
grep -q 'safe_alias_candidate_count=4' "$CARD" || { echo "[cross-block-field-get-design] card missing safe alias count" >&2; exit 1; }
grep -q 'arbitrary_copy_coalescing_allowed=0' "$CARD" || { echo "[cross-block-field-get-design] card must forbid arbitrary coalescing" >&2; exit 1; }
grep -q 'implementation_started=0' "$CARD" || { echo "[cross-block-field-get-design] card must be design-only" >&2; exit 1; }
grep -q 'winner_claim=0' "$CARD" || { echo "[cross-block-field-get-design] card must keep winner claim closed" >&2; exit 1; }

echo "[cross-block-field-get-design] ok"

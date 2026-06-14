#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-670-FIELD-GET-DIRECT-CONSUMER-FORWARDING-REFRESH-002.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-669-PARAM-ALIAS-COPY-OWNER-REFRESH-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_field_get_direct_consumer_refresh_guard.sh"
REFRESH="tools/allocator/hako_mimalloc_field_get_direct_consumer_refresh_probe.py"

[[ -f "$CARD" ]] || { echo "[field-get-refresh] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[field-get-refresh] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$REFRESH" ]] || { echo "[field-get-refresh] missing refresh probe: $REFRESH" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[field-get-refresh] row670 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[field-get-refresh] row669 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[field-get-refresh] check index missing guard entry" >&2; exit 1; }
grep -q 'output_contract=hako-mimalloc-field-get-direct-consumer-refresh-v2' "$CARD" || { echo "[field-get-refresh] card missing output contract evidence" >&2; exit 1; }
grep -q 'selected_owner=cross_block_field_get_alias_copy_chain' "$CARD" || { echo "[field-get-refresh] card missing selected owner" >&2; exit 1; }
grep -q 'covered_by_existing_rule_count=0' "$CARD" || { echo "[field-get-refresh] card missing existing-rule coverage evidence" >&2; exit 1; }
grep -q 'optimization_open=0' "$CARD" || { echo "[field-get-refresh] card must keep optimization closed" >&2; exit 1; }
grep -q 'winner_claim=0' "$CARD" || { echo "[field-get-refresh] card must keep winner claim closed" >&2; exit 1; }

echo "[field-get-refresh] ok"

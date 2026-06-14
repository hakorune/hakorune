#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-672-CROSS-BLOCK-FIELD-GET-ALIAS-FORWARDING-KEEPER-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-671-CROSS-BLOCK-FIELD-GET-ALIAS-FORWARDING-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_cross_block_field_get_alias_keeper_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
POST="tools/allocator/hako_mimalloc_field_get_alias_keeper_post_probe.py"

[[ -f "$CARD" ]] || { echo "[cross-block-field-get-keeper] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[cross-block-field-get-keeper] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$POST" ]] || { echo "[cross-block-field-get-keeper] missing post probe: $POST" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[cross-block-field-get-keeper] row672 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[cross-block-field-get-keeper] row671 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[cross-block-field-get-keeper] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_cross_block_field_get_keeper.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
post_report="$tmp_dir/post.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null
python3 "$POST" --mir-json "$mir_json" --out "$post_report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$post_report"; then
    echo "[cross-block-field-get-keeper] missing report line: $expected" >&2
    cat "$post_report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-field-get-alias-keeper-post-probe-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "copy_count=69"
require_line "expression_materialization_copy_count=3"
require_line "field_get_expression_copy_count=0"
require_line "forwarding_candidate_copy_count=0"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "summary=ok"

echo "[cross-block-field-get-keeper] ok"

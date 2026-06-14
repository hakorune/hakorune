#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-687-CALL-OPERAND-MATERIALIZATION-FORWARDING-IMPLEMENTATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-686-CALL-OPERAND-MATERIALIZATION-FORWARDING-GUARD-SURFACE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_call_operand_materialization_forwarding_implementation_guard.sh"
INVENTORY="tools/allocator/hako_mimalloc_call_operand_materialization_copy_chain_inventory.py"
POST="tools/allocator/hako_mimalloc_call_operand_materialization_forwarding_post_probe.py"
LOCAL_SSA="src/mir/builder/ssa/local.rs"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"

[[ -f "$CARD" ]] || { echo "[call-operand-forwarding-impl] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[call-operand-forwarding-impl] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$INVENTORY" ]] || { echo "[call-operand-forwarding-impl] missing inventory tool: $INVENTORY" >&2; exit 1; }
[[ -f "$POST" ]] || { echo "[call-operand-forwarding-impl] missing post probe: $POST" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[call-operand-forwarding-impl] row687 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[call-operand-forwarding-impl] row686 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[call-operand-forwarding-impl] check index missing guard entry" >&2; exit 1; }
grep -q 'fn can_forward_same_block_copy_root_to_receiver' "$LOCAL_SSA" || {
  echo "[call-operand-forwarding-impl] missing receiver-only copy-root policy method" >&2
  exit 1
}
grep -q 'fn same_block_copy_root' "$LOCAL_SSA" || {
  echo "[call-operand-forwarding-impl] missing same-block copy-root helper" >&2
  exit 1
}
grep -q 'matches!(self, LocalKind::Recv)' "$LOCAL_SSA" || {
  echo "[call-operand-forwarding-impl] keeper must stay receiver-only" >&2
  exit 1
}
if grep -q 'PAGE_HOTPATH_HELPERS\\|acquire_usize\\|selectSinglePageFastPath\\|reuse' "$LOCAL_SSA"; then
  echo "[call-operand-forwarding-impl] LocalSSA must not special-case helper names" >&2
  exit 1
fi

tmp_dir="$(mktemp -d /tmp/hakorune_call_operand_forwarding_impl.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
inventory_report="$tmp_dir/inventory.out"
post_report="$tmp_dir/post.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$INVENTORY" \
  --mir-json "$mir_json" \
  --source-evidence "docs/development/current/main/phases/phase-296x/296x-683-POST-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-OWNER-REFRESH-REPEAT-001.md" \
  --out "$inventory_report"
python3 "$POST" \
  --guard-surface "$PREV_CARD" \
  --post-inventory "$inventory_report" \
  --out "$post_report"

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[call-operand-forwarding-impl] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

for file in "$post_report" "$CARD"; do
  require_line_in_file "$file" "output_contract=hako-mimalloc-call-operand-materialization-forwarding-implementation-v0"
  require_line_in_file "$file" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
  require_line_in_file "$file" "source_evidence=296x-686"
  require_line_in_file "$file" "selected_keeper_shape=same_block_root_receiver_operand_forwarding"
  require_line_in_file "$file" "pre_selected_keeper_candidate_count=2"
  require_line_in_file "$file" "post_selected_keeper_candidate_count=0"
  require_line_in_file "$file" "post_call_operand_unique_copy_count=27"
  require_line_in_file "$file" "post_call_operand_unique_copy_count_upper_bound=27"
  require_line_in_file "$file" "arg_forwarding_enabled=0"
  require_line_in_file "$file" "helper_name_special_case=0"
  require_line_in_file "$file" "requires_dominance_guard=0"
  require_line_in_file "$file" "variable_map_semantics_changed=0"
  require_line_in_file "$file" "phi_lifecycle_changed=0"
  require_line_in_file "$file" "implementation_started=1"
  require_line_in_file "$file" "optimization_open=0"
  require_line_in_file "$file" "winner_claim=0"
  require_line_in_file "$file" "summary=ok"
done

echo "[call-operand-forwarding-impl] ok"

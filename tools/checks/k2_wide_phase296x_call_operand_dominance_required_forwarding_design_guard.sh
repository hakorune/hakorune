#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-691-CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-690-CALL-OPERAND-RESIDUAL-POLICY-SELECTION-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-692-CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-GUARD-SURFACE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_call_operand_dominance_required_forwarding_design_guard.sh"
TOOL="tools/allocator/hako_mimalloc_call_operand_dominance_required_forwarding_design.py"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"

[[ -f "$CARD" ]] || { echo "[call-operand-dominance-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[call-operand-dominance-design] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[call-operand-dominance-design] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[call-operand-dominance-design] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[call-operand-dominance-design] row691 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[call-operand-dominance-design] row690 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[call-operand-dominance-design] row692 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[call-operand-dominance-design] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_call_operand_dominance_design.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
report="$tmp_dir/report.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$TOOL" --mir-json "$mir_json" --policy-selection "$PREV_CARD" --out "$report"

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[call-operand-dominance-design] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

for file in "$report" "$CARD"; do
  require_line_in_file "$file" "output_contract=hako-mimalloc-call-operand-dominance-required-forwarding-design-v0"
  require_line_in_file "$file" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
  require_line_in_file "$file" "source_evidence=296x-690"
  require_line_in_file "$file" "selected_policy_family=dominance_required_call_operand_forwarding"
  require_line_in_file "$file" "pre_candidate_count=14"
  require_line_in_file "$file" "safe_dominance_candidate_count=14"
  require_line_in_file "$file" "unsafe_candidate_count=0"
  require_line_in_file "$file" "safe_receiver_candidate_count=13"
  require_line_in_file "$file" "safe_arg_candidate_count=1"
  require_line_in_file "$file" "selected_keeper_shape=dominance_guarded_receiver_operand_forwarding"
  require_line_in_file "$file" "selected_keeper_candidate_count=13"
  require_line_in_file "$file" "rejected_arg_forwarding_count=1"
  require_line_in_file "$file" "arg_forwarding_enabled=0"
  require_line_in_file "$file" "requires_dominance_guard=1"
  require_line_in_file "$file" "helper_name_special_case=0"
  require_line_in_file "$file" "variable_map_semantics_changed=0"
  require_line_in_file "$file" "phi_lifecycle_changed=0"
  require_line_in_file "$file" "implementation_started=0"
  require_line_in_file "$file" "optimization_open=0"
  require_line_in_file "$file" "winner_claim=0"
  require_line_in_file "$file" "next_task=call_operand_dominance_required_forwarding_guard_surface"
  require_line_in_file "$file" "summary=ok"
done

require_line_in_file "$NEXT_CARD" "Task: CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-GUARD-SURFACE-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-691"
require_line_in_file "$NEXT_CARD" "implementation_started=0"
require_line_in_file "$NEXT_CARD" "winner_claim=0"

echo "[call-operand-dominance-design] ok"

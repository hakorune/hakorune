#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="resolved-if-control-structure-r0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc

FLOW_MOD="$ROOT_DIR/src/mir/resolved_control_flow/mod.rs"
FACADE="$ROOT_DIR/src/mir/resolved_control_flow/if_control/mod.rs"
PRODUCT="$ROOT_DIR/src/mir/resolved_control_flow/if_control/product.rs"
LEDGER="$ROOT_DIR/src/mir/resolved_control_flow/if_control/use_ledger.rs"
ANALYZER="$ROOT_DIR/src/mir/resolved_control_flow/if_control/analyzer.rs"
README="$ROOT_DIR/src/mir/resolved_control_flow/README.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"

guard_require_files "$TAG" "$FLOW_MOD" "$FACADE" "$PRODUCT" "$LEDGER" \
  "$ANALYZER" "$README" "$INDEX"

for file in "$FACADE" "$PRODUCT" "$LEDGER" "$ANALYZER"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "If-control source reached the 800-line hard boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done
facade_lines="$(wc -l < "$FACADE" | tr -d '[:space:]')"
if (( facade_lines >= 760 )); then
  guard_fail "$TAG" "If-control facade reached the 760-line split trigger: $facade_lines"
fi

legacy_flat="$ROOT_DIR/src/mir/resolved_control_flow/if_control.rs"
if [[ -e "$legacy_flat" ]]; then
  guard_fail "$TAG" "flat legacy If-control owner returned: ${legacy_flat#"$ROOT_DIR/"}"
fi

guard_expect_fixed_in_file "$TAG" 'mod analyzer;' "$FACADE" \
  "If-control facade must retain the analyzer child"
guard_expect_fixed_in_file "$TAG" 'mod product;' "$FACADE" \
  "If-control facade must retain the product child"
guard_expect_fixed_in_file "$TAG" 'mod use_ledger;' "$FACADE" \
  "If-control facade must retain the use-ledger child"
guard_expect_fixed_in_file "$TAG" 'pub(crate) mod if_control;' "$FLOW_MOD" \
  "resolved_control_flow must retain one logical If-control registration"
guard_expect_fixed_in_file "$TAG" 'if_control/mod.rs' "$README" \
  "resolved-control-flow README must document the split owner"
guard_expect_fixed_in_file "$TAG" 'resolved_if_control_structure_r0_guard.sh' "$INDEX" \
  "check index must list the reusable If-control structure guard"

registration_count="$(rg -F -o 'pub(crate) mod if_control;' "$FLOW_MOD" | wc -l | tr -d '[:space:]')"
if [[ "$registration_count" != "1" ]]; then
  guard_fail "$TAG" "logical If-control registration must remain unique: count=$registration_count"
fi

for symbol in \
  'struct VerifiedLocatedIfControlV1' \
  'struct VerifiedResolvedFunctionIfControlV1' \
  'fn analyze_resolved_if_control_v1' \
  'fn verify_resolved_function_if_control_v1' \
  'fn verify_resolved_function_if_control_with_direct_call_v1'; do
  count="$(rg -F -o "$symbol" "$FACADE" "$PRODUCT" "$LEDGER" "$ANALYZER" | wc -l | tr -d '[:space:]')"
  if [[ "$count" != "1" ]]; then
    guard_fail "$TAG" "If-control authority symbol must have one definition: $symbol count=$count"
  fi
done

if rg -n \
  'struct VerifiedLocatedIfControlV1|struct VerifiedResolvedFunctionIfControlV1|fn verify_resolved_function_if_control_v1|fn verify_resolved_function_if_control_with_direct_call_v1' \
  "$ROOT_DIR/src/mir/resolved_control_flow" -g '*.rs' \
  | rg -v '/if_control/(product|analyzer)\.rs:'; then
  guard_fail "$TAG" "If-control product/verifier definitions escaped their designated children"
fi

echo "[$TAG] ok (one logical facade, one product/verifier authority, children below size limits)"

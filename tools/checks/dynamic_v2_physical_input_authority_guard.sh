#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="dynamic-v2-physical-input-authority"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

EVIDENCE="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/physical_evidence.rs"
INPUT="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/physical_input.rs"
EXIT_TX="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/exit_transaction.rs"
COSEAL_TESTS="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/coseal/tests.rs"
DEMAND_MOD="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/physical_demand/mod.rs"
DEMAND_MODEL="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/physical_demand/model.rs"
DEMAND_ISSUER="$ROOT_DIR/src/mir/compiler/dynamic_full_body_recipe/physical_demand/issuer.rs"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$EVIDENCE" "$INPUT" "$EXIT_TX" "$COSEAL_TESTS" \
  "$DEMAND_MOD" "$DEMAND_MODEL" "$DEMAND_ISSUER"

guard_expect_fixed_in_file "$TAG" \
  "DYNAMIC_FULL_LOOP_PHYSICAL_ITEM_COUNT_V2: usize = 17" "$EVIDENCE" \
  "physical evidence must retain the exact bounded item coverage"
guard_expect_fixed_in_file "$TAG" \
  "DYNAMIC_FULL_LOOP_PHYSICAL_OPERATION_COUNT_V2: usize = 15" "$EVIDENCE" \
  "physical evidence must retain the exact bounded operation coverage"
guard_expect_fixed_in_file "$TAG" \
  "issue_physical_evidence_v2" "$EVIDENCE" \
  "the source/effect ledger must have one envelope-owned issuer"
guard_expect_fixed_in_file "$TAG" \
  "with_physical_input" "$EXIT_TX" \
  "the final exit transaction must be the sole physical-input ingress"
guard_expect_fixed_in_file "$TAG" \
  "physical_evidence_coseals_exact_placement_operation_and_effect_coverage" "$COSEAL_TESTS" \
  "exact physical evidence coverage test is missing"
guard_expect_fixed_in_file "$TAG" \
  "VerifiedDynamicLoopOperationPhysicalDemandV2" "$DEMAND_MODEL" \
  "Dynamic physical demand must remain a distinct V2 product"
guard_expect_fixed_in_file "$TAG" \
  "physical_demand_consumes_the_complete_view_inside_the_htrb_loan" "$EXIT_TX" \
  "whole-program Dynamic demand HRTB test is missing"

for forbidden in \
  "as_sig(" \
  "as_recipe(" \
  "LoopItemKeyV1::new(" \
  "VerifiedLoopJoinSigV2" \
  "LoopRecipeItemV2"; do
  if rg -F -q -- "$forbidden" "$INPUT"; then
    guard_fail "$TAG" "physical-input view contains forbidden raw/reconstructed authority: $forbidden"
  fi
done

for forbidden in \
  "VerifiedLoopOperationPhysicalDemandV1" \
  "LoopOperationPhysicalDemandV1" \
  "as_sig(" \
  "as_recipe(" \
  "BasicBlockId" \
  "ValueId" \
  "MirBuilder" \
  "operation_row("; do
  if rg -F -q -- "$forbidden" "$DEMAND_MOD" "$DEMAND_MODEL" "$DEMAND_ISSUER"; then
    guard_fail "$TAG" "Dynamic demand contains forbidden V1/physical/single-item authority: $forbidden"
  fi
done

for file in "$EVIDENCE" "$INPUT" "$EXIT_TX" "$DEMAND_MOD" "$DEMAND_MODEL" "$DEMAND_ISSUER"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "source file reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done

echo "[$TAG] ok"

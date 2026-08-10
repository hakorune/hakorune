#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="normal-callable-complete-batch"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

PARSER_LOAN="$ROOT_DIR/src/parser/normal_callable_program_source/semantic_syntax_loan.rs"
BATCH_ISSUER="$ROOT_DIR/src/mir/callable_semantic_batch/issuer.rs"
DEMAND_ISSUER="$ROOT_DIR/src/mir/callable_parameter_demand/issuer.rs"
PACKAGE_ISSUER="$ROOT_DIR/src/mir/normal_callable_semantic_package/issuer.rs"
BATCH_TESTS="$ROOT_DIR/src/mir/callable_semantic_batch/tests.rs"
PACKAGE_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/tests.rs"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" \
  "$PARSER_LOAN" "$BATCH_ISSUER" "$DEMAND_ISSUER" \
  "$PACKAGE_ISSUER" "$BATCH_TESTS" "$PACKAGE_TESTS"

reject_fixed_in_file() {
  local pattern="$1"
  local file="$2"
  local message="$3"
  if rg -F -q -- "$pattern" "$file"; then
    guard_fail "$TAG" "$message"
  fi
}

guard_expect_fixed_in_file "$TAG" \
  "FinalCallableSemanticSyntaxLoanV1" "$PARSER_LOAN" \
  "final source must own the complete callable syntax loan"
guard_expect_fixed_in_file "$TAG" \
  "parameters: Option<Box" "$PARSER_LOAN" \
  "parameter source must remain an exact partial projection"
guard_expect_fixed_in_file "$TAG" \
  ".with_callable_semantic_syntax" "$BATCH_ISSUER" \
  "semantic batch must traverse complete final callable syntax"
reject_fixed_in_file \
  "with_callable_parameter_syntax" "$BATCH_ISSUER" \
  "parameter catalog must not define semantic batch membership"
guard_expect_fixed_in_file "$TAG" \
  "let Some(source_parameters) = row.parameters() else" "$DEMAND_ISSUER" \
  "parameter demand must skip unprojected callable rows without inference"
reject_fixed_in_file \
  "parameter_demands.len() != batch.declarations().len()" "$PACKAGE_ISSUER" \
  "package must not equate partial demand count with complete batch count"
guard_expect_fixed_in_file "$TAG" \
  "MissingDynamicParameterDemand" "$PACKAGE_ISSUER" \
  "selected Dynamic candidate must fail closed without parameter authority"
guard_expect_fixed_in_file "$TAG" \
  "top_level_and_box_methods_share_one_complete_batch" "$BATCH_TESTS" \
  "mixed top-level complete-batch fixture is missing"
guard_expect_fixed_in_file "$TAG" \
  "top_level_and_dynamic_candidate_share_one_complete_package_batch" "$PACKAGE_TESTS" \
  "mixed top-level plus Dynamic package fixture is missing"
guard_expect_fixed_in_file "$TAG" \
  "selected_gate_dynamic_candidate_rejects_without_parameter_authority" "$PACKAGE_TESTS" \
  "missing Dynamic parameter-authority negative is absent"

for file in \
  "$PARSER_LOAN" "$BATCH_ISSUER" "$DEMAND_ISSUER" \
  "$PACKAGE_ISSUER" "$BATCH_TESTS" "$PACKAGE_TESTS"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "source file reached hard 800-line boundary: ${file#"$ROOT_DIR/"} has $lines"
  fi
done

echo "[$TAG] ok"

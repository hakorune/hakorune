#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-generic-g0-normal-package-consumer-i0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CONSUMER="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port/generic_g0.rs"
ROUTE="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port/canonical_route.rs"
PORT="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port.rs"
ADMISSION="$ROOT_DIR/src/mir/builder/normal_top_level_function_admission.rs"
RESOLVED="$ROOT_DIR/src/mir/builder/resolved_lowering/mod.rs"
LOWERER="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/generic_lowerer.rs"
TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline_loop_tests.rs"
ROUTE_TESTS="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port/canonical_route_tests.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-loop-g0-normal-package-function-consumer-i0-2026-09-11.md"
COMPILER_README="$ROOT_DIR/src/mir/compiler/README.md"
BUILDER_README="$ROOT_DIR/src/mir/builder/README.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_generic_g0_normal_package_consumer_i0_guard.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$CONSUMER" "$ROUTE" "$PORT" "$ADMISSION" "$RESOLVED" \
  "$LOWERER" "$TESTS" "$ROUTE_TESTS" "$CARD" "$COMPILER_README" "$BUILDER_README" "$INDEX"

guard_expect_fixed_in_file "$TAG" "with_selected_lowering_input(&key" "$CONSUMER" \
  "normal G0 must borrow the exact selected package input"
guard_expect_fixed_in_file "$TAG" "try_select_top_level_generic_g0_plan_v1" "$CONSUMER" \
  "normal G0 must have one function-level selector"
guard_expect_fixed_in_file "$TAG" "CanonicalCallableRouteV1::GenericG0(plan)" "$CONSUMER" \
  "the selected Generic route must be consumed explicitly"
guard_expect_fixed_in_file "$TAG" "GenericG0(CanonicalGenericG0PlanV1" "$ROUTE" \
  "the canonical route must expose one Generic G0 arm"
guard_expect_fixed_in_file "$TAG" "lower_normal_top_level_function_with_canonical_generic_g0_plan_v1" "$ADMISSION" \
  "top-level admission must consume the selected plan"
guard_expect_fixed_in_file "$TAG" "lower_resolved_generic_g0_function_pending_v1" "$RESOLVED" \
  "the package edge must use the pending canonical lowerer"
guard_expect_fixed_in_file "$TAG" "commit_resolved_pending" "$ADMISSION" \
  "publication must remain on the existing collector terminal"
guard_expect_fixed_in_file "$TAG" "normal_package_routes_top_level_generic_g0_through_existing_terminal" "$TESTS" \
  "the normal package positive acceptance must stay visible"
guard_expect_fixed_in_file "$TAG" "normal_package_generic_g0_reaches_existing_exe_emitter" "$TESTS" \
  "the source-to-EXE acceptance witness must stay visible"
guard_expect_fixed_in_file "$TAG" "generic_g0_selection_rejects_missing_policy_mode" "$ROUTE_TESTS" \
  "invalid policy mode must have a focused negative acceptance"
guard_expect_fixed_in_file "$TAG" "LOOP-G0-NORMAL-PACKAGE-FUNCTION-CONSUMER-I0" "$CARD" \
  "the active I0 card must remain the documentation anchor"
guard_expect_fixed_in_file "$TAG" "Generic G0 normal-package function consumer I0" "$COMPILER_README" \
  "the compiler README must describe the I0 terminal"
guard_expect_fixed_in_file "$TAG" 'Generic G0 top-level `FreeFunction`' "$BUILDER_README" \
  "the builder README must record the bounded package edge"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" \
  "the check index must list the I0 guard"

for forbidden in source_ast VerifiedResolvedSourceUnitV1 route_loop \
  generic_g0_physical_emitter_session; do
  if rg -n -F -- "$forbidden" "$CONSUMER" >/dev/null 2>&1; then
    guard_fail "$TAG" "normal package consumer contains forbidden re-entry: $forbidden"
  fi
done

if rg -n -F -- 'CanonicalCallableRouteV1::GenericG0(_) => with_selected_source_scope' "$PORT" >/dev/null 2>&1; then
  guard_fail "$TAG" "cataloged Generic G0 selection must not fall through to the legacy source route"
fi
guard_expect_fixed_in_file "$TAG" "cataloged-method-outside-i0" "$PORT" \
  "cataloged Generic G0 methods must reject outside this top-level I0"

for file in "$PORT" "$ROUTE" "$CONSUMER" "$ADMISSION" "$RESOLVED" "$LOWERER"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "I0 source reached the 800-line hard boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (exact package loan, explicit Generic arm, pending canonical session, existing Single publication)"

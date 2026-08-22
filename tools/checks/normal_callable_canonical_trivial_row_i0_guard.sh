#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="normal-callable-canonical-trivial-row-i0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

LOAN_PORT="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port.rs"
LOWERING="$ROOT_DIR/src/mir/builder/normal_cataloged_box_method_lowering.rs"
RESOLVED="$ROOT_DIR/src/mir/builder/resolved_lowering/mod.rs"
LIFECYCLE_TEST="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_lifecycle_tests.rs"
README="$ROOT_DIR/src/mir/normal_callable_semantic_package/README.md"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mir-callable-canonical-trivial-row-i0-2026-08-22.md"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" \
  "$LOAN_PORT" "$LOWERING" "$RESOLVED" "$LIFECYCLE_TEST" "$README" "$CARD" "$STATE"

guard_expect_fixed_in_file "$TAG" \
  "CanonicalLoweringPreflightV1::verify_function(input)" "$LOAN_PORT" \
  "canonical callable route must use the existing preflight issuer"
guard_expect_fixed_in_file "$TAG" \
  "classify_canonical_trivial_route(selected.source())" "$LOAN_PORT" \
  "selected resolver input must be classified before Builder effects"
guard_expect_fixed_in_file "$TAG" \
  "CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan)" "$LOAN_PORT" \
  "only the existing trivial plan may enter this I0"
guard_expect_fixed_in_file "$TAG" \
  "CanonicalTrivialRouteV1::Outside" "$LOAN_PORT" \
  "outside shapes must remain an explicit classification"
guard_expect_fixed_in_file "$TAG" \
  "lower_normal_cataloged_static_box_method_with_canonical_trivial_plan_v1" "$LOAN_PORT" \
  "canonical callable consumer is missing"
guard_expect_fixed_in_file "$TAG" \
  "lower_resolved_trivial_function_draft_with_physical_name_v1" "$LOWERING" \
  "canonical callable consumer must use the physical-symbol sibling"
guard_expect_fixed_in_file "$TAG" \
  "capture_resolved_function_pending_session_v1" "$LOWERING" \
  "canonical callable body must use the existing unpublished session"
guard_expect_fixed_in_file "$TAG" \
  "complete_resolved_child_with_physical_loan" "$LOWERING" \
  "canonical callable result must use the existing collector owner"
guard_expect_fixed_in_file "$TAG" \
  "physical_name.unwrap_or_else" "$RESOLVED" \
  "canonical lowerer must preserve the admitted physical symbol"

for file in "$LOAN_PORT" "$LOWERING" "$RESOLVED"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 760 )); then
    guard_fail "$TAG" "production source reached the 760-line split boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

if rg -F -q -- "lower_normal_cataloged_static_box_method_with_signature_and_source_v1" \
  "$LOAN_PORT" "$LOWERING"; then
  guard_fail "$TAG" "old resolved-signature legacy seam must remain retired"
fi
if rg -F -q -- "build_static_method_draft_with_port_v1" "$LOAN_PORT" "$LOWERING"; then
  guard_fail "$TAG" "canonical callable row must not reach the legacy body driver"
fi

canonical_calls="$(rg -F -c -- \
  "lower_normal_cataloged_static_box_method_with_canonical_trivial_plan_v1" "$LOAN_PORT")"
if [[ "$canonical_calls" != "1" ]]; then
  guard_fail "$TAG" "canonical callable consumer must have one production call: count=$canonical_calls"
fi
canonical_definitions="$(rg -F -c -- \
  "pub(in crate::mir::builder) fn lower_normal_cataloged_static_box_method_with_canonical_trivial_plan_v1" \
  "$LOWERING")"
if [[ "$canonical_definitions" != "1" ]]; then
  guard_fail "$TAG" "canonical callable consumer definition count must be one: count=$canonical_definitions"
fi
physical_name_calls="$(rg -F -o -- \
  "lower_resolved_trivial_function_draft_with_physical_name_v1" "$RESOLVED" "$LOWERING" | wc -l | tr -d '[:space:]')"
if [[ "$physical_name_calls" != "2" ]]; then
  guard_fail "$TAG" "physical-symbol lowerer must have one definition and one caller: count=$physical_name_calls"
fi

guard_expect_fixed_in_file "$TAG" \
  "fn source_backed_selected_callable_uses_the_installed_package_port" \
  "$LIFECYCLE_TEST" "canonical trivial positive fixture is missing"
guard_expect_fixed_in_file "$TAG" \
  "static-result-ingress/target-unavailable" "$LIFECYCLE_TEST" \
  "parser-scan evidence must retain the next typed blocker"
guard_expect_fixed_in_file "$TAG" \
  "callable-semantic-lowering/missing-variable-site" "$LIFECYCLE_TEST" \
  "parser-scan negative must prove the old source-row blocker is absent"
guard_expect_fixed_in_file "$TAG" \
  "source_backed_package_failure_is_terminal_before_builder_effects" \
  "$LIFECYCLE_TEST" "pre-effect terminal negative is missing"
guard_expect_fixed_in_file "$TAG" \
  "CallableSemanticLoweringState" "$README" \
  "module README must record the legacy semantic-state boundary"
guard_expect_fixed_in_file "$TAG" \
  "explicit \`Outside\` route" "$README" \
  "module README must record the canonical/outside authority split"
guard_expect_fixed_in_file "$TAG" \
  "CALLABLE-CANONICAL-TRIVIAL-ROW-I0" "$CARD" "active I0 card is missing"
guard_expect_fixed_in_file "$TAG" \
  'current_execution_row = "CALLABLE-CANONICAL-TRIVIAL-ROW-I0"' "$STATE" \
  "CURRENT_STATE must select this I0 while it is active"

echo "[$TAG] ok"

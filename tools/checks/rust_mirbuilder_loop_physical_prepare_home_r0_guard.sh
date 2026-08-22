#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-loop-physical-prepare-home-r0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

PARENT="$ROOT_DIR/src/mir/compiler/loop_physical_prepare.rs"
TESTS="$ROOT_DIR/src/mir/compiler/loop_physical_prepare_tests.rs"
REGISTRY="$ROOT_DIR/src/mir/compiler/module_registry.in.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-structure-refactor-queue-d0-2026-08-23.md"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$PARENT" "$TESTS" "$REGISTRY" "$CARD"

parent_lines="$(wc -l < "$PARENT" | tr -d '[:space:]')"
test_lines="$(wc -l < "$TESTS" | tr -d '[:space:]')"
if (( parent_lines >= 760 )); then
  guard_fail "$TAG" "loop physical prepare parent must stay below the 760-line split trigger: $parent_lines"
fi
if (( parent_lines >= 800 || test_lines >= 800 )); then
  guard_fail "$TAG" "loop physical prepare source reached the 800-line hard boundary: parent=$parent_lines tests=$test_lines"
fi

guard_expect_fixed_in_file "$TAG" '#![cfg(test)]' "$PARENT" \
  "the physical prepare owner must remain test-only"
guard_expect_fixed_in_file "$TAG" '#[cfg(test)]' "$PARENT" \
  "the external test child must remain test-gated"
guard_expect_fixed_in_file "$TAG" '#[path = "loop_physical_prepare_tests.rs"]' "$PARENT" \
  "the existing test cluster must remain an external path child"
guard_expect_fixed_in_file "$TAG" 'mod tests;' "$PARENT" \
  "the logical tests module must remain registered"
guard_expect_fixed_in_file "$TAG" '#[cfg(test)]' "$REGISTRY" \
  "the compiler registry owner must remain test-gated"
guard_expect_fixed_in_file "$TAG" 'pub(crate) mod loop_physical_prepare;' "$REGISTRY" \
  "the compiler registry must retain the existing owner"
guard_expect_fixed_in_file "$TAG" 'use super::*;' "$TESTS" \
  "the extracted test cluster must retain the parent module surface"
guard_expect_fixed_in_file "$TAG" 'MIRBUILDER-LOOP-PHYSICAL-PREPARE-HOME-R0' "$CARD" \
  "the structure queue must retain the bounded BoxShape row"

if rg -n '^mod tests \{' "$PARENT"; then
  guard_fail "$TAG" "the test cluster is still embedded in the parent file"
fi

for symbol in \
  demand_owns_the_co_seal_after_source_views_are_dropped \
  input_brand_rejects_a_root_view_before_any_product_is_opened \
  input_brand_accepts_the_exact_catalog_view \
  input_brand_rejects_foreign_catalog_and_header_views \
  current_method_call_fixture_is_a_typed_missing_target_boundary \
  resolver_static_fixture_produces_declaration_backed_prepared_positive; do
  guard_expect_fixed_in_file "$TAG" "$symbol" "$TESTS" "test symbol must remain in the extracted cluster: $symbol"
  if rg -n -F "$symbol" "$PARENT"; then
    guard_fail "$TAG" "test symbol remains embedded in the parent: $symbol"
  fi
done

echo "[$TAG] ok (parent=$parent_lines lines, tests=$test_lines lines, six tests preserved, cfg(test) retained)"

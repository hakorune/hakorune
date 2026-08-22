#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-builder-test-home-r0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

BUILDER="$ROOT_DIR/src/mir/builder.rs"
TESTS="$ROOT_DIR/src/mir/builder/builder_binding_id_tests.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-structure-refactor-queue-d0-2026-08-23.md"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$BUILDER" "$TESTS" "$CARD"

builder_lines="$(wc -l < "$BUILDER" | tr -d '[:space:]')"
test_lines="$(wc -l < "$TESTS" | tr -d '[:space:]')"
if (( builder_lines >= 760 )); then
  guard_fail "$TAG" "production barrel must stay below the 760-line split trigger: $builder_lines"
fi
if (( builder_lines >= 800 || test_lines >= 800 )); then
  guard_fail "$TAG" "builder test-home source reached the 800-line hard boundary: builder=$builder_lines tests=$test_lines"
fi

guard_expect_fixed_in_file "$TAG" '#[path = "builder/builder_binding_id_tests.rs"]' "$BUILDER" \
  "binding tests must remain an external path child"
guard_expect_fixed_in_file "$TAG" 'mod binding_id_tests;' "$BUILDER" \
  "the logical binding_id_tests module must remain registered once"
guard_expect_fixed_in_file "$TAG" 'use super::*;' "$TESTS" \
  "the extracted test cluster must retain the parent module surface"
guard_expect_fixed_in_file "$TAG" 'test_binding_map_initialization' "$TESTS" \
  "binding test symbol must remain in the extracted cluster"
guard_expect_fixed_in_file "$TAG" 'test_binding_allocation_sequential' "$TESTS" \
  "binding allocation test symbol must remain in the extracted cluster"
guard_expect_fixed_in_file "$TAG" 'test_shadowing_binding_restore' "$TESTS" \
  "shadowing test symbol must remain in the extracted cluster"
guard_expect_fixed_in_file "$TAG" 'test_valueid_binding_parallel_allocation' "$TESTS" \
  "parallel allocator test symbol must remain in the extracted cluster"
guard_expect_fixed_in_file "$TAG" 'MIRBUILDER-BUILDER-TEST-HOME-R0' "$CARD" \
  "structure queue must retain the selected BoxShape row"

for symbol in \
  test_binding_map_initialization \
  test_binding_allocation_sequential \
  test_shadowing_binding_restore \
  test_valueid_binding_parallel_allocation; do
  if rg -n -F "$symbol" "$BUILDER"; then
    guard_fail "$TAG" "test symbol remains embedded in production barrel: $symbol"
  fi
done

echo "[$TAG] ok (builder=$builder_lines lines, test_home=$test_lines lines, four symbols preserved)"

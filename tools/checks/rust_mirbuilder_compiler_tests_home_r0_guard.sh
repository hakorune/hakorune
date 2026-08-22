#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-compiler-tests-home-r0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FACADE="$ROOT_DIR/src/mir/compiler/tests.rs"
COMPILER_MOD="$ROOT_DIR/src/mir/compiler/mod.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-structure-refactor-queue-d0-2026-08-23.md"
TEST_DIR="$ROOT_DIR/src/mir/compiler/tests"
TEST_GROUPS=(
  finish_schedule.rs
  numeric_contracts.rs
  basic_lowering.rs
  string_corridor.rs
  method_id.rs
  await_lowering.rs
  exception_control.rs
)
TEST_NAMES=(
  trivial_binding_ssa_finish_schedule_skips_legacy_rc
  current_canonical_and_legacy_finish_schedules_keep_legacy_rc
  selected_dynamic_finish_schedule_skips_legacy_postseal_mutators
  selected_dynamic_finish_schedule_rejects_scrubbed_or_partial_metadata
  test_basic_mir_compilation
  canonical_verification_failure_discards_candidate_before_commit
  compile_attaches_dynamic_integer_range_contract_before_verify
  compile_preserves_exact_numeric_signature_facts
  compile_publishes_declared_method_param_types_to_signature
  compile_publishes_exact_numeric_box_field_proof_from_ordinary_literal
  compile_rejects_out_of_range_ordinary_literal_at_exact_numeric_box_field
  test_mir_dump
  test_lowering_is_type_function_call_in_print
  test_lowering_is_method_call_in_print
  test_lowering_extern_console_log
  test_lowering_boxcall_array_push
  test_compile_attaches_string_corridor_fact_for_string_length
  test_compile_attaches_string_corridor_candidate_for_string_length
  test_boxcall_method_id_on_universal_slot
  test_lowering_await_expression
  test_await_has_checkpoints
  test_rewritten_await_still_checkpoints
  test_throw_compilation
  test_loop_compilation
  test_try_catch_compilation
)

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$FACADE" "$COMPILER_MOD" "$CARD"
guard_require_files "$TAG" "${TEST_GROUPS[@]/#/$TEST_DIR/}"

facade_lines="$(wc -l < "$FACADE" | tr -d '[:space:]')"
if (( facade_lines >= 760 )); then
  guard_fail "$TAG" "compiler tests facade must stay below the 760-line split trigger: $facade_lines"
fi

guard_expect_fixed_in_file "$TAG" '#[cfg(test)]' "$COMPILER_MOD" \
  "compiler tests must remain test-gated"
guard_expect_fixed_in_file "$TAG" 'mod tests;' "$COMPILER_MOD" \
  "compiler tests logical parent must remain registered"
guard_expect_fixed_in_file "$TAG" '#[path = "tests/finish_schedule.rs"]' "$FACADE" \
  "finish schedule group must remain behind the facade"
guard_expect_fixed_in_file "$TAG" '#[path = "tests/exception_control.rs"]' "$FACADE" \
  "exception/control group must remain behind the facade"
guard_expect_fixed_in_file "$TAG" 'MIRBUILDER-COMPILER-TESTS-HOME-R0' "$CARD" \
  "structure queue must retain the selected compiler test-home row"

if rg -n '^#\[test\]|^fn ' "$FACADE"; then
  guard_fail "$TAG" "test bodies or test attributes remain in the facade"
fi

test_count="$(rg -l '^#\[test\]' "${TEST_GROUPS[@]/#/$TEST_DIR/}" | xargs -r rg -c '^#\[test\]' | awk -F: '{sum += $NF} END {print sum + 0}')"
ignore_count="$(rg -l '^#\[ignore' "${TEST_GROUPS[@]/#/$TEST_DIR/}" | xargs -r rg -c '^#\[ignore' | awk -F: '{sum += $NF} END {print sum + 0}')"
if [[ "$test_count" != 25 ]]; then
  guard_fail "$TAG" "expected 25 compiler tests after split, got $test_count"
fi
if [[ "$ignore_count" != 6 ]]; then
  guard_fail "$TAG" "expected 6 ignored compiler tests after split, got $ignore_count"
fi

for test_name in "${TEST_NAMES[@]}"; do
  test_matches="$(rg -l -F "$test_name" "$TEST_DIR" | wc -l | tr -d '[:space:]')"
  if [[ "$test_matches" != 1 ]]; then
    guard_fail "$TAG" "expected exactly one moved compiler test symbol: $test_name (files=$test_matches)"
  fi
done

for group in "${TEST_GROUPS[@]}"; do
  group_file="$TEST_DIR/$group"
  group_lines="$(wc -l < "$group_file" | tr -d '[:space:]')"
  if (( group_lines >= 800 )); then
    guard_fail "$TAG" "test group reached the 800-line hard boundary: $group=$group_lines"
  fi
  guard_expect_fixed_in_file "$TAG" 'use super::*;' "$group_file" \
    "test group must consume only the facade surface: $group"
done

echo "[$TAG] ok (facade=$facade_lines lines, groups=7, tests=25, ignored=6)"

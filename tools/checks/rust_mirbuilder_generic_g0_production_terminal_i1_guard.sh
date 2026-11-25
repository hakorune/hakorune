#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-generic-g0-production-terminal-i1"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

LOWERER="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/generic_lowerer.rs"
RESOLVED_MOD="$ROOT_DIR/src/mir/builder/resolved_lowering/mod.rs"
ADMISSION="$ROOT_DIR/src/mir/compiler/generic_g0_physical_operation_cohort/emitter_admission.rs"
ADMISSION_TESTS="$ROOT_DIR/src/mir/compiler/generic_g0_physical_operation_cohort/emitter_admission_tests.rs"
COHORT="$ROOT_DIR/src/mir/compiler/generic_g0_physical_operation_cohort.rs"
ARITY_TESTS="$ROOT_DIR/src/mir/compiler/generic_g0_physical_function_entry_input.rs"
AFTER_TESTS="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/generic_production_canary_tests.rs"
RECURSIVE_AFTER="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/recursive_after.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-loop-g0-production-terminal-i1-2026-09-11.md"
README="$ROOT_DIR/src/mir/compiler/README.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_generic_g0_production_terminal_i1_guard.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$LOWERER" "$RESOLVED_MOD" \
  "$ADMISSION" "$ADMISSION_TESTS" "$COHORT" "$ARITY_TESTS" \
  "$AFTER_TESTS" "$RECURSIVE_AFTER" "$CARD" \
  "$README" "$INDEX"

guard_expect_fixed_in_file "$TAG" "lower_generic_g0_function_draft_pending_v1" "$LOWERER" \
  "Generic G0 must have one named production lowerer"
guard_expect_fixed_in_file "$TAG" "admission.into_session_preflight()" "$LOWERER" \
  "the lowerer must consume the prepared admission"
guard_expect_fixed_in_file "$TAG" "finish_profile_close" "$LOWERER" \
  "the lowerer must use the existing profile-close terminal"
guard_expect_fixed_in_file "$TAG" "prepared.commit_pending()" "$LOWERER" \
  "the lowerer must retain the unpublished draft through DraftSeal"
guard_expect_fixed_in_file "$TAG" "physical_callable_lane_count" "$ARITY_TESTS" \
  "physical lane count must come from the source-owned storage projection"
guard_expect_fixed_in_file "$TAG" "tail: VerifiedGenericG0TailCapabilityV1" "$COHORT" \
  "the source-owned cohort must retain the Generic tail"
guard_expect_fixed_in_file "$TAG" "issue_generic_g0_physical_emitter_admission_from_source_parent_v1" "$ADMISSION" \
  "the production path must use the source-parent admission issuer"
guard_expect_fixed_in_file "$TAG" "entry_coverage_ok" "$ADMISSION" \
  "admission must pin carrier-entry coverage before the lowerer"
guard_expect_fixed_in_file "$TAG" "EntryCoverageMismatch" "$ADMISSION" \
  "missing carrier-entry coverage must have a typed reject"
guard_expect_fixed_in_file "$TAG" "rejects_missing_carrier_entry_before_lowerer_publication" "$ADMISSION_TESTS" \
  "focused tests must prove producer/dispatch carrier loss rejects at admission"
guard_expect_fixed_in_file "$TAG" "generic_g0_computed_condition_left_reaches_real_recursive_after" "$AFTER_TESTS" \
  "focused tests must exercise computed-left through the real After path"
guard_expect_fixed_in_file "$TAG" "prepare_recursive_after_v1(completed" "$AFTER_TESTS" \
  "computed-left evidence must call the real recursive After preparer"
guard_expect_fixed_in_file "$TAG" "prepared.descriptors().len(), 3" "$ARITY_TESTS" \
  "focused tests must pin receiver plus two explicit physical lanes"
guard_expect_fixed_in_file "$TAG" "Generic G0 production terminal I1" "$CARD" \
  "the active card must record the bounded production terminal"
guard_expect_fixed_in_file "$TAG" "Generic G0 production terminal I1" "$README" \
  "the compiler README must document the production terminal"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" \
  "check index must list the Generic G0 terminal guard"

if ! rg -U -q -- '#\[cfg\(test\)\]\nmod generic_g0_physical_emitter_session;' "$RESOLVED_MOD"; then
  guard_fail "$TAG" "the legacy Generic emitter session must remain test-only"
fi

if rg -n -F -- 'recursive_after_uses_explicit_predicate_for_computed_condition_left' "$RECURSIVE_AFTER" >/dev/null 2>&1; then
  guard_fail "$TAG" "computed-left evidence must not remain a tautological helper-only test"
fi

production_session_refs="$({
  rg -l --glob '*.rs' -F 'generic_g0_physical_emitter_session' "$ROOT_DIR/src/mir" || true
} | grep -v '/generic_g0_physical_emitter_session.rs$' | grep -v '/resolved_lowering/mod.rs$' || true)"
if [[ -n "$production_session_refs" ]]; then
  guard_fail "$TAG" "test-only Generic emitter session gained a production reference: $production_session_refs"
fi

for forbidden in route_loop publish_once GenericG0NotActivated generic_g0_physical_emitter_session; do
  if rg -n -F -- "$forbidden" "$LOWERER" >/dev/null 2>&1; then
    guard_fail "$TAG" "production Generic G0 terminal contains a forbidden alternate authority: $forbidden"
  fi
done

PENDING="$ROOT_DIR/src/mir/builder/resolved_lowering/function_draft_entry.rs"
ADMISSION_OWNER="$ROOT_DIR/src/mir/builder/normal_top_level_function_admission.rs"
guard_expect_fixed_in_file "$TAG" "lower_resolved_generic_g0_function_pending_v1(" "$ADMISSION_OWNER" \
  "normal top-level admission must use the pending Generic G0 draft entry"
guard_expect_fixed_in_file "$TAG" "lower_generic_g0_function_draft_pending_v1(" "$PENDING" \
  "the pending draft entry must use the physical lowerer"

for file in "$LOWERER" "$ADMISSION" "$ADMISSION_TESTS" "$COHORT" "$ARITY_TESTS" "$AFTER_TESTS"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "Generic G0 terminal source reached the 800-line hard boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (normal pending draft admission, physical lowerer, test-only legacy session)"

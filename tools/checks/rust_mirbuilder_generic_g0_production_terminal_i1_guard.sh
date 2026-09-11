#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-generic-g0-production-terminal-i1"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

LOWERER="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/generic_lowerer.rs"
RESOLVED_MOD="$ROOT_DIR/src/mir/builder/resolved_lowering/mod.rs"
CUTOVER="$ROOT_DIR/src/mir/compiler/resolved_generic_g0_cutover.rs"
COMPILER="$ROOT_DIR/src/mir/compiler/mod.rs"
PACKAGE="$ROOT_DIR/src/mir/compiler/source_bound_package.rs"
G0_PACKAGE="$ROOT_DIR/src/mir/compiler/source_bound_package_generic_g0.rs"
ADMISSION="$ROOT_DIR/src/mir/compiler/generic_g0_physical_operation_cohort/emitter_admission.rs"
ADMISSION_TESTS="$ROOT_DIR/src/mir/compiler/generic_g0_physical_operation_cohort/emitter_admission_tests.rs"
COHORT="$ROOT_DIR/src/mir/compiler/generic_g0_physical_operation_cohort.rs"
PLAN="$ROOT_DIR/src/mir/compiler/capability/first_family_plan.rs"
TESTS="$ROOT_DIR/src/mir/compiler/generic_g0_capability_tests.rs"
AFTER_TESTS="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/generic_production_canary_tests.rs"
RECURSIVE_AFTER="$ROOT_DIR/src/mir/builder/resolved_lowering/loop_recipe_physicalizer/recursive_after.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-loop-g0-production-terminal-i1-2026-09-11.md"
README="$ROOT_DIR/src/mir/compiler/README.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_generic_g0_production_terminal_i1_guard.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$LOWERER" "$RESOLVED_MOD" "$CUTOVER" "$COMPILER" \
  "$PACKAGE" "$G0_PACKAGE" "$ADMISSION" "$ADMISSION_TESTS" "$COHORT" "$PLAN" "$TESTS" \
  "$AFTER_TESTS" "$RECURSIVE_AFTER" "$CARD" \
  "$README" "$INDEX"

guard_expect_fixed_in_file "$TAG" "lower_generic_g0_function_draft_v1" "$LOWERER" \
  "Generic G0 must have one named production lowerer"
guard_expect_fixed_in_file "$TAG" "admission.into_session_preflight()" "$LOWERER" \
  "the lowerer must consume the prepared admission"
guard_expect_fixed_in_file "$TAG" "finish_profile_close" "$LOWERER" \
  "the lowerer must use the existing profile-close terminal"
guard_expect_fixed_in_file "$TAG" "prepared.commit().consume_non_authority_evidence()" "$LOWERER" \
  "the lowerer must return an unpublished draft through DraftSeal"
guard_expect_fixed_in_file "$TAG" "compile_generic_g0_source_bound" "$CUTOVER" \
  "the Generic G0 switch must have one source-bound cutover"
guard_expect_fixed_in_file "$TAG" "commit_prepared_module" "$CUTOVER" \
  "publication must use the existing external commit"
guard_expect_fixed_in_file "$TAG" "physical_callable_lane_count" "$PLAN" \
  "physical lane count must come from the source-owned storage projection"
guard_expect_fixed_in_file "$TAG" "collect_single(&header, physical_arity, draft)" "$PACKAGE" \
  "Single collection must consume the explicit physical arity"
guard_expect_fixed_in_file "$TAG" "Some(physical_arity)," "$G0_PACKAGE" \
  "Generic G0 must carry physical arity beside logical header identity"
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
guard_expect_fixed_in_file "$TAG" "CanonicalGenericG0BoundaryErrorV1" "$CUTOVER" \
  "the production cutover must preserve typed Generic G0 stage identity"
guard_expect_fixed_in_file "$TAG" "generic_g0_prepared_commit_failure_discards_unpublished_module" "$TESTS" \
  "focused tests must prove zero-publication late failure"
guard_expect_fixed_in_file "$TAG" "function.signature.params.len(), 3" "$TESTS" \
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

if rg -n -F -- 'bridge_error' "$CUTOVER" >/dev/null 2>&1; then
  guard_fail "$TAG" "production Generic G0 cutover still contains the debug-string bridge_error"
fi
if rg -n -F -- 'format!("generic_g0/' "$CUTOVER" >/dev/null 2>&1; then
  guard_fail "$TAG" "production Generic G0 cutover still formats typed rejects into stage strings"
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
  if rg -n -F -- "$forbidden" "$LOWERER" "$CUTOVER" >/dev/null 2>&1; then
    guard_fail "$TAG" "production Generic G0 terminal contains a forbidden alternate authority: $forbidden"
  fi
done

generic_switch_count="$(rg -F -o -- 'CanonicalLoopFamilyPlanV1::GenericG0(plan)' "$COMPILER" | wc -l | tr -d '[:space:]')"
if [[ "$generic_switch_count" -ne 1 ]]; then
  guard_fail "$TAG" "compiler Generic G0 switch must have exactly one production arm; found $generic_switch_count"
fi
generic_lower_calls="$({
  rg -l --glob '*.rs' -F 'lower_resolved_generic_g0_function_draft(' "$ROOT_DIR/src/mir" || true
} | grep -v '/resolved_lowering/mod.rs$' | grep -v '_tests.rs$' || true)"
if [[ "$generic_lower_calls" != "$PACKAGE" ]]; then
  guard_fail "$TAG" "Generic G0 lowerer must have exactly one production package caller: $generic_lower_calls"
fi
commit_count="$(rg -F -o -- 'commit_prepared_module(' "$CUTOVER" | wc -l | tr -d '[:space:]')"
if [[ "$commit_count" -ne 1 ]]; then
  guard_fail "$TAG" "Generic G0 cutover must have exactly one commit edge; found $commit_count"
fi

for file in "$LOWERER" "$CUTOVER" "$PACKAGE" "$G0_PACKAGE" "$ADMISSION" "$ADMISSION_TESTS" "$COHORT" "$PLAN" "$TESTS" "$AFTER_TESTS"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "Generic G0 terminal source reached the 800-line hard boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (one source switch, one physical lowerer, existing Single publication, test-only legacy session)"

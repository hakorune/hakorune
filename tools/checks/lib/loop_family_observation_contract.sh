#!/usr/bin/env bash

# Shared caller-zero contract for the bounded loop-family observers.
#
# The helper is intentionally family-parameterized so each row gets the same
# authority/line/caller checks without growing a row-specific guard block.

guard_loop_family_observation_one() {
  local root_dir="$1"
  local tag="$2"
  local family="$3"
  local stem="$4"
  local policy_fn="$5"
  local adapter_fn="$6"
  local test_count="$7"
  local source_root="$root_dir/src/mir"
  local structural_source="$source_root/loop_structural_facts/${stem}_source.rs"
  local structural_observation="$source_root/loop_structural_facts/${stem}_observation.rs"
  local compiler_projection="$root_dir/src/mir/compiler/${stem}_projection.rs"
  local compiler_adapter="$root_dir/src/mir/compiler/${stem}_observation.rs"
  local policy="$root_dir/src/mir/loop_route_policy/${stem}_observation.rs"
  local tests="$root_dir/src/mir/loop_route_policy/${stem}_observation_tests.rs"

  guard_require_files "$tag" "$structural_source" "$structural_observation" \
    "$compiler_projection" "$compiler_adapter" "$policy" "$tests"
  for file in "$structural_source" "$structural_observation" "$compiler_projection" \
    "$compiler_adapter" "$policy" "$tests"; do
    local lines
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    (( lines < 800 )) || guard_fail "$tag" "$family observer file exceeds boundary: $file"
  done

  [[ "$(rg -o -F '#[test]' "$tests" | wc -l | tr -d '[:space:]')" == "$test_count" ]] ||
    guard_fail "$tag" "$family focused test count drift"

  for forbidden in ASTNode MirBuilder ValueId BasicBlockId LoopRouteId Retry \
    'crate::mir::builder' 'loop_recipe_contract' 'route_loop(' \
    'try_cf_loop_joinir(' 'FrozenLoopRouteScheduleV1'; do
    if rg -n -F "$forbidden" "$structural_source" "$structural_observation" "$policy" >/dev/null; then
      guard_fail "$tag" "$family observer crossed forbidden authority: $forbidden"
    fi
  done

  for required in "Verified${family}BreakContinueSourceProjectionV1" \
    "Verified${family}BreakContinueSourceShapeV1" 'matches_source_identity('; do
    rg -n -F "$required" "$structural_source" "$compiler_projection" >/dev/null ||
      guard_fail "$tag" "$family source identity/product anchor missing: $required"
  done
  for required in "${family}SourceAttemptOutcomeV1" "${family}SourceIdentityV1" \
    "${family}ObservationCoverageV1" "Verified${family}SourceAttemptV1"; do
    rg -n -F "$required" "$structural_observation" >/dev/null ||
      guard_fail "$tag" "$family neutral source-attempt anchor missing: $required"
  done
  for required in "${family}FamilyObservationV1" "${policy_fn}(" \
    "${family}ObservationContextV1" "Verified${family}FamilyCandidateV1"; do
    rg -n -F "$required" "$policy" >/dev/null ||
      guard_fail "$tag" "$family policy observer anchor missing: $required"
  done

  if rg -l -F "${policy_fn}(" "$source_root" |
    awk -v p="$policy" -v t="$tests" \
      -v m="$root_dir/src/mir/loop_route_policy/mod.rs" \
      -v spine="$root_dir/src/mir/compiler/loop_node_winner_spine.rs" \
      '$0 != p && $0 != t && $0 != m && $0 != spine && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "$family policy observer acquired a production caller"
  fi
  if rg -l -F "${adapter_fn}(" "$source_root" |
    awk -v a="$compiler_adapter" -v t="$tests" \
      -v spine="$root_dir/src/mir/compiler/loop_node_winner_spine.rs" \
      -v wt="$root_dir/src/mir/compiler/loop_family_window_probe_tests.rs" \
      '$0 != a && $0 != t && $0 != spine && $0 != wt && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "$family source adapter escaped caller-zero boundary"
  fi
  for constructor in "${family}SourceIdentityV1::new(" "Verified${family}SourceAttemptV1::new("; do
    if rg -l -F "$constructor" "$source_root" |
      awk -v a="$compiler_adapter" -v t="$tests" -v at="$admission_tests" \
        -v spine="$root_dir/src/mir/compiler/loop_node_winner_spine.rs" \
        '$0 != a && $0 != t && $0 != spine && $0 != "" { found=1 } END { exit found }'; then
      :
    else
      guard_fail "$tag" "$family sealed constructor escaped source/test boundary: $constructor"
    fi
  done
}

guard_loop_family_observation_contract() {
  local root_dir="$1"
  local tag="$2"
  guard_loop_family_observation_one "$root_dir" "$tag" "LoopTrue" \
    "loop_true_break_continue" "issue_loop_true_family_observation_v1" \
    "issue_loop_true_source_attempt_v1" 9
  guard_loop_family_observation_one "$root_dir" "$tag" "LoopCond" \
    "loop_cond_break_continue" "issue_loop_cond_family_observation_v1" \
    "issue_loop_cond_source_attempt_v1" 9
}

guard_generic_g0_observation_contract() {
  local root_dir="$1"
  local tag="$2"
  local source_root="$root_dir/src/mir"
  local structural_source="$source_root/loop_structural_facts/generic_g0/mod.rs"
  local structural_observation="$source_root/loop_structural_facts/generic_g0_observation.rs"
  local compiler_projection="$root_dir/src/mir/compiler/generic_g0_projection/mod.rs"
  local compiler_handoff="$root_dir/src/mir/compiler/generic_g0_projection/handoff.rs"
  local compiler_adapter="$root_dir/src/mir/compiler/generic_g0_observation.rs"
  local compiler_tests="$root_dir/src/mir/compiler/generic_g0_observation_tests.rs"
  local policy="$root_dir/src/mir/loop_route_policy/generic_g0_observation.rs"
  local policy_tests="$root_dir/src/mir/loop_route_policy/generic_g0_observation_tests.rs"

  guard_require_files "$tag" "$structural_source" "$structural_observation" \
    "$compiler_projection" "$compiler_handoff" "$compiler_adapter" "$compiler_tests" \
    "$policy" "$policy_tests"
  for file in "$structural_source" "$structural_observation" "$compiler_projection" \
    "$compiler_handoff" "$compiler_adapter" "$compiler_tests" "$policy" "$policy_tests"; do
    local lines
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    (( lines < 800 )) || guard_fail "$tag" "Generic G0 observer file exceeds boundary: $file"
  done

  rg -n -F 'pub(crate) mod handoff;' "$compiler_projection" >/dev/null ||
    guard_fail "$tag" "Generic G0 handoff module is missing from the projection"
  [[ "$(rg -o -F '#[test]' "$compiler_tests" | wc -l | tr -d '[:space:]')" == "5" ]] ||
    guard_fail "$tag" "Generic G0 compiler observation test count drift"
  [[ "$(rg -o -F '#[test]' "$policy_tests" | wc -l | tr -d '[:space:]')" == "8" ]] ||
    guard_fail "$tag" "Generic G0 policy observation test count drift"

  for forbidden in ASTNode MirBuilder ValueId BasicBlockId LoopRouteId Retry \
    'crate::mir::builder' 'loop_recipe_contract' 'route_loop(' \
    'try_cf_loop_joinir(' 'FrozenLoopRouteScheduleV1'; do
    if rg -n -F "$forbidden" "$structural_source" "$structural_observation" "$policy" >/dev/null; then
      guard_fail "$tag" "Generic G0 observer crossed forbidden authority: $forbidden"
    fi
  done
  for required in VerifiedGenericStructuralFactsG0 issue_generic_g0_structural_facts_v1; do
    rg -n -F "$required" "$structural_source" "$compiler_projection" >/dev/null ||
      guard_fail "$tag" "Generic G0 structural product anchor missing: $required"
  done
  for required in GenericG0SourceAttemptOutcomeV1 GenericG0SourceIdentityV1 \
    GenericG0ObservationCoverageV1 VerifiedGenericG0SourceAttemptV1; do
    rg -n -F "$required" "$structural_observation" >/dev/null ||
      guard_fail "$tag" "Generic G0 neutral source-attempt anchor missing: $required"
  done
  for required in GenericG0FamilyObservationV1 issue_generic_g0_family_observation_v1 \
    GenericG0ObservationContextV1 VerifiedGenericG0FamilyCandidateV1; do
    rg -n -F "$required" "$policy" >/dev/null ||
      guard_fail "$tag" "Generic G0 policy observer anchor missing: $required"
  done
  for required in VerifiedGenericG0PolicyHandoffV1 GenericG0SourceBrandV1 \
    VerifiedGenericG0PostLoopReadV1 issue_generic_g0_policy_handoff_v1; do
    rg -n -F "$required" "$structural_source" "$compiler_handoff" "$policy" >/dev/null ||
      guard_fail "$tag" "Generic G0 policy handoff anchor missing: $required"
  done
  for forbidden in ASTNode FunctionSyntaxViewV1 MirBuilder ValueId BasicBlockId \
    loop_recipe_contract RecipeBody LoopRecipeV1 Retry fallback NoCandidate \
    'crate::mir::builder'; do
    if rg -n -F "$forbidden" "$compiler_handoff" "$structural_source" >/dev/null; then
      guard_fail "$tag" "Generic G0 policy handoff crossed forbidden authority: $forbidden"
    fi
  done
  if rg -l -F 'issue_generic_g0_policy_handoff_v1(' "$source_root" |
    awk -v h="$compiler_handoff" -v a="$compiler_adapter" -v pt="$policy_tests" \
      -v gt="$root_dir/src/mir/loop_route_policy/generic_g0_tests.rs" \
      '$0 != h && $0 != a && $0 != pt && $0 != gt && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "Generic G0 policy handoff acquired a production caller"
  fi

  if rg -l -F 'issue_generic_g0_family_observation_v1(' "$source_root" |
    awk -v p="$policy" -v t="$policy_tests" \
      -v m="$root_dir/src/mir/loop_route_policy/mod.rs" \
      -v spine="$root_dir/src/mir/compiler/loop_node_winner_spine.rs" \
      '$0 != p && $0 != t && $0 != m && $0 != spine && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "Generic G0 policy observer acquired a production caller"
  fi
  if rg -l -F 'issue_generic_g0_source_attempt_v1(' "$source_root" |
    awk -v a="$compiler_adapter" -v ct="$compiler_tests" -v pt="$policy_tests" \
      -v wt="$root_dir/src/mir/compiler/loop_family_window_probe_tests.rs" \
      '$0 != a && $0 != ct && $0 != pt && $0 != wt && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "Generic G0 source adapter escaped caller-zero boundary"
  fi
  for constructor in 'GenericG0SourceIdentityV1::new(' 'VerifiedGenericG0SourceAttemptV1::new('; do
    if rg -l -F "$constructor" "$source_root" |
      awk -v a="$compiler_adapter" -v ct="$compiler_tests" -v pt="$policy_tests" \
        -v spine="$root_dir/src/mir/compiler/loop_node_winner_spine.rs" \
        '$0 != a && $0 != ct && $0 != pt && $0 != spine && $0 != "" { found=1 } END { exit found }'; then
      :
    else
      guard_fail "$tag" "Generic G0 sealed constructor escaped source/test boundary: $constructor"
    fi
  done
}

guard_loop_family_row_context_retention_contract() {
  local root_dir="$1"
  local tag="$2"
  local policy_root="$root_dir/src/mir/loop_route_policy"
  local policies=(
    "$policy_root/direct_accum_observation.rs"
    "$policy_root/nested_predicate_observation.rs"
    "$policy_root/loop_true_break_continue_observation.rs"
    "$policy_root/loop_cond_break_continue_observation.rs"
    "$policy_root/generic_g0_observation.rs"
  )
  local tests=(
    "$policy_root/direct_accum_observation_tests.rs"
    "$policy_root/nested_predicate_observation_tests.rs"
    "$policy_root/loop_true_break_continue_observation_tests.rs"
    "$policy_root/loop_cond_break_continue_observation_tests.rs"
    "$policy_root/generic_g0_observation_tests.rs"
  )

  guard_require_files "$tag" "${policies[@]}" "${tests[@]}"
  for file in "${policies[@]}" "${tests[@]}"; do
    local lines
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    (( lines < 800 )) || guard_fail "$tag" "row-context observer/test exceeds boundary: $file"
  done

  for policy in "${policies[@]}"; do
    rg -q 'ObservationEvidenceV1' "$policy" ||
      guard_fail "$tag" "row-context evidence envelope missing: $policy"
    rg -q 'expected:|observed_identity:|observed_mode:|observed_coverage:' "$policy" ||
      guard_fail "$tag" "row-context evidence fields missing: $policy"
    rg -q 'let \(outcome, identity, mode, coverage\) = attempt\.into_parts\(\);' "$policy" ||
      guard_fail "$tag" "attempt must be decomposed exactly once before early returns: $policy"
    rg -q 'FamilyObservationV1 \{' "$policy" ||
      guard_fail "$tag" "family dispositions must be evidence-bearing struct variants: $policy"
    rg -q 'fn evidence\(&self\)' "$policy" ||
      guard_fail "$tag" "family evidence accessor missing: $policy"
    if rg -n 'FamilyObservationV1::(Declined|Unresolved|Rejected)\(' "$policy" >/dev/null; then
      guard_fail "$tag" "bare reason-only family disposition remains: $policy"
    fi
  done

  for test in "${tests[@]}"; do
    rg -q 'evidence\(\)' "$test" ||
      guard_fail "$tag" "focused row-context evidence assertion missing: $test"
  done
}

guard_loop_family_window_lease_contract() {
  local root_dir="$1"
  local tag="$2"
  local lease="$root_dir/src/mir/resolved_semantics/loop_family_window.rs"
  local source="$root_dir/src/mir/resolved_semantics/loop_region.rs"
  local tests="$root_dir/src/mir/resolved_semantics/loop_family_window_tests.rs"
  local generic_handoff="$root_dir/src/mir/compiler/generic_g0_projection/handoff.rs"

  guard_require_files "$tag" "$lease" "$source" "$tests"
  for file in "$lease" "$source" "$tests"; do
    local lines
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    (( lines < 800 )) || guard_fail "$tag" "window lease file exceeds boundary: $file"
  done

  rg -q 'struct VerifiedLoopFamilyWindowLeaseV1' "$lease" ||
    guard_fail "$tag" "resolver window lease product missing"
  rg -q 'VerifiedResolvedLoopSourceV1' "$lease" ||
    guard_fail "$tag" "window lease must retain resolver source token"
  if rg -n '#\[derive\([^]]*\b(Clone|Copy)\b' "$lease" >/dev/null; then
    guard_fail "$tag" "window lease must remain non-Clone/non-Copy"
  fi
  rg -q 'issue_loop_family_window_lease_v1' "$lease" ||
    guard_fail "$tag" "resolver-only window lease issuer missing"
  rg -q 'resolved_loop_source\(site\)' "$lease" ||
    guard_fail "$tag" "window lease must issue from exact resolver lookup"
  for forbidden in VerifiedResolvedSourceUnitV1 shared_loop_source_window_tests \
    'crate::ast' 'crate::mir::builder' loop_recipe_contract LoopRouteId \
    FrozenLoopRouteScheduleV1; do
    if rg -n -F "$forbidden" "$lease" >/dev/null; then
      guard_fail "$tag" "window lease crossed forbidden authority: $forbidden"
    fi
  done
  [[ "$(rg -o -F '#[test]' "$tests" | wc -l | tr -d '[:space:]')" == "3" ]] ||
    guard_fail "$tag" "window lease focused test count drift"
  if rg -l -F 'issue_loop_family_window_lease_v1(' "$root_dir/src/mir" |
    awk -v l="$lease" -v t="$tests" \
      -v gh="$generic_handoff" \
      -v spine="$root_dir/src/mir/compiler/loop_node_winner_spine.rs" \
      -v cap="$root_dir/src/mir/compiler/generic_g0_capability.rs" \
      -v rot="$root_dir/src/mir/loop_route_policy/all_route_observation_tests.rs" \
      -v got="$root_dir/src/mir/loop_route_policy/generic_g0_observation_tests.rs" \
      -v pat="$root_dir/src/mir/compiler/loop_node_physical_admission_tests.rs" \
      '$0 != l && $0 != t && $0 != gh && $0 != spine && $0 != cap && $0 != rot && $0 != got && $0 != pat && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "window lease escaped its canonical caller boundary"
  fi
}

guard_loop_family_admission_contract() {
  local root_dir="$1"
  local tag="$2"
  local admission="$root_dir/src/mir/loop_route_policy/family_admission.rs"

  guard_require_files "$tag" "$admission"
  local lines
  lines="$(wc -l < "$admission" | tr -d '[:space:]')"
  (( lines < 800 )) || guard_fail "$tag" "admission types file exceeds boundary: $admission"
  for required in LoopFamilyAdmissionModeV1 LoopFamilyAdmissionCoverageV1; do
    rg -n -F "$required" "$admission" >/dev/null ||
      guard_fail "$tag" "Generic G0 admission type missing: $required"
  done
  for forbidden in ASTNode MirBuilder ValueId BasicBlockId LoopRouteId \
    FrozenLoopRouteScheduleV1 loop_recipe_contract family_selection policy.rs \
    candidate_count CandidateCount Overlap OutOfWindow Retry fallback; do
    if rg -n -F "$forbidden" "$admission" >/dev/null; then
      guard_fail "$tag" "admission types crossed selector/lowering authority: $forbidden"
    fi
  done
  if rg -n -e 'issue_(direct_accum|nested_predicate|loop_true|loop_cond|generic_g0)_family_observation_v1|issue_loop_family_window_lease_v1' "$admission" >/dev/null; then
    guard_fail "$tag" "admission types must not issue source evidence"
  fi
}

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
  local admission_tests="$root_dir/src/mir/loop_route_policy/family_admission_tests.rs"
  local selector_tests="$root_dir/src/mir/loop_route_policy/family_selector_tests.rs"

  guard_require_files "$tag" "$structural_source" "$structural_observation" \
    "$compiler_projection" "$compiler_adapter" "$policy" "$tests"
  for file in "$structural_source" "$structural_observation" "$compiler_projection" \
    "$compiler_adapter" "$policy" "$tests"; do
    local lines
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    (( lines < 800 )) || guard_fail "$tag" "$family observer file exceeds boundary: $file"
  done

  rg -q '^#!\[cfg\(test\)\]' "$compiler_adapter" ||
    guard_fail "$tag" "$family compiler adapter must remain cfg(test)-only"
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
    awk -v p="$policy" -v t="$tests" -v at="$admission_tests" -v st="$selector_tests" \
      -v m="$root_dir/src/mir/loop_route_policy/mod.rs" \
      '$0 != p && $0 != t && $0 != at && $0 != st && $0 != m && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "$family policy observer acquired a production caller"
  fi
  if rg -l -F "${adapter_fn}(" "$source_root" |
    awk -v a="$compiler_adapter" -v t="$tests" -v st="$selector_tests" \
      '$0 != a && $0 != t && $0 != st && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "$family source adapter escaped caller-zero boundary"
  fi
  for constructor in "${family}SourceIdentityV1::new(" "Verified${family}SourceAttemptV1::new("; do
    if rg -l -F "$constructor" "$source_root" |
      awk -v a="$compiler_adapter" -v t="$tests" -v at="$admission_tests" \
        -v st="$selector_tests" \
        '$0 != a && $0 != t && $0 != at && $0 != st && $0 != "" { found=1 } END { exit found }'; then
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
    "issue_loop_true_source_attempt_for_test" 9
  guard_loop_family_observation_one "$root_dir" "$tag" "LoopCond" \
    "loop_cond_break_continue" "issue_loop_cond_family_observation_v1" \
    "issue_loop_cond_source_attempt_for_test" 9
}

guard_generic_g0_observation_contract() {
  local root_dir="$1"
  local tag="$2"
  local source_root="$root_dir/src/mir"
  local structural_source="$source_root/loop_structural_facts/generic_g0/mod.rs"
  local structural_observation="$source_root/loop_structural_facts/generic_g0_observation.rs"
  local compiler_projection="$root_dir/src/mir/compiler/generic_g0_projection/mod.rs"
  local compiler_adapter="$root_dir/src/mir/compiler/generic_g0_observation.rs"
  local compiler_tests="$root_dir/src/mir/compiler/generic_g0_observation_tests.rs"
  local policy="$root_dir/src/mir/loop_route_policy/generic_g0_observation.rs"
  local policy_tests="$root_dir/src/mir/loop_route_policy/generic_g0_observation_tests.rs"
  local admission_tests="$root_dir/src/mir/loop_route_policy/family_admission_tests.rs"
  local selector_tests="$root_dir/src/mir/loop_route_policy/family_selector_tests.rs"

  guard_require_files "$tag" "$structural_source" "$structural_observation" \
    "$compiler_projection" "$compiler_adapter" "$compiler_tests" "$policy" "$policy_tests"
  for file in "$structural_source" "$structural_observation" "$compiler_projection" \
    "$compiler_adapter" "$compiler_tests" "$policy" "$policy_tests"; do
    local lines
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    (( lines < 800 )) || guard_fail "$tag" "Generic G0 observer file exceeds boundary: $file"
  done

  rg -q '^#!\[cfg\(test\)\]' "$compiler_adapter" ||
    guard_fail "$tag" "Generic G0 compiler adapter must remain cfg(test)-only"
  [[ "$(rg -o -F '#[test]' "$compiler_tests" | wc -l | tr -d '[:space:]')" == "5" ]] ||
    guard_fail "$tag" "Generic G0 compiler observation test count drift"
  [[ "$(rg -o -F '#[test]' "$policy_tests" | wc -l | tr -d '[:space:]')" == "7" ]] ||
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

  if rg -l -F 'issue_generic_g0_family_observation_v1(' "$source_root" |
    awk -v p="$policy" -v t="$policy_tests" -v at="$admission_tests" -v st="$selector_tests" \
      -v m="$root_dir/src/mir/loop_route_policy/mod.rs" \
      '$0 != p && $0 != t && $0 != at && $0 != st && $0 != m && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "Generic G0 policy observer acquired a production caller"
  fi
  if rg -l -F 'issue_generic_g0_source_attempt_for_test(' "$source_root" |
    awk -v a="$compiler_adapter" -v ct="$compiler_tests" -v pt="$policy_tests" -v st="$selector_tests" \
      '$0 != a && $0 != ct && $0 != pt && $0 != st && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "Generic G0 source adapter escaped caller-zero boundary"
  fi
  for constructor in 'GenericG0SourceIdentityV1::new(' 'VerifiedGenericG0SourceAttemptV1::new('; do
    if rg -l -F "$constructor" "$source_root" |
      awk -v a="$compiler_adapter" -v ct="$compiler_tests" -v pt="$policy_tests" \
        -v at="$admission_tests" -v st="$selector_tests" \
        '$0 != a && $0 != ct && $0 != pt && $0 != at && $0 != st && $0 != "" { found=1 } END { exit found }'; then
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
  local selector_tests="$root_dir/src/mir/loop_route_policy/family_selector_tests.rs"

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
  local assembler_tests="$root_dir/src/mir/loop_route_policy/family_admission_tests.rs"
  if rg -l -F 'issue_loop_family_window_lease_v1(' "$root_dir/src/mir" |
    awk -v l="$lease" -v t="$tests" -v at="$assembler_tests" -v st="$selector_tests" \
      '$0 != l && $0 != t && $0 != at && $0 != st && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "window lease acquired a production caller"
  fi
}

guard_loop_family_admission_contract() {
  local root_dir="$1"
  local tag="$2"
  local assembler="$root_dir/src/mir/loop_route_policy/family_admission.rs"
  local tests="$root_dir/src/mir/loop_route_policy/family_admission_tests.rs"
  local mod_file="$root_dir/src/mir/loop_route_policy/mod.rs"
  local selector_tests="$root_dir/src/mir/loop_route_policy/family_selector_tests.rs"

  guard_require_files "$tag" "$assembler" "$tests"
  for file in "$assembler" "$tests"; do
    local lines
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    (( lines < 800 )) || guard_fail "$tag" "common admission file exceeds boundary: $file"
  done
  [[ "$(rg -o -F '#[test]' "$tests" | wc -l | tr -d '[:space:]')" == "6" ]] ||
    guard_fail "$tag" "common admission focused test count drift"

  for required in LoopFamilyObservationRowV1 VerifiedLoopFamilyAdmissionWindowV1 \
    VerifiedLoopFamilyAdmissionRowsV1 LoopFamilyAdmissionFailureEvidenceV1 \
    LoopFamilyAdmissionIssueV1 'lease:' 'rows:' 'issues:'; do
    rg -n -F "$required" "$assembler" >/dev/null ||
      guard_fail "$tag" "common admission anchor missing: $required"
  done
  for required in DirectAccum NestedPredicate LoopTrue LoopCond GenericG0; do
    rg -n -F "$required" "$assembler" >/dev/null ||
      guard_fail "$tag" "common admission family row missing: $required"
  done
  for forbidden in ASTNode MirBuilder ValueId BasicBlockId LoopRouteId \
    FrozenLoopRouteScheduleV1 loop_recipe_contract family_selection policy.rs \
    'candidate_count' CandidateCount Overlap OutOfWindow Retry fallback; do
    if rg -n -F "$forbidden" "$assembler" >/dev/null; then
      guard_fail "$tag" "common admission crossed selector/lowering authority: $forbidden"
    fi
  done
  if rg -n -e 'issue_(direct_accum|nested_predicate|loop_true|loop_cond|generic_g0)_family_observation_v1|issue_loop_family_window_lease_v1' "$assembler" >/dev/null; then
    guard_fail "$tag" "common admission must consume sealed rows/lease, not reissue them"
  fi
  for required in 'Rejected' 'Unresolved' 'Ready'; do
    rg -n -F "$required" "$assembler" >/dev/null ||
      guard_fail "$tag" "common admission disposition missing: $required"
  done

  if rg -l -F 'assemble_loop_family_admission_window_v1(' "$root_dir/src/mir" |
    awk -v a="$assembler" -v t="$tests" -v m="$mod_file" -v st="$selector_tests" \
      '$0 != a && $0 != t && $0 != m && $0 != st && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "common admission acquired a production caller"
  fi
  if rg -l -F 'into_admission_row(' "$root_dir/src/mir/loop_route_policy" |
    awk -v a="$assembler" -v t="$tests" -v m="$mod_file" -v st="$selector_tests" \
      '$0 != a && $0 != t && $0 != m && $0 != st && \
       $0 !~ /(direct_accum|nested_predicate|loop_true_break_continue|loop_cond_break_continue|generic_g0)_observation\.rs$/ && \
       $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "family row projection escaped the admission boundary"
  fi
}

guard_loop_family_selector_contract() {
  local root_dir="$1"
  local tag="$2"
  local selector="$root_dir/src/mir/loop_route_policy/family_selector.rs"
  local tests="$root_dir/src/mir/loop_route_policy/family_selector_tests.rs"
  local assembler="$root_dir/src/mir/loop_route_policy/family_admission.rs"

  guard_require_files "$tag" "$selector" "$tests" "$assembler"
  for file in "$selector" "$tests"; do
    local lines
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    (( lines < 800 )) || guard_fail "$tag" "family selector file exceeds boundary: $file"
  done
  [[ "$(rg -o -F '#[test]' "$tests" | wc -l | tr -d '[:space:]')" == "3" ]] ||
    guard_fail "$tag" "family selector focused test count drift"

  for required in CanonicalLoopFamilyCandidateV1 CanonicalLoopFamilySelectionOutcomeV1 \
    CanonicalLoopFamilySelectionReasonV1 select_canonical_loop_family_v1 \
    'Selected(' 'Rejected(' 'Unresolved(' 'Overlap' 'OutOfWindow'; do
    rg -n -F "$required" "$selector" >/dev/null ||
      guard_fail "$tag" "family selector anchor missing: $required"
  done
  for forbidden in ASTNode VerifiedResolvedSourceUnitV1 FunctionSourceViewV1 \
    LoopRouteId FrozenLoopRouteScheduleV1 family_selection policy.rs \
    'crate::mir::builder' 'loop_structural_facts' 'issue_.*source_attempt_for_test' \
    NoCandidate retry fallback RuntimeDataBox; do
    if rg -n -F "$forbidden" "$selector" >/dev/null; then
      guard_fail "$tag" "family selector crossed forbidden authority: $forbidden"
    fi
  done
  if rg -l -F 'select_canonical_loop_family_v1(' "$root_dir/src/mir" |
    awk -v s="$selector" -v t="$tests" \
      '$0 != s && $0 != t && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "family selector acquired a production caller"
  fi
  if rg -n -F 'assemble_loop_family_admission_window_v1(' "$selector" >/dev/null; then
    guard_fail "$tag" "family selector must consume Ready windows, not assemble them"
  fi
  rg -n -F 'window.into_parts()' "$selector" >/dev/null ||
    guard_fail "$tag" "family selector must consume its window by value"
}

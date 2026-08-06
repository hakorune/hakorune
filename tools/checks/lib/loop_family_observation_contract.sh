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
    awk -v p="$policy" -v t="$tests" -v m="$root_dir/src/mir/loop_route_policy/mod.rs" \
      '$0 != p && $0 != t && $0 != m && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "$family policy observer acquired a production caller"
  fi
  if rg -l -F "${adapter_fn}(" "$source_root" |
    awk -v a="$compiler_adapter" -v t="$tests" \
      '$0 != a && $0 != t && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "$family source adapter escaped caller-zero boundary"
  fi
  for constructor in "${family}SourceIdentityV1::new(" "Verified${family}SourceAttemptV1::new("; do
    if rg -l -F "$constructor" "$source_root" |
      awk -v a="$compiler_adapter" -v t="$tests" \
        '$0 != a && $0 != t && $0 != "" { found=1 } END { exit found }'; then
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
    awk -v p="$policy" -v t="$policy_tests" -v m="$root_dir/src/mir/loop_route_policy/mod.rs" \
      '$0 != p && $0 != t && $0 != m && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "Generic G0 policy observer acquired a production caller"
  fi
  if rg -l -F 'issue_generic_g0_source_attempt_for_test(' "$source_root" |
    awk -v a="$compiler_adapter" -v ct="$compiler_tests" -v pt="$policy_tests" \
      '$0 != a && $0 != ct && $0 != pt && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "Generic G0 source adapter escaped caller-zero boundary"
  fi
  for constructor in 'GenericG0SourceIdentityV1::new(' 'VerifiedGenericG0SourceAttemptV1::new('; do
    if rg -l -F "$constructor" "$source_root" |
      awk -v a="$compiler_adapter" -v ct="$compiler_tests" -v pt="$policy_tests" \
        '$0 != a && $0 != ct && $0 != pt && $0 != "" { found=1 } END { exit found }'; then
      :
    else
      guard_fail "$tag" "Generic G0 sealed constructor escaped source/test boundary: $constructor"
    fi
  done
}

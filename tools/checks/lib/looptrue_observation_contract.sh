#!/usr/bin/env bash

guard_looptrue_observation_contract() {
  local root_dir="$1"
  local tag="$2"
  local structural_source="$root_dir/src/mir/loop_structural_facts/loop_true_break_continue_source.rs"
  local structural_observation="$root_dir/src/mir/loop_structural_facts/loop_true_break_continue_observation.rs"
  local compiler_projection="$root_dir/src/mir/compiler/loop_true_break_continue_projection.rs"
  local compiler_adapter="$root_dir/src/mir/compiler/loop_true_break_continue_observation.rs"
  local policy="$root_dir/src/mir/loop_route_policy/loop_true_break_continue_observation.rs"
  local tests="$root_dir/src/mir/loop_route_policy/loop_true_break_continue_observation_tests.rs"

  guard_require_files "$tag" "$structural_source" "$structural_observation" \
    "$compiler_projection" "$compiler_adapter" "$policy" "$tests"
  for file in "$structural_source" "$structural_observation" "$compiler_projection" \
    "$compiler_adapter" "$policy" "$tests"; do
    local lines
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    (( lines < 800 )) || guard_fail "$tag" "LoopTrue S1 file exceeds boundary: $file"
  done

  rg -q '^#!\[cfg\(test\)\]' "$compiler_adapter" ||
    guard_fail "$tag" "LoopTrue S1 compiler adapter must remain cfg(test)-only"
  [[ "$(rg -o -F '#[test]' "$tests" | wc -l | tr -d '[:space:]')" == "9" ]] ||
    guard_fail "$tag" "LoopTrue S1 focused test count drift"

  for forbidden in ASTNode MirBuilder ValueId BasicBlockId LoopRouteId Retry \
    'crate::mir::builder' 'loop_recipe_contract' 'route_loop(' \
    'try_cf_loop_joinir(' 'FrozenLoopRouteScheduleV1' \
    'VerifiedLoopTrueBreakContinuePolicyDemandV1'; do
    if rg -n -F "$forbidden" "$structural_source" "$structural_observation" "$policy" >/dev/null; then
      guard_fail "$tag" "LoopTrue S1 observer crossed forbidden authority: $forbidden"
    fi
  done

  for required in VerifiedLoopTrueBreakContinueSourceProjectionV1 \
    VerifiedLoopTrueBreakContinueSourceShapeV1 'matches_source_identity('; do
    rg -n -F "$required" "$structural_source" "$compiler_projection" >/dev/null ||
      guard_fail "$tag" "LoopTrue source identity/product anchor missing: $required"
  done
  for required in LoopTrueSourceAttemptOutcomeV1 LoopTrueSourceIdentityV1 \
    LoopTrueObservationCoverageV1 VerifiedLoopTrueSourceAttemptV1; do
    rg -n -F "$required" "$structural_observation" >/dev/null ||
      guard_fail "$tag" "LoopTrue neutral source-attempt anchor missing: $required"
  done
  for required in LoopTrueFamilyObservationV1 'issue_loop_true_family_observation_v1(' \
    LoopTrueObservationContextV1 VerifiedLoopTrueFamilyCandidateV1; do
    rg -n -F "$required" "$policy" >/dev/null ||
      guard_fail "$tag" "LoopTrue policy observer anchor missing: $required"
  done

  if rg -l -F 'issue_loop_true_family_observation_v1(' "$root_dir/src/mir" |
    awk -v p="$policy" -v t="$tests" -v m="$root_dir/src/mir/loop_route_policy/mod.rs" \
      '$0 != p && $0 != t && $0 != m && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "LoopTrue S1 policy observer acquired a production caller"
  fi
  if rg -l -F 'issue_loop_true_source_attempt_for_test(' "$root_dir/src/mir" |
    awk -v a="$compiler_adapter" -v t="$tests" \
      '$0 != a && $0 != t && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "LoopTrue S1 source adapter escaped caller-zero boundary"
  fi
  for constructor in 'LoopTrueSourceIdentityV1::new(' 'VerifiedLoopTrueSourceAttemptV1::new('; do
    if rg -l -F "$constructor" "$root_dir/src/mir" |
      awk -v a="$compiler_adapter" -v t="$tests" \
        '$0 != a && $0 != t && $0 != "" { found=1 } END { exit found }'; then
      :
    else
      guard_fail "$tag" "LoopTrue S1 sealed constructor escaped source/test boundary: $constructor"
    fi
  done
}

#!/usr/bin/env bash

guard_joinir_loop_compile_candidate_scope() {
  local root_dir="$1"
  local tag="$2"
  local manifest="$root_dir/tools/checks/manifests/joinir_loop_compile_candidate_scope_v1.tsv"
  local routing="$root_dir/src/mir/builder/control_flow/joinir/routing.rs"
  local router="$root_dir/src/mir/builder/control_flow/joinir/route_entry/router.rs"
  local raw_child="$root_dir/src/mir/builder/raw_loop_child_entry.rs"
  local recursive_child="$root_dir/src/mir/builder/recursive_child_lowering.rs"
  local normal="$root_dir/src/mir/compiler/normal_default_pipeline.rs"
  local raw_compile="$root_dir/src/mir/compiler/raw_published_compile.rs"
  local raw_open="$root_dir/src/mir/compiler/raw_root_eligibility.rs"
  local raw_recipe="$root_dir/src/mir/compiler/raw_root_source_facts/recipe_projection.rs"
  local canonical="$root_dir/src/mir/compiler/source_bound_package.rs"
  local canonical_dispatch="$root_dir/src/mir/compiler/canonical_core_dispatch.rs"
  local canonical_input="$root_dir/src/mir/compiler/lowering_input.rs"
  local capability="$root_dir/src/mir/compiler/capability.rs"
  local first_family_plan="$root_dir/src/mir/compiler/capability/first_family_plan.rs"
  local source_bound_plan="$root_dir/src/mir/compiler/source_bound_plan.rs"
  local m1_test="$root_dir/src/mir/compiler/loop_candidate_abort_p0.rs"
  local direct_accum_cutover="$root_dir/src/mir/compiler/resolved_direct_accum_cutover.rs"
  local hardening_test="$root_dir/src/mir/compiler/resolved_direct_accum_hardening_p0.rs"
  local external_commit="$root_dir/src/mir/compiler/external_commit.rs"
  local loop_region="$root_dir/src/mir/resolved_semantics/loop_region.rs"
  local source_adapter="$root_dir/src/mir/loop_structural_facts/resolved_source_adapter.rs"
  local generic_g0_projection="$root_dir/src/mir/compiler/generic_g0_projection/mod.rs"
  local generic_g0_handoff="$root_dir/src/mir/compiler/generic_g0_projection/handoff.rs"
  local generic_g0_projection_tests="$root_dir/src/mir/compiler/generic_g0_projection_tests.rs"
  local generic_g0_source_type_dir="$root_dir/src/mir/resolved_semantics/generic_g0"
  local generic_g0_source_type="$generic_g0_source_type_dir/mod.rs"
  local generic_g0_numeric_dir="$root_dir/src/mir/numeric_substrate/generic_g0"
  local generic_g0_numeric="$generic_g0_numeric_dir/mod.rs"
  local generic_g0_numeric_tests="$generic_g0_numeric_dir/tests.rs"
  local generic_g0_numeric_adapter="$root_dir/src/mir/compiler/generic_g0_projection/numeric.rs"
  local generic_g0_numeric_projection_tests="$root_dir/src/mir/compiler/generic_g0_numeric_projection_tests.rs"
  local generic_g0_policy="$root_dir/src/mir/loop_route_policy/generic_g0.rs"
  local generic_g0_policy_tests="$root_dir/src/mir/loop_route_policy/generic_g0_tests.rs"
  local generic_g0_observation_policy="$root_dir/src/mir/loop_route_policy/generic_g0_observation.rs"
  local generic_g0_observation_policy_tests="$root_dir/src/mir/loop_route_policy/generic_g0_observation_tests.rs"
  local generic_g0_policy_mod="$root_dir/src/mir/loop_route_policy/mod.rs"
  local generic_g0_structural="$root_dir/src/mir/loop_structural_facts/generic_g0/mod.rs"
  local direct_accum_observation_source="$root_dir/src/mir/loop_structural_facts/direct_accum_observation.rs"
  local direct_accum_observation_adapter="$root_dir/src/mir/compiler/direct_accum_observation.rs"
  local direct_accum_observation_policy="$root_dir/src/mir/loop_route_policy/direct_accum_observation.rs"
  local direct_accum_observation_tests="$root_dir/src/mir/loop_route_policy/direct_accum_observation_tests.rs"
  local nested_source_dto="$root_dir/src/mir/loop_structural_facts/nested_predicate_source.rs"
  local nested_observation_source="$root_dir/src/mir/loop_structural_facts/nested_predicate_observation.rs"
  local nested_observation_adapter="$root_dir/src/mir/compiler/nested_predicate_observation.rs"
  local nested_observation_policy="$root_dir/src/mir/loop_route_policy/nested_predicate_observation.rs"
  local nested_observation_tests="$root_dir/src/mir/loop_route_policy/nested_predicate_observation_tests.rs"
  local family_admission_tests="$root_dir/src/mir/loop_route_policy/family_admission_tests.rs"
  local family_selector_source="$root_dir/src/mir/loop_route_policy/family_selector.rs"
  local family_selector_tests="$root_dir/src/mir/loop_route_policy/family_selector_tests.rs"
  local join_sig_dir="$root_dir/src/mir/loop_recipe_contract/join_sig"
  local join_sig_facade="$join_sig_dir/mod.rs"
  local join_sig_model="$join_sig_dir/model.rs"
  local join_sig_port="$join_sig_dir/port.rs"
  local join_sig_visibility="$join_sig_dir/visibility.rs"
  local join_sig_flow="$join_sig_dir/flow.rs"
  local legacy_join_sig="$root_dir/src/mir/loop_recipe_contract/join_sig.rs"
  local nested_source_projection="$root_dir/src/mir/compiler/nested_predicate_projection.rs"
  local nested_recipe_producer="$root_dir/src/mir/compiler/nested_predicate_producer.rs"
  local nested_source_handoff="$root_dir/src/mir/compiler/nested_predicate_source_handoff.rs"
  local nested_topology="$root_dir/src/mir/compiler/nested_predicate_topology.rs"
  local nested_topology_tests="$root_dir/src/mir/compiler/nested_predicate_topology_tests.rs"
  local nested_physical_input="$root_dir/src/mir/compiler/nested_predicate_physical_input.rs"
  local nested_physical_input_tests="$root_dir/src/mir/compiler/nested_predicate_physical_input_tests.rs"
  local nested_effect_plan="$root_dir/src/mir/compiler/nested_predicate_effect_plan.rs"
  local nested_effect_plan_tests="$root_dir/src/mir/compiler/nested_predicate_effect_plan_tests.rs"
  local nested_effect_adapter_tests="$root_dir/src/mir/builder/resolved_lowering/nested_predicate_effect_adapter_tests.rs"
  local producer_facade="$root_dir/src/mir/builder/control_flow/plan/loop_recipe_producer_facade_tests.rs"

  guard_require_files "$tag" "$manifest" "$routing" "$router" "$raw_child" \
    "$recursive_child" "$normal" "$raw_compile" "$raw_open" "$raw_recipe" \
    "$canonical" "$canonical_dispatch" "$canonical_input" "$m1_test" \
    "$direct_accum_cutover" "$hardening_test" "$external_commit" \
    "$capability" "$first_family_plan" "$source_bound_plan" "$loop_region" \
    "$source_adapter" "$generic_g0_projection" "$generic_g0_handoff" "$generic_g0_projection_tests" \
    "$generic_g0_source_type" "$generic_g0_numeric" "$generic_g0_numeric_tests" \
    "$generic_g0_numeric_adapter" "$generic_g0_numeric_projection_tests" \
    "$generic_g0_policy" "$generic_g0_policy_tests" "$generic_g0_policy_mod" "$generic_g0_structural" \
    "$direct_accum_observation_source" "$direct_accum_observation_adapter" \
    "$direct_accum_observation_policy" "$direct_accum_observation_tests" \
    "$nested_source_dto" "$nested_observation_source" "$nested_observation_adapter" \
    "$nested_observation_policy" "$nested_observation_tests" \
    "$join_sig_facade" "$join_sig_model" "$join_sig_port" \
    "$join_sig_visibility" "$join_sig_flow" \
    "$nested_source_projection" "$nested_recipe_producer" \
    "$nested_source_handoff" "$nested_topology" "$nested_topology_tests" \
    "$nested_physical_input" "$nested_physical_input_tests" \
    "$nested_effect_plan" "$nested_effect_plan_tests" "$nested_effect_adapter_tests" \
    "$producer_facade"

  local header=$'ingress_kind\tpublic_ingress\tcandidate_owner\tloop_reachability\tpublication_owner\tambient_write_policy'
  [[ "$(head -n1 "$manifest")" == "$header" ]] || \
    guard_fail "$tag" "Loop candidate scope manifest header drift"
  if ! awk -F '\t' '
    NR == 1 { next }
    NF != 6 { exit 1 }
    $1 !~ /^(normal|repl|raw-public|raw-reference|vm-hako-reference|canonical-resolved|canonical-resolved-direct-accum|canonical-core-script|canonical-core-main)$/ { exit 1 }
    $3 !~ /(SessionV1|TransactionV1)$/ { exit 1 }
    $4 !~ /^(reachable|typed-unreachable)$/ { exit 1 }
    $6 !~ /^identity-monotonic\+diagnostic-scratch/ { exit 1 }
    ($1 ~ /^(normal|repl|vm-hako-reference)$/ && $4 != "reachable") { exit 1 }
    ($1 ~ /^(raw-public|raw-reference|canonical-resolved|canonical-core-)/ && $1 != "canonical-resolved-direct-accum" && $4 != "typed-unreachable") { exit 1 }
    ($1 == "canonical-resolved-direct-accum" && $4 != "reachable") { exit 1 }
    END { if (NR != 10) exit 1 }
  ' "$manifest"; then
    guard_fail "$tag" "Loop candidate scope manifest row contract failed"
  fi

  local route_refs
  route_refs="$(rg -n -F 'route_loop(' "$root_dir/src" --glob '*.rs' || true)"
  if [[ "$(printf '%s\n' "$route_refs" | sed '/tests\//d' | sed '/_tests\.rs:/d' | sed '/::tests::/d' | sed '/pub(crate) fn route_loop/d' | sed '/^$/d' | wc -l | tr -d '[:space:]')" != "1" ]]; then
    guard_fail "$tag" "production route_loop caller count drift"
  fi
  if ! rg -n -F 'route_loop(self, &ctx)?' "$routing" >/dev/null || \
     ! rg -n -F 'pub(crate) fn route_loop(' "$router" >/dev/null; then
    guard_fail "$tag" "route_loop definition/caller anchors drift"
  fi

  local joinir_refs
  joinir_refs="$(rg -n -F 'try_cf_loop_joinir(' "$root_dir/src" --glob '*.rs' || true)"
  if [[ "$(printf '%s\n' "$joinir_refs" | sed '/tests\//d' | sed '/_tests\.rs:/d' | sed '/^$/d' | wc -l | tr -d '[:space:]')" != "2" ]]; then
    guard_fail "$tag" "try_cf_loop_joinir definition/caller count drift"
  fi
  if ! rg -n -F 'lower_loop_or_freeze_v1(' "$routing" "$raw_child" "$recursive_child" >/dev/null; then
    guard_fail "$tag" "shared Loop terminal owner anchors drift"
  fi

  if ! rg -n -F 'route_loop(' "$root_dir/src" --glob '*.rs' \
      | awk -F: '$1 !~ /route_entry\/router\.rs$/ && $1 !~ /joinir\/routing\.rs$/ && $1 !~ /_tests?\.rs$/ { found = 1 } END { exit found }'; then
    guard_fail "$tag" "route_loop caller escaped the sole routing owner"
  fi
  if ! rg -n -F 'try_cf_loop_joinir(' "$root_dir/src" --glob '*.rs' \
      | awk -F: '$1 !~ /joinir\/routing\.rs$/ && $1 !~ /_tests?\.rs$/ { found = 1 } END { exit found }'; then
    guard_fail "$tag" "try_cf_loop_joinir caller escaped the routing owner"
  fi

  for required in \
    "$normal|ModuleBuilderInvocationSessionV1::open_for_token|1" \
    "$normal|prepare_external_commit|1" \
    "$normal|prepared.commit(&mut compiler.builder)|1" \
    "$raw_compile|open_physical(&self.builder)|1" \
    "$raw_compile|prepare_external_commit|1" \
    "$raw_open|ModuleBuilderInvocationSessionV1::open_for_token|1" \
    "$canonical|ModuleBuilderInvocationSessionV1::open_for_token|1" \
    "$direct_accum_cutover|pub(super) fn compile_direct_accum_source_bound(|1" \
    "$canonical|lower_resolved_direct_accum_function_draft|1" \
    "$canonical_dispatch|prepare_normal_main_module_transaction|1"
  do
    local file="${required%%|*}"
    local rest="${required#*|}"
    local pattern="${rest%|*}"
    local expected="${rest##*|}"
    local count="$(rg -o -F "$pattern" "$file" | wc -l | tr -d '[:space:]')"
    [[ "$count" == "$expected" ]] || guard_fail "$tag" "candidate scope anchor drift: $pattern count=$count expected=$expected"
  done
  for generic_policy_file in "$generic_g0_policy" "$generic_g0_policy_tests"; do
    local policy_lines
    policy_lines="$(wc -l < "$generic_policy_file" | tr -d '[:space:]')"
    if (( policy_lines >= 800 )); then
      guard_fail "$tag" "Generic G0 policy file exceeds boundary: ${generic_policy_file#"$root_dir/"} lines=$policy_lines"
    fi
  done
  for forbidden in ASTNode MirBuilder ValueId BasicBlockId LoopRouteId Retry \
    'crate::mir::builder' 'loop_recipe_contract'; do
    if rg -n -F "$forbidden" "$generic_g0_policy" >/dev/null; then
      guard_fail "$tag" "Generic G0 policy imported forbidden authority: $forbidden"
    fi
  done
  for required in \
    'VerifiedGenericTypedSourceBundleG0' \
    'VerifiedGenericFamilyObservationG0' \
    'GenericG0PolicyOutcomeV1' \
    'GenericG0PolicyContextV1' \
    'unsupported_condition'
  do
    rg -n -F "$required" "$generic_g0_policy" >/dev/null || \
      guard_fail "$tag" "Generic G0 policy anchor missing: $required"
  done
  if [[ "$(rg -o -F '#[test]' "$generic_g0_policy_tests" | wc -l | tr -d '[:space:]')" != "7" ]]; then
    guard_fail "$tag" "Generic G0 policy focused test count drift"
  fi
  for required in \
    'GenericG0ConditionOperatorV1' \
    'GenericG0UpdateOperatorV1' \
    'operator: condition_operator' \
    'operator: update_operator'
  do
    rg -n -F "$required" "$generic_g0_structural" "$generic_g0_projection" >/dev/null || \
      guard_fail "$tag" "Generic G0 neutral operator-fact anchor missing: $required"
  done
  for policy_ref in 'issue_generic_g0_candidate_v1(' 'VerifiedGenericFamilyObservationG0'; do
    if rg -n -F "$policy_ref" "$root_dir/src" --glob '*.rs' |
      awk -F: -v issuer="$generic_g0_policy" -v tests="$generic_g0_policy_tests" \
        -v policy_mod="$generic_g0_policy_mod" -v observation="$generic_g0_observation_policy" \
        -v observation_tests="$generic_g0_observation_policy_tests" \
        '$1 != issuer && $1 != tests && $1 != policy_mod && $1 != observation && $1 != observation_tests && $1 != "" { found = 1 } END { exit found }'; then
      :
    else
      guard_fail "$tag" "Generic G0 policy caller escaped caller-zero boundary: $policy_ref"
    fi
  done

  for direct_file in \
    "$direct_accum_observation_source" \
    "$direct_accum_observation_adapter" \
    "$direct_accum_observation_policy" \
    "$direct_accum_observation_tests"; do
    local direct_lines
    direct_lines="$(wc -l < "$direct_file" | tr -d '[:space:]')"
    if (( direct_lines >= 800 )); then
      guard_fail "$tag" "DirectAccum S1 observation file exceeds boundary: ${direct_file#"$root_dir/"} lines=$direct_lines"
    fi
  done
  for nested_file in "$nested_source_dto" "$nested_observation_source" \
    "$nested_observation_adapter" "$nested_observation_policy" "$nested_observation_tests"; do
    nested_lines="$(wc -l < "$nested_file" | tr -d '[:space:]')"
    if (( nested_lines >= 800 )); then
      guard_fail "$tag" "Nested S1 observation file exceeds boundary: ${nested_file#"$root_dir/"} lines=$nested_lines"
    fi
  done
  for forbidden in ASTNode MirBuilder ValueId BasicBlockId LoopRouteId Retry \
    'crate::mir::builder' 'loop_recipe_contract' 'route_loop(' \
    'try_cf_loop_joinir(' 'VerifiedLoopPolicyWinnerV1'; do
    if rg -n -F "$forbidden" "$direct_accum_observation_source" "$direct_accum_observation_policy" >/dev/null; then
      guard_fail "$tag" "DirectAccum S1 observation crossed forbidden authority: $forbidden"
    fi
  done
  for required in \
    'DirectAccumSourceAttemptOutcomeV1' \
    'DirectAccumSourceIdentityV1' \
    'DirectAccumObservationCoverageV1' \
    'VerifiedDirectAccumSourceAttemptV1'; do
    rg -n -F "$required" "$direct_accum_observation_source" >/dev/null || \
      guard_fail "$tag" "DirectAccum S1 neutral source-attempt anchor missing: $required"
  done
  for required in \
    'DirectAccumFamilyObservationV1' \
    'issue_direct_accum_family_observation_v1(' \
    'DirectAccumObservationContextV1' \
    'DirectAccumObservationDeclineV1' \
    'DirectAccumObservationUnresolvedV1' \
    'DirectAccumObservationRejectV1'; do
    rg -n -F "$required" "$direct_accum_observation_policy" >/dev/null || \
      guard_fail "$tag" "DirectAccum S1 policy observer anchor missing: $required"
  done
  if [[ "$(rg -o -F '#[test]' "$direct_accum_observation_tests" | wc -l | tr -d '[:space:]')" != "7" ]]; then
    guard_fail "$tag" "DirectAccum S1 focused test count drift"
  fi
  rg -n -F '#![cfg(test)]' "$direct_accum_observation_adapter" >/dev/null || \
    guard_fail "$tag" "DirectAccum S1 compiler adapter must remain test-only"
  for direct_ref in \
    'issue_direct_accum_family_observation_v1(' \
    'VerifiedDirectAccumFamilyCandidateV1'; do
    if rg -n -F "$direct_ref" "$root_dir/src" --glob '*.rs' |
      awk -F: -v policy="$direct_accum_observation_policy" \
        -v tests="$direct_accum_observation_tests" \
        -v admission_tests="$family_admission_tests" \
        -v selector_source="$family_selector_source" -v selector_tests="$family_selector_tests" \
        -v policy_mod="$generic_g0_policy_mod" \
        '$1 != policy && $1 != tests && $1 != admission_tests && $1 != selector_source && $1 != selector_tests && $1 != policy_mod && $1 != "" { found = 1 } END { exit found }'; then
      :
    else
      guard_fail "$tag" "DirectAccum S1 observer caller escaped caller-zero boundary: $direct_ref"
    fi
  done
  for direct_ref in 'issue_direct_accum_source_attempt_for_test('; do
    if rg -n -F "$direct_ref" "$root_dir/src" --glob '*.rs' |
      awk -F: -v adapter="$direct_accum_observation_adapter" \
        -v tests="$direct_accum_observation_tests" \
        -v admission_tests="$family_admission_tests" \
        -v selector_source="$family_selector_source" -v selector_tests="$family_selector_tests" \
        '$1 != adapter && $1 != tests && $1 != admission_tests && $1 != selector_source && $1 != selector_tests && $1 != "" { found = 1 } END { exit found }'; then
      :
    else
      guard_fail "$tag" "DirectAccum S1 source adapter escaped test-only caller boundary: $direct_ref"
    fi
  done
  for direct_ref in \
    'DirectAccumSourceIdentityV1::new(' \
    'VerifiedDirectAccumSourceAttemptV1::new('; do
    if rg -n -F "$direct_ref" "$root_dir/src" --glob '*.rs' |
      awk -F: -v adapter="$direct_accum_observation_adapter" \
        -v tests="$direct_accum_observation_tests" \
        -v admission_tests="$family_admission_tests" \
        -v selector_source="$family_selector_source" -v selector_tests="$family_selector_tests" \
        '$1 != adapter && $1 != tests && $1 != admission_tests && $1 != selector_source && $1 != selector_tests && $1 != "" { found = 1 } END { exit found }'; then
      :
    else
      guard_fail "$tag" "DirectAccum S1 sealed constructor escaped source/test boundary: $direct_ref"
    fi
  done
  for forbidden in ASTNode MirBuilder ValueId BasicBlockId LoopRouteId Retry \
    'crate::mir::builder' 'loop_recipe_contract' 'route_loop(' 'try_cf_loop_joinir(' \
    'VerifiedLoopPolicyWinnerV1'; do
    if rg -n -F "$forbidden" "$nested_source_dto" "$nested_observation_source" \
      "$nested_observation_policy" >/dev/null; then
      guard_fail "$tag" "Nested S1 observation crossed forbidden authority: $forbidden"
    fi
  done
  for required in VerifiedNestedLoopSourceProjectionV1 VerifiedNestedPredicateSourceAttemptV1 \
    NestedPredicateSourceAttemptOutcomeV1 NestedPredicateObservationContextV1 \
    VerifiedNestedPredicateFamilyCandidateV1 'issue_nested_predicate_family_observation_v1('; do
    rg -n -F "$required" "$nested_source_dto" "$nested_observation_source" \
      "$nested_observation_policy" >/dev/null ||
      guard_fail "$tag" "Nested S1 observation anchor missing: $required"
  done
  [[ "$(rg -o -F '#[test]' "$nested_observation_tests" | wc -l | tr -d '[:space:]')" == "7" ]] ||
    guard_fail "$tag" "Nested S1 focused test count drift"
  rg -n -F '#![cfg(test)]' "$nested_observation_adapter" >/dev/null ||
    guard_fail "$tag" "Nested S1 compiler adapter must remain test-only"
  for nested_ref in 'issue_nested_predicate_family_observation_v1(' \
    'VerifiedNestedPredicateFamilyCandidateV1'; do
    if rg -n -F "$nested_ref" "$root_dir/src" --glob '*.rs' |
      awk -F: -v policy="$nested_observation_policy" -v tests="$nested_observation_tests" \
        -v admission_tests="$family_admission_tests" \
        -v selector_source="$family_selector_source" -v selector_tests="$family_selector_tests" \
        -v policy_mod="$root_dir/src/mir/loop_route_policy/mod.rs" \
        '$1 != policy && $1 != tests && $1 != admission_tests && $1 != selector_source && $1 != selector_tests && $1 != policy_mod && $1 != "" { found = 1 } END { exit found }'; then
      :
    else
      guard_fail "$tag" "Nested S1 policy observer escaped caller-zero boundary: $nested_ref"
    fi
  done
  if rg -n -F 'issue_nested_predicate_source_attempt_for_test(' "$root_dir/src" --glob '*.rs' |
    awk -F: -v adapter="$nested_observation_adapter" -v tests="$nested_observation_tests" \
      -v selector_source="$family_selector_source" -v selector_tests="$family_selector_tests" \
      '$1 != adapter && $1 != tests && $1 != selector_source && $1 != selector_tests && $1 != "" { found = 1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "Nested S1 source adapter escaped caller-zero boundary"
  fi

  for forbidden in \
    'compiler.builder.try_cf_loop_joinir' \
    'compiler.builder.route_loop' \
    'self.builder.try_cf_loop_joinir' \
    'self.builder.route_loop'
  do
    if rg -n -F "$forbidden" "$root_dir/src" --glob '*.rs' >/dev/null; then
      guard_fail "$tag" "direct live Builder Loop edge returned: $forbidden"
    fi
  done

  if rg -n -F 'CanonicalFirstFamilyPlanV1::DirectAccum' "$root_dir/src" --glob '*.rs' >/dev/null || \
     rg -n -F 'ExactCanonicalPreflightPlanV1::DirectAccum' "$root_dir/src" --glob '*.rs' >/dev/null; then
    guard_fail "$tag" "DirectAccum escaped the canonical Loop family envelope"
  fi
  for required in \
    'CanonicalLoopFamilyPlanV1' \
    'Loop(CanonicalLoopFamilyPlanV1::DirectAccum' \
    'CanonicalFirstFamilyPlanV1::Loop' \
    'ExactCanonicalPreflightPlanV1::Loop'
  do
    rg -n -F "$required" "$capability" "$first_family_plan" "$source_bound_plan" "$canonical" >/dev/null || \
      guard_fail "$tag" "canonical Loop family envelope anchor missing: $required"
  done

  for required in \
    'VerifiedResolvedLoopSourceForestV1' \
    'resolved_loop_source_forest' \
    'SkippedIntermediateLoop' \
    'UnsupportedAncestry'
  do
    rg -n -F "$required" "$loop_region" >/dev/null || \
      guard_fail "$tag" "Nested source-forest anchor missing: $required"
  done
  if rg -n -F 'resolved_loop_source_forest(' "$root_dir/src" --glob '*.rs' \
      | awk -F: -v generic_g0_projection="$generic_g0_projection" \
          '$1 != generic_g0_projection && $1 !~ /resolved_semantics\/loop_region\.rs$/ && $1 !~ /compiler\/nested_predicate_profile\.rs$/ && $1 !~ /compiler\/nested_predicate_projection\.rs$/ && $1 !~ /_tests?\.rs$/ && $1 !~ /\/tests\.rs$/ { found = 1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "Nested source forest escaped its caller-zero resolver boundary"
  fi

  if [[ ! -d "$generic_g0_source_type_dir" ]]; then
    guard_fail "$tag" "Generic G0 source-type semantic directory is missing"
  fi
  while IFS= read -r semantic_file; do
    local semantic_lines
    semantic_lines="$(wc -l < "$semantic_file" | tr -d '[:space:]')"
    if (( semantic_lines >= 800 )); then
      guard_fail "$tag" "Generic G0 source-type semantic file exceeds boundary: ${semantic_file#"$root_dir/"} lines=$semantic_lines"
    fi
    for forbidden in 'ASTNode' 'FunctionSourceViewV1' 'Located' 'MirBuilder' \
      'ValueId' 'BasicBlockId' 'Recipe' 'Retry' 'LoopRouteId'
    do
      if rg -n -F "$forbidden" "$semantic_file" >/dev/null; then
        guard_fail "$tag" "Generic G0 source-type issuer imported forbidden authority: ${semantic_file#"$root_dir/"} symbol=$forbidden"
      fi
    done
  done < <(find "$generic_g0_source_type_dir" -type f -name '*.rs' -print | sort)
  for source_type_ref in \
    'issue_generic_g0_source_type_inventory_v1(' \
    'issue_generic_g0_source_type_bundle_v1('
  do
    if rg -n -F "$source_type_ref" "$root_dir/src" --glob '*.rs' \
            | awk -F: -v issuer="$generic_g0_source_type" \
            -v projection="$generic_g0_projection" \
            -v handoff="$generic_g0_handoff" \
            -v tests="$generic_g0_projection_tests" \
            -v observation_adapter="$root_dir/src/mir/compiler/generic_g0_observation.rs" \
            -v observation_tests="$root_dir/src/mir/compiler/generic_g0_observation_tests.rs" \
            -v numeric_tests="$generic_g0_numeric_projection_tests" \
            -v policy_tests="$generic_g0_policy_tests" \
            '$1 != issuer && $1 != projection && $1 != handoff && $1 != tests && $1 != observation_adapter && $1 != observation_tests && $1 != numeric_tests && $1 != policy_tests && $1 != "" { found = 1 } END { exit found }'; then
      :
    else
      guard_fail "$tag" "Generic G0 source-type issuer escaped its caller-zero boundary: $source_type_ref"
    fi
  done
  for generic_file in "$generic_g0_projection" "$generic_g0_projection_tests"; do
    local generic_lines
    generic_lines="$(wc -l < "$generic_file" | tr -d '[:space:]')"
    if (( generic_lines >= 800 )); then
      guard_fail "$tag" "Generic G0 projection file exceeds boundary: ${generic_file#"$root_dir/"} lines=$generic_lines"
    fi
  done
  if [[ ! -d "$generic_g0_numeric_dir" ]]; then
    guard_fail "$tag" "Generic G0 numeric semantic directory is missing"
  fi
  while IFS= read -r numeric_file; do
    local numeric_lines
    numeric_lines="$(wc -l < "$numeric_file" | tr -d '[:space:]')"
    if (( numeric_lines >= 800 )); then
      guard_fail "$tag" "Generic G0 numeric file exceeds boundary: ${numeric_file#"$root_dir/"} lines=$numeric_lines"
    fi
    for forbidden in 'ASTNode' 'FunctionSourceView' 'Compiler' 'Builder' \
      'Recipe' 'MirBuilder' 'ValueId' 'BasicBlockId' 'LoopRouteId' 'Box::leak' 'Retry'
    do
      if rg -n -F "$forbidden" "$numeric_file" >/dev/null; then
        guard_fail "$tag" "Generic G0 numeric issuer imported forbidden authority: ${numeric_file#"$root_dir/"} symbol=$forbidden"
      fi
    done
  done < <(find "$generic_g0_numeric_dir" -type f -name '*.rs' -print | sort)
  for numeric_file in "$generic_g0_numeric_adapter" "$generic_g0_numeric_projection_tests"; do
    local numeric_adapter_lines
    numeric_adapter_lines="$(wc -l < "$numeric_file" | tr -d '[:space:]')"
    if (( numeric_adapter_lines >= 800 )); then
      guard_fail "$tag" "Generic G0 numeric adapter/test exceeds boundary: ${numeric_file#"$root_dir/"} lines=$numeric_adapter_lines"
    fi
  done
  for numeric_ref in \
    'issue_generic_g0_numeric_fact_lease_v1(' \
    'issue_generic_g0_typed_source_bundle_v1('
  do
    if [[ "$numeric_ref" == issue_generic_g0_numeric_fact_lease_v1* ]]; then
      if rg -n -F "$numeric_ref" "$root_dir/src" --glob '*.rs' \
          | awk -F: -v issuer="$generic_g0_numeric" \
              -v adapter="$generic_g0_numeric_adapter" \
              -v handoff="$generic_g0_handoff" \
              -v tests="$generic_g0_numeric_tests" \
              -v projection_tests="$generic_g0_numeric_projection_tests" \
              '$1 != issuer && $1 != adapter && $1 != tests && $1 != projection_tests && $1 != "" { found = 1 } END { exit found }'; then
        :
      else
        guard_fail "$tag" "Generic G0 numeric issuer escaped caller-zero boundary: $numeric_ref"
      fi
    else
      if rg -n -F "$numeric_ref" "$root_dir/src" --glob '*.rs' \
          | awk -F: -v adapter="$generic_g0_numeric_adapter" \
              -v handoff="$generic_g0_handoff" \
              -v observation_adapter="$root_dir/src/mir/compiler/generic_g0_observation.rs" \
              -v tests="$generic_g0_numeric_projection_tests" \
              -v policy_tests="$generic_g0_policy_tests" \
              -v observation_tests="$root_dir/src/mir/compiler/generic_g0_observation_tests.rs" \
              '$1 != adapter && $1 != handoff && $1 != observation_adapter && $1 != tests && $1 != policy_tests && $1 != observation_tests && $1 != "" { found = 1 } END { exit found }'; then
        :
      else
        guard_fail "$tag" "Generic G0 numeric adapter escaped caller-zero boundary: $numeric_ref"
      fi
    fi
  done
  if [[ -e "$legacy_join_sig" ]]; then
    guard_fail "$tag" "JoinSig legacy flat module returned: ${legacy_join_sig#"$root_dir/"}"
  fi
  if [[ ! -d "$join_sig_dir" ]]; then
    guard_fail "$tag" "JoinSig split directory is missing"
  fi
  while IFS= read -r join_sig_file; do
    local join_sig_lines
    join_sig_lines="$(wc -l < "$join_sig_file" | tr -d '[:space:]')"
    if (( join_sig_lines >= 800 )); then
      guard_fail "$tag" "JoinSig module exceeds boundary: ${join_sig_file#"$root_dir/"} lines=$join_sig_lines"
    fi
    for forbidden in 'ASTNode' 'MirBuilder' 'CorePlan' 'ValueId' \
      'BasicBlockId' 'Retry' 'LoopRouteId'
    do
      if rg -n -F "$forbidden" "$join_sig_file" >/dev/null; then
        guard_fail "$tag" "JoinSig module imported physical/source authority: ${join_sig_file#"$root_dir/"} symbol=$forbidden"
      fi
    done
  done < <(find "$join_sig_dir" -type f -name '*.rs' -print | sort)
  for required in \
    'mod model;' 'mod port;' 'mod visibility;' 'mod flow;' \
    'pub(crate) use model::' 'pub(crate) use flow::LoopJoinSigElaboratorV1'
  do
    rg -n -F "$required" "$join_sig_facade" >/dev/null || \
      guard_fail "$tag" "JoinSig facade anchor missing: $required"
  done
  for forbidden in 'ASTNode' 'MirBuilder' 'LoopRouteId' 'ValueId' 'BasicBlockId' 'Retry'
  do
    if rg -n -F "$forbidden" "$loop_region" >/dev/null; then
      guard_fail "$tag" "Nested source forest imported physical/route authority: $forbidden"
    fi
  done

  for required in \
    'VerifiedLoopSourceForestBindingV1' \
    'bind_resolved_loop_source_forest_v1(' \
    'into_source_binding(' \
    'LoopSourceForestBindingRejectV1'
  do
    rg -n -F "$required" "$source_adapter" >/dev/null || \
      guard_fail "$tag" "Nested source-binding adapter anchor missing: $required"
  done
  # Generic G0 S4 is a test-only Recipe producer and is the second explicit
  # caller-zero consumer of the shared forest-binding adapter.
  if rg -n -F 'bind_resolved_loop_source_forest_v1(' "$root_dir/src" --glob '*.rs' \
      | awk -F: '$1 !~ /loop_structural_facts\/resolved_source_adapter\.rs$/ && $1 !~ /compiler\/nested_predicate_projection\.rs$/ && $1 !~ /loop_recipe_contract\/generic_g0\/producer\.rs$/ && $1 !~ /_tests?\.rs$/ && $1 !~ /\/tests\.rs$/ { found = 1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "Nested source-binding adapter escaped its caller-zero boundary"
  fi

  for required in \
    'VerifiedNestedLoopSourceProjectionV1' \
    'issue_nested_predicate_source_projection_v1' \
    'NestedObservedRecurrenceOwnerV1' \
    'resolved_loop_source_forest' \
    'bind_resolved_loop_source_forest_v1('
  do
    rg -n -F "$required" "$nested_source_projection" >/dev/null || \
      guard_fail "$tag" "Nested Predicate source projection anchor missing: $required"
  done
  for forbidden in 'MirBuilder' 'LoopRouteId' 'ValueId' 'BasicBlockId' 'Retry' 'route_loop('
  do
    if rg -n -F "$forbidden" "$nested_source_projection" >/dev/null; then
      guard_fail "$tag" "Nested Predicate source projection imported physical/route authority: $forbidden"
    fi
  done
  for required in \
    'VerifiedNestedPredicateRecipeProductV1' \
    'produce_nested_predicate_recipe_v1' \
    'verify_source_bound_recipe_v1' \
    'LoopJoinSigElaboratorV1::elaborate'
  do
    rg -n -F "$required" "$nested_recipe_producer" >/dev/null || \
      guard_fail "$tag" "Nested Predicate Recipe producer anchor missing: $required"
  done
  for required in \
    'VerifiedNestedPhysicalSourceHandoffV1' \
    'VerifiedNestedPhysicalTopologyV1' \
    'VerifiedNestedPhysicalEmissionInputV1' \
    'issue_nested_predicate_physical_emission_input_v1' \
    'NestedParentResumePortV1' \
    'NestedLogicalExpansionV1' \
    'VerifiedNestedTopologyPredecessorSealV1' \
    'NestedCarrierDestinationV1' \
    'into_topology_input' \
    'VerifiedNestedPhysicalBlockProjectionV1' \
    'VerifiedNestedPhysicalCandidateInputV1'
  do
    rg -n -F "$required" "$nested_source_handoff" "$nested_topology" "$nested_recipe_producer" "$nested_physical_input" >/dev/null || \
      guard_fail "$tag" "Nested Predicate topology handoff/topology anchor missing: $required"
  done
  for forbidden in \
    'ASTNode' 'MirBuilder' 'ValueId' 'BasicBlockId' 'PhiTxn' \
    'BindingSsaBuilderV1' 'CanonicalSsaFunctionSessionV2' 'Retry' 'route_loop('
  do
    if rg -n -F "$forbidden" "$nested_source_handoff" "$nested_topology" >/dev/null; then
      guard_fail "$tag" "Nested Predicate topology crossed physical/route authority: $forbidden"
    fi
  done
  for forbidden in \
    'ASTNode' 'MirBuilder' 'PhiTxn' 'BindingSsaBuilderV1' \
    'CanonicalSsaFunctionSessionV2' 'Retry' 'route_loop('
  do
    if rg -n -F "$forbidden" "$nested_physical_input" >/dev/null; then
      guard_fail "$tag" "Nested Predicate physical input crossed emission authority: $forbidden"
    fi
  done
  local nested_topology_refs
  nested_topology_refs="$(rg -n -F 'issue_nested_predicate_physical_emission_input_v1(' "$root_dir/src" --glob '*.rs' || true)"
  if [[ "$(printf '%s\n' "$nested_topology_refs" \
      | awk -F: '$1 !~ /compiler\/nested_predicate_topology\.rs$/ && $1 !~ /compiler\/nested_predicate_topology_tests\.rs$/ && $1 !~ /compiler\/nested_predicate_profile\.rs$/ && $1 !~ /compiler\/nested_predicate_physical_input_tests\.rs$/ && $1 != "" { count += 1 } END { print count + 0 }')" != "0" ]]; then
    guard_fail "$tag" "Nested Predicate topology issuer escaped caller-zero boundary"
  fi
  for forbidden in \
    'ASTNode' 'FunctionSourceViewV1' 'Located' 'MirBuilder' 'ValueId' \
    'BasicBlockId' 'CanonicalCfgSessionV1' 'BindingSsaBuilderV1' 'PhiTxn' \
    'Retry' 'route_loop('
  do
    if rg -n -F "$forbidden" "$nested_recipe_producer" >/dev/null; then
      guard_fail "$tag" "Nested Predicate Recipe producer imported forbidden authority: $forbidden"
    fi
  done
  local nested_producer_refs
  nested_producer_refs="$(rg -n -F 'produce_nested_predicate_recipe_v1(' "$root_dir/src" --glob '*.rs' || true)"
  if [[ "$(printf '%s\n' "$nested_producer_refs" \
      | awk -F: '$1 !~ /compiler\/nested_predicate_producer\.rs$/ && $1 !~ /compiler\/nested_predicate_producer_tests\.rs$/ && $1 !~ /compiler\/nested_predicate_topology_tests\.rs$/ && $1 !~ /compiler\/nested_predicate_profile\.rs$/ && $1 !~ /compiler\/nested_predicate_physical_input_tests\.rs$/ && $1 !~ /compiler\/nested_predicate_effect_plan_tests\.rs$/ && $1 !~ /resolved_lowering\/nested_predicate_effect_adapter_tests\.rs$/ && $1 != "" { count += 1 } END { print count + 0 }')" != "0" ]]; then
    guard_fail "$tag" "Nested Predicate Recipe producer escaped caller-zero boundary"
  fi
  for required in \
    'VerifiedNestedPrefixInputV1' \
    'VerifiedNestedBindingEffectPlanV1' \
    'pub(crate) const ALL: [Self; 9]' \
    'NestedScopeRetirementBoundaryV1::RootLoopRegionExit' \
    'issue_nested_binding_execution_claims_v1('
  do
    rg -n -F "$required" "$nested_effect_plan" >/dev/null || \
      guard_fail "$tag" "Nested Predicate resolver effect-plan anchor missing: $required"
  done
  for forbidden in \
    'ASTNode' 'MirBuilder' 'ValueId' 'BasicBlockId' 'PhiTxn' \
    'BindingSsaBuilderV1' 'CanonicalSsaFunctionSessionV2' 'Retry' 'route_loop('
  do
    if rg -n -F "$forbidden" "$nested_effect_plan" >/dev/null; then
      guard_fail "$tag" "Nested Predicate resolver effect plan crossed physical/route authority: $forbidden"
    fi
  done
  local nested_effect_plan_refs
  nested_effect_plan_refs="$(rg -l -F 'issue_nested_binding_execution_claims_v1(' "$root_dir/src" --glob '*.rs' || true)"
  if printf '%s\n' "$nested_effect_plan_refs" | awk -v issuer="$nested_effect_plan" -v tests="$nested_effect_plan_tests" -v adapter="$nested_effect_adapter_tests" -v profile="$root_dir/src/mir/compiler/nested_predicate_profile.rs" \
      '$0 != issuer && $0 != tests && $0 != adapter && $0 != profile && $0 != "" { found = 1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "Nested Predicate resolver effect-plan issuer escaped caller-zero boundary"
  fi
  for file in "$nested_effect_plan" "$nested_effect_plan_tests"; do
    local effect_plan_lines
    effect_plan_lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( effect_plan_lines >= 800 )); then
      guard_fail "$tag" "Nested Predicate effect-plan file exceeds boundary: ${file#"$root_dir/"} lines=$effect_plan_lines"
    fi
  done
  for forbidden in 'ASTNode' 'MirBuilder' 'LoopRouteId' 'ValueId' 'BasicBlockId' 'Retry'
  do
    if rg -n -F "$forbidden" "$source_adapter" >/dev/null; then
      guard_fail "$tag" "Nested source-binding adapter imported physical/route authority: $forbidden"
    fi
  done
  for required in '#![cfg(test)]' 'NestedEffectAdapter' 'publish_initialized_prefix' 'consume_role' 'finish(&self)' \
    'enter_scope_region' 'close_scope_region_success' 'ScopeKindV1::LoopBody' 'Root.After' ; do
    rg -n -F "$required" "$nested_effect_adapter_tests" >/dev/null || \
      guard_fail "$tag" "Nested Predicate effect adapter test anchor missing: $required"
  done
  if rg -n -F 'route_loop(' "$nested_effect_adapter_tests" >/dev/null; then
    guard_fail "$tag" "Nested Predicate effect adapter test acquired a route caller"
  fi

  for required in \
    '#![cfg(test)]' \
    'VerifiedLoopRecipeProducerFacadeV1' \
    'nested_always_witness_binds_source_without_production_caller' \
    'LoopRouteId::NestedLoopMinimal'
  do
    rg -n -F "$required" "$producer_facade" >/dev/null || \
      guard_fail "$tag" "Nested Always caller-zero facade anchor missing: $required"
  done
  if rg -n -F 'VerifiedLoopRecipeProducerFacadeV1::consume(' "$root_dir/src" --glob '*.rs' \
      | awk -F: '$1 !~ /loop_recipe_producer_facade_tests.rs$/ { found = 1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "Nested Always Recipe facade escaped its caller-zero test boundary"
  fi

  if ! rg -n -F 'RawLocatedScalarStmtV1::Loop { .. }' "$raw_recipe" >/dev/null || \
     ! rg -n -F 'RawUnsupportedBodyStatementKindV1::Loop' "$raw_recipe" >/dev/null || \
     ! rg -n -F 'unsupported_shape_fails_before_builder_effects_without_legacy_retry' "$canonical_input" >/dev/null || \
     ! rg -n -F 'ASTNode::Loop {' "$canonical_input" >/dev/null; then
    guard_fail "$tag" "Raw/canonical Loop pre-effect rejection anchors drift"
  fi

  for required in \
    'loop_effect_then_later_failure_discards_candidate_and_reuses_live_compiler' \
    'reset_loop_physical_effect_probe' \
    'take_loop_physical_effect_probe' \
    'loop_candidate_test_fingerprint' \
    'Undefined variable: missing' \
    'loop-failure.hako' \
    'loop-reused.hako'
  do
    rg -n -F "$required" "$m1_test" >/dev/null || \
      guard_fail "$tag" "M1 candidate-abort proof anchor missing: $required"
  done

  for required in \
    'production_failure_after_prepare_discards_candidate_and_reuses_compiler' \
    'successful_direct_accum_public_result_uses_final_barrier_contract' \
    'compile_direct_accum_source_bound_with_prepared_failure_for_test'
  do
    rg -n -F "$required" "$hardening_test" "$direct_accum_cutover" >/dev/null || \
      guard_fail "$tag" "resolved DirectAccum hardening proof anchor missing: $required"
  done

  rg -n -F 'project_canonical_verification_result' "$external_commit" >/dev/null || \
    guard_fail "$tag" "canonical final-barrier projection anchor missing"
  if rg -n -F 'pre_transform.map_err' "$external_commit" >/dev/null; then
    guard_fail "$tag" "canonical publication still projects pre_transform directly"
  fi
}

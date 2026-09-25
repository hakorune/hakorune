#!/usr/bin/env bash
# Shared-guard helper: the logical Loop product must remain pre-effect only.
guard_joinir_logical_demand_contract() {
  local root_dir="$1"
  local tag="$2"
  local route_id="$root_dir/src/mir/loop_recipe_contract/route_id.rs"
  local portable_recipe_dir="$root_dir/src/mir/loop_recipe_contract"
  local producer_id="$portable_recipe_dir/producer_id.rs"
  local loop_structural_facts_dir="$root_dir/src/mir/loop_structural_facts"
  local direct_accum_recipe_producer="$portable_recipe_dir/direct_accum_producer.rs"
  local variable_accum_recipe_producer="$portable_recipe_dir/variable_accum_recurrence_producer.rs";
  # The compiler-owned DirectAccum profile is the sole disconnected issuer of
  # the portable producer. It is not a downstream production consumer; the
  # guard must distinguish this issuer from an accidental route caller.
  local direct_accum_issuer="$root_dir/src/mir/compiler/direct_accum_profile.rs"
  local direct_accum_capability="$root_dir/src/mir/compiler/direct_accum_capability.rs"
  local direct_accum_projection="$root_dir/src/mir/compiler/direct_accum_projection.rs"
  local direct_accum_observation_adapter="$root_dir/src/mir/compiler/direct_accum_observation.rs"
  local nested_observation_source="$root_dir/src/mir/loop_structural_facts/nested_predicate_observation.rs"
  local nested_observation_adapter="$root_dir/src/mir/compiler/nested_predicate_observation.rs"
  local nested_observation_policy="$root_dir/src/mir/loop_route_policy/nested_predicate_observation.rs"
  local nested_observation_tests="$root_dir/src/mir/loop_route_policy/nested_predicate_observation_tests.rs"
  local family_selector_tests="$root_dir/src/mir/loop_route_policy/family_selector_tests.rs"
  local loop_true_source_projection="$root_dir/src/mir/compiler/loop_true_break_continue_projection.rs"
  local loop_true_observation_adapter="$root_dir/src/mir/compiler/loop_true_break_continue_observation.rs"
  local loop_cond_source_projection="$root_dir/src/mir/compiler/loop_cond_break_continue_projection.rs"
  local loop_cond_observation_adapter="$root_dir/src/mir/compiler/loop_cond_break_continue_observation.rs"
  local generic_g0_observation_adapter="$root_dir/src/mir/compiler/generic_g0_observation.rs"
  local loop_route_policy_dir="$root_dir/src/mir/loop_route_policy"
  local route_registry_dir="$root_dir/src/mir/builder/control_flow/joinir/route_entry/registry"
  local generic_resolved_test_prefix="$route_registry_dir/generic_resolved_carrier_"
  local loop_phi_materializer="$root_dir/src/mir/builder/control_flow/plan/loop_phi_materializer.rs"
  local loop_phi_materializer_tests="$root_dir/src/mir/builder/control_flow/plan/loop_phi_materializer_tests.rs"
  local loop_accum_physical_role_tests="$root_dir/src/mir/builder/control_flow/plan/loop_accum_physical_role_plan_tests.rs"
  local loop_accum_binding_ssa_tests="$root_dir/src/mir/builder/control_flow/plan/loop_accum_binding_ssa_session_tests.rs"
  local loop_accum_emitter_tests="$root_dir/src/mir/builder/control_flow/plan/loop_accum_binding_ssa_emitter_tests.rs"
  local loop_recipe_producer_tests="$root_dir/src/mir/builder/control_flow/plan/loop_recipe_producer_facade_tests.rs"
  local nested_predicate_producer="$root_dir/src/mir/compiler/nested_predicate_producer.rs"
  local nested_predicate_producer_tests="$root_dir/src/mir/compiler/nested_predicate_producer_tests.rs";
  local loop_physical_edge_path="$root_dir/src/mir/builder/control_flow/plan/loop_physical_edge_path.rs"
  # The accepted Dynamic physical-input co-seal is the only named consumer
  # outside the portable contract subtree. Its two fixture files are test
  # evidence only; do not widen this to the whole Dynamic compiler subtree.
  local dynamic_physical_input="$root_dir/src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/physical_input.rs"
  local dynamic_semantic_tests="$root_dir/src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/tests.rs"
  local dynamic_recipe_tests="$root_dir/src/mir/compiler/dynamic_full_body_recipe/tests.rs"
  local dynamic_recipe_mod="$root_dir/src/mir/compiler/dynamic_full_body_recipe/mod.rs"
  local projection="$root_dir/src/mir/builder/control_flow/facts/stmt_view.rs"
  local simple_while="$root_dir/src/mir/builder/control_flow/plan/facts/loop_simple_while_facts.rs"
  local accum_const="$root_dir/src/mir/builder/control_flow/plan/facts/accum_const_loop_facts.rs"
  local loop_break_facts="$root_dir/src/mir/builder/control_flow/plan/loop_break/facts/types.rs"
  local if_phi_join="$root_dir/src/mir/builder/control_flow/facts/if_phi_join_facts.rs"
  local files=(
    "$route_id"
    "$projection"
    "$simple_while"
    "$accum_const"
    "$loop_break_facts"
    "$if_phi_join"
  )
  local file lines

  guard_require_files "$tag" "${files[@]}"
  guard_require_files "$tag" \
    "$loop_phi_materializer" "$loop_phi_materializer_tests" \
    "$loop_accum_physical_role_tests" \
    "$loop_accum_binding_ssa_tests" "$loop_accum_emitter_tests" \
    "$loop_physical_edge_path" "$direct_accum_issuer" "$direct_accum_capability" "$variable_accum_recipe_producer" \
    "$direct_accum_projection" "$direct_accum_observation_adapter" \
    "$loop_true_source_projection" "$loop_true_observation_adapter" \
    "$loop_cond_source_projection" "$loop_cond_observation_adapter" "$nested_observation_source" \
    "$nested_observation_adapter" "$nested_observation_policy" "$nested_observation_tests" \
    "$generic_g0_observation_adapter"
  if ! rg -q '^#!\[cfg\(test\)\]' "$loop_accum_physical_role_tests"; then
    guard_fail "$tag" "physical role-plan observer must remain cfg(test)-only"
  fi
  if ! rg -q '^#!\[cfg\(test\)\]' "$loop_accum_binding_ssa_tests"; then
    guard_fail "$tag" "Binding-SSA session proof must remain cfg(test)-only"
  fi
  if ! rg -q '^#!\[cfg\(test\)\]' "$loop_accum_emitter_tests"; then
    guard_fail "$tag" "Binding-SSA emitter proof must remain cfg(test)-only"
  fi
  for binding_ssa_file in "$loop_accum_binding_ssa_tests" "$loop_accum_emitter_tests"; do
    if rg -n \
      'LoopPhiMaterializer|materialize_loop_phis|insert_phi_at_head|update_phi_instruction|CorePlan|PlanLowerer|RouteAttemptOutcome|Retry' \
      "$binding_ssa_file" >/dev/null; then
      guard_fail "$tag" "Binding-SSA proof bypassed canonical owner boundary: ${binding_ssa_file#"$root_dir/"}"
    fi
    if rg -n 'Option<' "$binding_ssa_file" | \
      rg -v 'Option<(PhiTxn|CanonicalCfgSessionV1|BindingSsaBuilderV1)' >/dev/null; then
      guard_fail "$tag" "Binding-SSA proof acquired retry/decline Option outside PHI transaction state: ${binding_ssa_file#"$root_dir/"}"
    fi
  done
  local portable_recipe_files=()
  mapfile -t portable_recipe_files < <(find "$portable_recipe_dir" -type f -name '*.rs' | sort)
  guard_require_files "$tag" \
    "$portable_recipe_dir/README.md" \
    "$portable_recipe_dir/schema.rs" \
    "$producer_id" \
    "$portable_recipe_dir/verify.rs" \
    "$portable_recipe_dir/join_sig/mod.rs" \
    "$portable_recipe_dir/normalize.rs"
  if (( ${#portable_recipe_files[@]} == 0 )); then
    guard_fail "$tag" "portable Loop recipe subtree has no Rust contract files"
  fi
  for file in "${portable_recipe_files[@]}"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "file exceeds boundary: ${file#"$root_dir/"} lines=$lines"
    fi
  done
  local portable_production_files=()
  for file in "${portable_recipe_files[@]}"; do
    if [[ "$file" == "$portable_recipe_dir/tests.rs" || "$file" == *_tests.rs ]]; then
      continue
    fi
    portable_production_files+=("$file")
  done
  if rg -n -w \
    'ASTNode|MirBuilder|CorePlan|ValueId|BasicBlockId|MirInstruction|Phi|Frag|RouteAttemptOutcome|RouteFn|ComposeFn' \
    "${portable_production_files[@]}" | rg -v ':[0-9]+:[[:space:]]*//' >/dev/null; then
    guard_fail "$tag" "portable Loop recipe acquired source, physical, or retry authority"
  fi
  if rg -n \
    'mutation_family|LoopMutationFamily|LoopRecipeFamily|legacy_family|opaque.*(emit|command)' \
    "${portable_production_files[@]}" >/dev/null; then
    guard_fail "$tag" "portable Loop recipe acquired legacy family or opaque emission authority"
  fi
  if rg -n -w 'LoopRouteId|producer_route' \
    "$portable_recipe_dir/verify.rs" "$portable_recipe_dir/normalize.rs" >/dev/null; then
    guard_fail "$tag" "portable semantic verifier/normalizer acquired route provenance authority"
  fi
  if rg -n -w 'LoopRouteId|producer_route' \
    "$portable_recipe_dir/schema.rs" \
    "$portable_recipe_dir/direct_accum_producer.rs" \
    "$portable_recipe_dir/loop_true_break_continue_producer.rs" \
    "$nested_predicate_producer" >/dev/null; then
    guard_fail "$tag" "portable schema/producer imported legacy LoopRouteId provenance"
  fi
  if ! rg -n -F 'producer_id: LoopRecipeProducerIdV1' "$portable_recipe_dir/schema.rs" >/dev/null || \
     ! rg -n -F 'LoopRecipeProducerIdV1::DirectAccumV1' "$portable_recipe_dir/direct_accum_producer.rs" >/dev/null || \
     ! rg -n -F 'LoopRecipeProducerIdV1::LoopTrueBreakContinueV1' "$portable_recipe_dir/loop_true_break_continue_producer.rs" >/dev/null || \
     ! rg -n -F 'LoopRecipeProducerIdV1::NestedPredicateV1' "$nested_predicate_producer" >/dev/null; then
    guard_fail "$tag" "portable producer ID issuer anchors drifted"
  fi
  local legacy_wire_refs
  legacy_wire_refs="$({ rg -n -w 'producer_route' \
    "$portable_recipe_dir/fixtures" --glob '*.json' || true; } | wc -l | tr -d '[:space:]')"
  if [[ "$legacy_wire_refs" != "0" ]]; then
    guard_fail "$tag" "portable fixtures still carry legacy producer_route wire"
  fi
  local join_sig_external_files=()
  mapfile -t join_sig_external_files < <(
    { rg -l -w \
        'LoopJoinPortV1|LoopJoinEdgeRoleV1|LoopJoinPayloadV1|LoopJoinEdgeV1|LoopJoinLoopV1|LoopJoinSigV1|VerifiedLoopJoinSigV1|LoopJoinSigElaboratorV1|LoopJoinSigRejectReasonV1' \
        "$root_dir/src/mir" || true; } \
      | awk \
          -v prefix="$portable_recipe_dir/" \
          -v materializer="$loop_phi_materializer" \
          -v materializer_tests="$loop_phi_materializer_tests" \
          -v physical_role_tests="$loop_accum_physical_role_tests" \
          -v binding_ssa_tests="$loop_accum_binding_ssa_tests" \
          -v producer_tests="$loop_recipe_producer_tests" \
          -v nested_producer="$nested_predicate_producer" \
          -v nested_producer_tests="$nested_predicate_producer_tests" \
          -v nested_topology="$root_dir/src/mir/compiler/nested_predicate_topology.rs" \
          -v nested_topology_tests="$root_dir/src/mir/compiler/nested_predicate_topology_tests.rs" \
          -v nested_physical_input="$root_dir/src/mir/compiler/nested_predicate_physical_input.rs" \
          -v nested_physical_input_tests="$root_dir/src/mir/compiler/nested_predicate_physical_input_tests.rs" \
          -v dynamic_physical_input="$dynamic_physical_input" \
          -v dynamic_semantic_tests="$dynamic_semantic_tests" \
          -v dynamic_recipe_tests="$dynamic_recipe_tests" \
          -v callable_recipe_coseal="$root_dir/src/mir/compiler/callable_single_loop_recipe_coseal.rs" \
          -v main0_continue_coseal="$root_dir/src/mir/compiler/main0_continue_recipe_coseal.rs" \
          -v main0_derived_coseal="$root_dir/src/mir/compiler/main0_derived_predicate_recipe_coseal.rs" \
          -v main0_step_coseal="$root_dir/src/mir/compiler/main0_in_body_step_recipe_coseal.rs" \
          -v node_admission="$root_dir/src/mir/compiler/loop_node_physical_admission.rs" \
          -v if_continuation="$root_dir/src/mir/builder/resolved_lowering/common_v2_if_continuation_target.rs" \
          -v physicalizer="$root_dir/src/mir/builder/control_flow/plan/loop_accum_physicalizer.rs" \
          -v physicalizer_session="$root_dir/src/mir/builder/control_flow/plan/loop_accum_physicalizer/session.rs" \
          -v edge_path="$loop_physical_edge_path" \
          'index($0, prefix) != 1 && $0 != materializer && $0 != materializer_tests  && $0 != physical_role_tests && $0 != binding_ssa_tests && $0 != producer_tests && $0 != nested_producer && $0 != nested_producer_tests && $0 != nested_topology && $0 != nested_topology_tests && $0 != nested_physical_input && $0 != nested_physical_input_tests && $0 != dynamic_physical_input && $0 != dynamic_semantic_tests && $0 != dynamic_recipe_tests && $0 != callable_recipe_coseal && $0 != main0_continue_coseal && $0 != main0_derived_coseal && $0 != main0_step_coseal && $0 != node_admission && $0 != if_continuation && $0 != physicalizer && $0 != physicalizer_session && $0 != edge_path'
  )
  if (( ${#join_sig_external_files[@]} != 0 )); then
    guard_fail "$tag" "caller-zero logical JoinSig symbols escaped the contract subtree"
  fi
  if rg -n \
    'ssa::phi_input_materializer|for_pred|define_phi_final|materialize_all_phi_inputs|BindingSsaBuilderV1|compute_predecessors|update_cfg|RouteAttemptOutcome|RouteFn|CorePlan|CanonicalLoopFacts|ASTNode' \
    "$loop_phi_materializer" >/dev/null; then
    guard_fail "$tag" "Loop PHI materializer bypassed the JoinSig/PhiTxn boundary"
  fi
  local loop_phi_external_callers=()
  mapfile -t loop_phi_external_callers < <(
    { rg -l 'materialize_loop_phis\(' "$root_dir/src/mir" || true; } \
      | awk -v materializer="$loop_phi_materializer" \
          -v materializer_tests="$loop_phi_materializer_tests" \
          '$0 != materializer && $0 != materializer_tests'
  )
  if (( ${#loop_phi_external_callers[@]} != 0 )); then
    guard_fail "$tag" "caller-zero Loop PHI materializer acquired a production caller"
  fi
  local loop_structural_fact_files=()
  mapfile -t loop_structural_fact_files < <(
    find "$loop_structural_facts_dir" -maxdepth 1 -name '*.rs' -type f | sort
  )
  guard_require_files "$tag" \
    "$loop_structural_facts_dir/README.md" \
    "$loop_structural_facts_dir/mod.rs" \
    "$loop_structural_facts_dir/resolved_source_adapter.rs"
  if (( ${#loop_structural_fact_files[@]} == 0 )); then
    guard_fail "$tag" "Loop structural facts subtree has no Rust contract files"
  fi
  for nested_file in "$nested_observation_source" "$nested_observation_adapter" \
    "$nested_observation_policy" "$nested_observation_tests"; do
    lines="$(wc -l < "$nested_file" | tr -d '[:space:]')"
    (( lines < 800 )) || guard_fail "$tag" "Nested S1 file exceeds boundary: $nested_file"
  done
  if ! rg -q 'the only compiler-side translation point' "$nested_observation_adapter"; then
    guard_fail "$tag" "Nested S1 adapter lost its sole-compiler-translation-point contract"
  fi
  [[ "$(rg -o -F '#[test]' "$nested_observation_tests" | wc -l | tr -d '[:space:]')" == "7" ]] ||
    guard_fail "$tag" "Nested S1 focused test count drift"
  if rg -n -F 'ASTNode' "$nested_observation_source" "$nested_observation_policy" >/dev/null ||
     rg -n -F 'MirBuilder' "$nested_observation_source" "$nested_observation_policy" >/dev/null ||
     rg -n -F 'loop_recipe_contract' "$nested_observation_source" "$nested_observation_policy" >/dev/null; then
    guard_fail "$tag" "Nested S1 logical observer acquired source/physical authority"
  fi
  if rg -l -F 'issue_nested_predicate_family_observation_v1(' "$root_dir/src/mir" |
    awk -v p="$nested_observation_policy" -v t="$nested_observation_tests" \
      -v at="$root_dir/src/mir/loop_route_policy/family_admission_tests.rs" \
      -v st="$family_selector_tests" \
      -v m="$root_dir/src/mir/loop_route_policy/mod.rs" \
      -v w="$root_dir/src/mir/compiler/loop_node_winner_spine.rs" \
      '$0 != p && $0 != t && $0 != at && $0 != st && $0 != m && $0 != w && $0 != "" { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "Nested S1 policy observer acquired a production caller"
  fi
  if rg -l -F 'VerifiedResolvedLoopSourceV1' "$root_dir/src/mir" |
    awk -v a="$direct_accum_observation_adapter" -v n="$nested_observation_adapter" \
      -v p="$direct_accum_projection" -v l="$loop_true_source_projection" \
      -v t="$loop_true_observation_adapter" -v c="$loop_cond_source_projection" \
      -v d="$loop_cond_observation_adapter" -v g="$generic_g0_observation_adapter" \
      -v q="$root_dir/src/mir/compiler/callable_single_loop_recipe_coseal.rs" \
      -v m="$root_dir/src/mir/compiler/callable_single_loop_source_map.rs" \
      -v s="$root_dir/src/mir/loop_structural_facts/" \
      -v r="$root_dir/src/mir/resolved_semantics/" \
      -v b="$root_dir/src/mir/builder/normal_callable_loop_source_facts/" \
      -v t2="$root_dir/src/mir/loop_recipe_contract/s6c_scan_with_init_tests.rs" \
      -v x2="$root_dir/src/mir/compiler/loop_break_composite_source_projection.rs" \
      -v x3="$root_dir/src/mir/compiler/loop_break_source_projection.rs" \
      -v x4="$root_dir/src/mir/compiler/loop_family_window_probe_tests.rs" \
      -v x5="$root_dir/src/mir/compiler/main0_continue_source_map.rs" \
      -v x6="$root_dir/src/mir/compiler/main0_derived_predicate_source_map.rs" \
      -v x7="$root_dir/src/mir/compiler/main0_in_body_step_source_map.rs" \
      '$0 != a && $0 != n && $0 != p && $0 != l && $0 != t && $0 != c && $0 != d && $0 != g && $0 != q && $0 != m && $0 != t2 && $0 != x2 && $0 != x3 && $0 != x4 && $0 != x5 && $0 != x6 && $0 != x7 && index($0,s) != 1 && index($0,r) != 1 && index($0,b) != 1 { found=1 } END { exit found }'; then
    :
  else
    guard_fail "$tag" "sealed resolved Loop source capability escaped its adapter boundary"
  fi
  local loop_structural_production_files=()
  for file in "${loop_structural_fact_files[@]}"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "file exceeds boundary: ${file#"$root_dir/"} lines=$lines"
    fi
    if [[ "$file" == "$loop_structural_facts_dir/tests.rs" || "$file" == *_tests.rs ]]; then
      continue
    fi
    loop_structural_production_files+=("$file")
  done
  if rg -n -w \
    'ASTNode|MirBuilder|CorePlan|ValueId|BasicBlockId|MirInstruction|Phi|Frag|LoopRouteContext|CanonicalLoopFacts|RouteAttemptOutcome|RouteFn|ComposeFn|LoopRecipeArtifactV1|LoopRouteId|LoopRecipeProvenanceV1|producer_route' \
    "${loop_structural_production_files[@]}" >/dev/null; then
    guard_fail "$tag" "Loop structural source authority acquired artifact, route, retry, AST, or physical authority"
  fi
  local structural_binding_callers
  structural_binding_callers="$(
    { rg -l 'bind_resolved_loop_root_v1\(' "$root_dir/src/mir" || true; } \
      | awk -v prefix="$loop_structural_facts_dir/" -v producer="$direct_accum_recipe_producer" -v variable_producer="$variable_accum_recipe_producer" \
          -v projection="$loop_true_source_projection" \
          -v callable_recipe_coseal="$root_dir/src/mir/compiler/callable_single_loop_recipe_coseal.rs" \
          -v dynamic_recipe_mod="$dynamic_recipe_mod" \
          -v wire_tests_prefix="$root_dir/src/mir/loop_recipe_contract/wire_parity_tests" \
          -v wire_route_tests="$root_dir/src/mir/loop_recipe_contract/wire_route_parity_tests.rs" \
          -v loopcond_issue="$root_dir/src/mir/compiler/loop_cond_break_continue_typed_map_issue.rs" \
          -v main0_continue_coseal="$root_dir/src/mir/compiler/main0_continue_recipe_coseal.rs" \
          -v main0_derived_coseal="$root_dir/src/mir/compiler/main0_derived_predicate_recipe_coseal.rs" \
          -v main0_step_coseal="$root_dir/src/mir/compiler/main0_in_body_step_recipe_coseal.rs" \
          'index($0, prefix) != 1 && index($0, wire_tests_prefix) != 1 && $0 != producer && $0 != variable_producer && $0 != projection && $0 != callable_recipe_coseal && $0 != dynamic_recipe_mod && $0 != wire_route_tests && $0 != loopcond_issue && $0 != main0_continue_coseal && $0 != main0_derived_coseal && $0 != main0_step_coseal' \
      | wc -l \
      | tr -d '[:space:]'
  )"
  if [[ "$structural_binding_callers" != "0" ]]; then
    guard_fail "$tag" "Loop source adapter acquired an unapproved production caller"
  fi
  local direct_accum_production_callers=()
  mapfile -t direct_accum_production_callers < <(
    { rg -l 'produce_direct_accum_recipe_v1\(' "$root_dir/src/mir" || true; } \
      | awk -v producer="$direct_accum_recipe_producer" -v issuer="$direct_accum_issuer" \
          -v spine="$root_dir/src/mir/compiler/loop_node_winner_spine.rs" \
          '$0 != producer && $0 != issuer && $0 != spine && $0 !~ /_tests\.rs$/'
  )
  if (( ${#direct_accum_production_callers[@]} != 0 )); then
    guard_fail "$tag" "Direct Accum Recipe producer acquired a production caller"
  fi
  local direct_accum_issuer_calls
  direct_accum_issuer_calls="$(
    { rg -o 'produce_direct_accum_recipe_v1\(' "$direct_accum_issuer" || true; } \
      | wc -l \
      | tr -d '[:space:]'
  )"
  if [[ "$direct_accum_issuer_calls" != "1" ]]; then
    guard_fail "$tag" "Direct Accum issuer call count drift: count=$direct_accum_issuer_calls expected=1"
  fi
  local direct_accum_source_probe_callers=()
  mapfile -t direct_accum_source_probe_callers < <(
    { rg -l 'probe_direct_accum_source_unit_v1\(' "$root_dir/src/mir" || true; } \
      | awk -v capability="$direct_accum_capability" '$0 != capability && $0 !~ /_tests\.rs$/'
  )
  if (( ${#direct_accum_source_probe_callers[@]} != 2 )) || \
     ! printf '%s\n' "${direct_accum_source_probe_callers[@]}" | rg -q -x "$root_dir/src/mir/compiler/capability.rs" || \
     ! printf '%s\n' "${direct_accum_source_probe_callers[@]}" | rg -q -x "$root_dir/src/mir/compiler/generic_g0_capability.rs"; then
    guard_fail "$tag" "Direct Accum source-unit probe caller set drifted"
  fi
  local direct_accum_physicalizer_production_callers=()
  mapfile -t direct_accum_physicalizer_production_callers < <(
    { rg -l 'physicalize_direct_accum_v1(_with_port)?\(' "$root_dir/src/mir" || true; } \
      | awk -v physicalizer="$root_dir/src/mir/builder/control_flow/plan/loop_accum_physicalizer.rs" \
          '$0 != physicalizer && $0 !~ /_tests\.rs$/'
  )
  if (( ${#direct_accum_physicalizer_production_callers[@]} != 1 )) || \
     [[ "${direct_accum_physicalizer_production_callers[0]}" != "$root_dir/src/mir/builder/resolved_lowering/direct_accum_lowerer.rs" ]]; then
    guard_fail "$tag" "Direct Accum physicalizer caller drifted: expected resolved direct lowerer only"
  fi
  local external_portable_source_files=()
  mapfile -t external_portable_source_files < <(
    { rg -l -w \
        'LoopRecipeArtifactV1|LoopRecipeSourceBindingV1|LoopNodeSourceBindingV1|LoopRecipeSourceOwnerV1|LoopSourcePathV1|LoopSourcePathStepV1|LoopRecipeProvenanceV1' \
        "$root_dir/src/mir" || true; } \
      | awk \
          -v recipe_prefix="$portable_recipe_dir/" \
          -v structural_prefix="$loop_structural_facts_dir/" \
          -v materializer="$loop_phi_materializer" \
          -v materializer_tests="$loop_phi_materializer_tests" \
          -v physical_role_tests="$loop_accum_physical_role_tests" \
          -v binding_ssa_tests="$loop_accum_binding_ssa_tests" \
          -v producer_tests="$loop_recipe_producer_tests" \
          -v nested_producer="$root_dir/src/mir/compiler/nested_predicate_producer.rs" \
          -v nested_producer_tests="$root_dir/src/mir/compiler/nested_predicate_producer_tests.rs" \
          -v callable_recipe_coseal="$root_dir/src/mir/compiler/callable_single_loop_recipe_coseal.rs" \
          -v main0_continue_coseal="$root_dir/src/mir/compiler/main0_continue_recipe_coseal.rs" \
          -v main0_derived_coseal="$root_dir/src/mir/compiler/main0_derived_predicate_recipe_coseal.rs" \
          -v main0_step_coseal="$root_dir/src/mir/compiler/main0_in_body_step_recipe_coseal.rs" \
          -v dynamic_recipe_mod="$dynamic_recipe_mod" \
          -v dynamic_physical_input="$dynamic_physical_input" \
          -v generic_test_prefix="$generic_resolved_test_prefix" \
          'index($0, recipe_prefix) != 1 && index($0, structural_prefix) != 1 && !(index($0, generic_test_prefix) == 1 && $0 ~ /_tests\.rs$/) && $0 != materializer && $0 != materializer_tests  && $0 != physical_role_tests && $0 != binding_ssa_tests && $0 != producer_tests && $0 != nested_producer && $0 != nested_producer_tests && $0 != callable_recipe_coseal && $0 != main0_continue_coseal && $0 != main0_derived_coseal && $0 != main0_step_coseal && $0 != dynamic_recipe_mod && $0 != dynamic_physical_input'
  )
  if (( ${#external_portable_source_files[@]} != 0 )); then
    guard_fail "$tag" "semantic or physical Loop consumer acquired source/provenance authority"
  fi
  local loop_phi_materializer_production
  loop_phi_materializer_production="$(sed '/^#\[cfg(test)\]/,$d' "$loop_phi_materializer")"
  if printf '%s\n' "$loop_phi_materializer_production" | rg -n \
    'LoopRecipeArtifactV1|LoopRecipeVerifierV1|ASTNode|LoopSourcePath|LoopRecipeProvenanceV1|LoopRouteId' \
    >/dev/null; then
    guard_fail "$tag" "Loop PHI materializer production path acquired source/provenance authority"
  fi
  local external_resolved_source_files=()
  mapfile -t external_resolved_source_files < <(
    { rg -l -w 'VerifiedResolvedLoopSourceV1' "$root_dir/src/mir" || true; } \
      | awk \
          -v structural_prefix="$loop_structural_facts_dir/" \
          -v resolved_prefix="$root_dir/src/mir/resolved_semantics/" \
          -v projection="$direct_accum_projection" \
          -v observation_adapter="$direct_accum_observation_adapter" \
          -v nested_observation_adapter="$nested_observation_adapter" \
          -v loop_true_projection="$loop_true_source_projection" \
          -v loop_true_observation_adapter="$loop_true_observation_adapter" \
          -v loop_cond_projection="$loop_cond_source_projection" \
          -v loop_cond_observation_adapter="$loop_cond_observation_adapter" \
          -v generic_g0_observation_adapter="$generic_g0_observation_adapter" \
          -v callable_recipe_coseal="$root_dir/src/mir/compiler/callable_single_loop_recipe_coseal.rs" \
          -v callable_source_map="$root_dir/src/mir/compiler/callable_single_loop_source_map.rs" \
          -v dynamic_recipe_mod="$dynamic_recipe_mod" \
          -v dynamic_physical_input="$dynamic_physical_input" \
          -v callable_facts_prefix="$root_dir/src/mir/builder/normal_callable_loop_source_facts/" \
          -v s6c_tests="$root_dir/src/mir/loop_recipe_contract/s6c_scan_with_init_tests.rs" \
          -v break_composite="$root_dir/src/mir/compiler/loop_break_composite_source_projection.rs" \
          -v break_source="$root_dir/src/mir/compiler/loop_break_source_projection.rs" \
          -v window_probe_tests="$root_dir/src/mir/compiler/loop_family_window_probe_tests.rs" \
          -v main0_continue_map="$root_dir/src/mir/compiler/main0_continue_source_map.rs" \
          -v main0_derived_map="$root_dir/src/mir/compiler/main0_derived_predicate_source_map.rs" \
          -v main0_step_map="$root_dir/src/mir/compiler/main0_in_body_step_source_map.rs" \
          'index($0, structural_prefix) != 1 && index($0, resolved_prefix) != 1 && index($0, callable_facts_prefix) != 1 && $0 != projection && $0 != observation_adapter && $0 != nested_observation_adapter && $0 != loop_true_projection && $0 != loop_true_observation_adapter && $0 != loop_cond_projection && $0 != loop_cond_observation_adapter && $0 != generic_g0_observation_adapter && $0 != callable_recipe_coseal && $0 != callable_source_map && $0 != dynamic_recipe_mod && $0 != dynamic_physical_input && $0 != s6c_tests && $0 != break_composite && $0 != break_source && $0 != window_probe_tests && $0 != main0_continue_map && $0 != main0_derived_map && $0 != main0_step_map'
  )
  if (( ${#external_resolved_source_files[@]} != 0 )); then
    guard_fail "$tag" "sealed resolved Loop source capability escaped its adapter boundary"
  fi
  local loop_route_policy_files=()
  mapfile -t loop_route_policy_files < <(
    find "$loop_route_policy_dir" -maxdepth 1 -name '*.rs' -type f | sort
  )
  guard_require_files "$tag" \
    "$loop_route_policy_dir/README.md" \
    "$loop_route_policy_dir/mod.rs" \
    "$loop_route_policy_dir/schema.rs" \
    "$loop_route_policy_dir/loop_true_break_continue.rs" \
    "$loop_route_policy_dir/policy.rs" \
    "$loop_route_policy_dir/policy_evidence.rs"
  if (( ${#loop_route_policy_files[@]} == 0 )); then
    guard_fail "$tag" "Loop route policy subtree has no Rust files"
  fi
  local loop_route_policy_production_files=()
  for file in "${loop_route_policy_files[@]}"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "file exceeds boundary: ${file#"$root_dir/"} lines=$lines"
    fi
    case "$file" in
      "$loop_route_policy_dir"/*_tests.rs) ;;
      *) loop_route_policy_production_files+=("$file") ;;
    esac
  done
  if rg -n -w \
    'ASTNode|MirBuilder|CanonicalLoopFacts|CorePlan|ValueId|BasicBlockId|MirInstruction|Frag|RouteFn|RouteAttemptOutcomeV1|Retry|LoopRecipeV1|VerifiedLoopRecipeV1|LoopPhysicalizerV1' \
    "${loop_route_policy_production_files[@]}" | rg -v ':[0-9]+:[[:space:]]*//' >/dev/null; then
    guard_fail "$tag" "frozen Loop route policy acquired AST, recipe, retry, or physical authority"
  fi
  if rg -n \
    'builder::control_flow::joinir::route_entry::registry|select_recipe_first_routes|RecipeFirstRouteSelectionV1|\bENTRIES\b|pred_[a-z0-9_]+' \
    "${loop_route_policy_production_files[@]}" >/dev/null; then
    guard_fail "$tag" "frozen Loop route policy imported live registry, selection, or predicate authority"
  fi
  if rg -n \
    'match[[:space:]]+[^\n]*(route_id|LoopRouteId)|LoopRouteId::[A-Za-z0-9_]+[[:space:]]*=>' \
    "${loop_route_policy_production_files[@]}" >/dev/null; then
    guard_fail "$tag" "opaque Loop route provenance acquired dispatch authority"
  fi
  # M12-R2C part 2 retired the synthetic 19-row schedule and the winner
  # ceremony. These names must stay absent from src/ so the deleted authority
  # cannot be reintroduced.
  local retired_schedule_hits
  retired_schedule_hits="$(
    { rg -l -g '*.rs' 'evaluate_frozen_loop_route_schedule_v1|freeze_loop_route_schedule_v1|FrozenLoopRouteScheduleV1|FrozenLoopRouteRowV1|VerifiedLoopPolicyWinnerV1' \
      "$root_dir/src" || true; } | wc -l | tr -d '[:space:]'
  )"
  if [[ "$retired_schedule_hits" != "0" ]]; then
    guard_fail "$tag" "retired frozen Loop schedule/winner authority reappeared in src/"
  fi
  if rg -n -U \
    '#\[derive\([^]]*Clone[^]]*\)\][[:space:]]*\npub\(crate\) struct (FrozenLoopRouteScheduleV1|FrozenLoopRouteRowV1)' \
    "$loop_route_policy_dir/schema.rs" >/dev/null; then
    guard_fail "$tag" "frozen Loop route schedule or row became Clone"
  fi
  for file in "${files[@]}"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "file exceeds boundary: ${file#"$root_dir/"} lines=$lines"
    fi
  done
  local logical_files=(
    "$projection"
    "$simple_while"
    "$accum_const"
    "$loop_break_facts"
  )
  for file in "${logical_files[@]}"; do
    if rg -n -w \
      'MirBuilder|CorePlan|ValueId|BasicBlockId|MirInstruction|Phi|Frag|RouteFn|ComposeFn|try_execute_recipe_first_selection|select_recipe_first_routes|ENTRIES' \
      "$file" >/dev/null; then
      guard_fail "$tag" "logical loop demand/provenance acquired physical or selection authority: ${file#"$root_dir/"}"
    fi
  done
}

# The portable If contract shares this lane guard with the Loop contract.  The
# D0-C seam permits exactly one named adapter outside the contract subtree;
# every other logical/physical-input constructor remains caller-zero.
guard_joinir_if_recipe_contract() {
  local root_dir="$1"
  local tag="$2"
  local contract_dir="$root_dir/src/mir/if_recipe_contract"
  local join_sig="$contract_dir/join_sig.rs"
  local physical_input="$contract_dir/physical_input.rs"
  local tests="$contract_dir/tests.rs"
  local adapter="$root_dir/src/mir/builder/resolved_lowering/if_recipe_adapter.rs"
  local physicalizer="$root_dir/src/mir/builder/resolved_lowering/trivial_ssa/if_recipe_physicalizer.rs"
  local lowerer="$root_dir/src/mir/builder/resolved_lowering/trivial_ssa/lowerer.rs"
  local materialization="$root_dir/src/mir/builder/resolved_lowering/trivial_ssa/lowerer/if_materialization.rs"
  local files=(
    "$contract_dir/README.md"
    "$contract_dir/mod.rs"
    "$contract_dir/error.rs"
    "$contract_dir/ids.rs"
    "$join_sig"
    "$contract_dir/normalize.rs"
    "$physical_input"
    "$contract_dir/schema.rs"
    "$contract_dir/source_binding.rs"
    "$contract_dir/verify.rs"
    "$tests"
    "$adapter"
    "$physicalizer"
    "$lowerer"
    "$materialization"
  )
  local file lines

  guard_require_files "$tag" "${files[@]}"
  for file in "${files[@]}"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "If recipe contract file exceeds boundary: ${file#"$root_dir/"} lines=$lines"
    fi
  done
  if rg -n -w \
    'ASTNode|MirBuilder|CorePlan|ValueId|BasicBlockId|IfCfgSessionV1|RouteAttemptOutcomeV1|Retry|CanonicalSsaFunctionSessionV2|PhiTxn|BindingSsaBuilderV1' \
    "$adapter" >/dev/null; then
    guard_fail "$tag" "D0-C If adapter acquired AST, Builder, route, or physical SSA authority"
  fi

  # `schema.rs` legitimately uses Option for the explicit/implicit else
  # disposition.  The physical/logical proof files must not acquire it.
  if rg -n -w \
    'MirBuilder|CorePlan|ValueId|BasicBlockId|CanonicalCfgSession|PhiTxn|BindingSsaBuilderV1|ASTNode|RouteAttemptOutcomeV1|Retry|Option' \
    "$join_sig" "$physical_input" >/dev/null; then
    guard_fail "$tag" "If logical/physical-input proof acquired physical, AST, route, retry, or Option authority"
  fi
  if rg -n -w \
    'ASTNode|MirBuilder|CorePlan|RouteAttemptOutcomeV1|Retry|Option|new_ssa|new_phi|lower_if_legacy_unselected|lower_if_materialization_core|ResolvedIfElsePortV1|else_port|Some\(true\)' \
    "$physicalizer" >/dev/null; then
    guard_fail "$tag" "If physicalizer acquired route, retry, raw AST, or a second physical owner"
  fi
  if ! rg -q 'lower_if_recipe_selected\(' "$physicalizer" || \
     rg -n 'lower_if_legacy_unselected|lower_if_materialization_core|lower_if_materialization\(' \
       "$physicalizer" >/dev/null; then
    guard_fail "$tag" "selected If physicalizer must use only the named selected helper"
  fi
  if ! rg -q 'fn lower_if_recipe_selected\(' "$materialization" || \
     ! rg -q 'fn lower_if_legacy_unselected\(' "$materialization" || \
     ! rg -q 'IfMaterializationTopologyV1::Selected' "$materialization"; then
    guard_fail "$tag" "If lowerer lost the selected/legacy shape-scoped helper split"
  fi
  if rg -n -U \
    '#\[derive\([^)]*Clone[^)]*\)\][[:space:]]*\n[[:space:]]*(pub\(crate\)[[:space:]]+)?struct[[:space:]]+(VerifiedIfJoinSigV1|VerifiedIfPhysicalInputV1)' \
    "$join_sig" "$physical_input" >/dev/null || \
     rg -n 'impl[[:space:]]+Clone[[:space:]]+for[[:space:]]+(VerifiedIfJoinSigV1|VerifiedIfPhysicalInputV1)' \
       "$join_sig" "$physical_input" >/dev/null; then
    guard_fail "$tag" "verified If logical/physical-input wrappers must remain non-Clone"
  fi
  if rg -n 'pub[[:space:]]+(artifact|join_sig)[[:space:]]*:' "$physical_input" >/dev/null; then
    guard_fail "$tag" "physical-input artifact/signature fields must remain private"
  fi
  if [[ "$(rg -c 'pub\(crate\) fn from_artifact\(' "$physical_input" || true)" != "1" ]]; then
    guard_fail "$tag" "physical-input must have exactly one consuming from_artifact issuer"
  fi
  if rg -n 'pub\(crate\) fn (new|from_parts|from_signature|from_join)' "$physical_input" >/dev/null; then
    guard_fail "$tag" "independent artifact/signature constructor appeared"
  fi

  local symbol_files=()
  mapfile -t symbol_files < <(
    { rg -l -w 'VerifiedIfJoinSigV1|VerifiedIfPhysicalInputV1|IfJoinSigElaboratorV1' \
        "$root_dir/src/mir" || true; }
  )
  for file in "${symbol_files[@]}"; do
    if [[ "$file" != "$contract_dir"/* && "$file" != "$adapter" && "$file" != "$physicalizer" ]]; then
      guard_fail "$tag" "If logical/physical-input symbol escaped caller-zero subtree: ${file#"$root_dir/"}"
    fi
  done

  local physical_callers=()
  mapfile -t physical_callers < <(
    { rg -l 'VerifiedIfPhysicalInputV1::from_artifact\(' "$root_dir/src/mir" || true; }
  )
  if (( ${#physical_callers[@]} != 2 )) || \
     ! printf '%s\n' "${physical_callers[@]}" | rg -q "^${tests}$" || \
     ! printf '%s\n' "${physical_callers[@]}" | rg -q "^${adapter}$"; then
    guard_fail "$tag" "physical-input issuer must have only contract tests plus the named D0-C adapter"
  fi
  local mapper_callers=()
  mapfile -t mapper_callers < <(
    { rg -l 'map_trivial_if_recipe_v1\(' "$root_dir/src/mir" || true; } |
      while IFS= read -r file; do
        [[ "$file" == "$root_dir/src/mir/resolved_value_profile/recipe_mapper.rs" ]] && continue
        [[ "$file" == *"/tests.rs" || "$file" == *"_tests.rs" ]] && continue
        printf '%s\n' "$file"
      done
  )
  if (( ${#mapper_callers[@]} != 1 )) || [[ "${mapper_callers[0]}" != "$adapter" ]]; then
    guard_fail "$tag" "If recipe mapper must have exactly one production caller: the named D0-C adapter"
  fi
  local physicalizer_callers=()
  mapfile -t physicalizer_callers < <(
    { rg -l 'physicalize_if_recipe_v1\(' "$root_dir/src/mir" || true; } \
      | awk -v physicalizer="$physicalizer" '$0 != physicalizer && $0 !~ /_tests\.rs$/'
  )
  if (( ${#physicalizer_callers[@]} != 1 )) || [[ "${physicalizer_callers[0]}" != "$materialization" ]]; then
    guard_fail "$tag" "If physicalizer must have exactly one production caller: trivial SSA lowerer"
  fi
  local selected_helper_callers=()
  mapfile -t selected_helper_callers < <(
    { rg -l 'lower_if_recipe_selected\(' "$root_dir/src/mir" || true; } \
      | awk -v materialization="$materialization" '$0 != materialization && $0 !~ /_tests\.rs$/'
  )
  if (( ${#selected_helper_callers[@]} != 1 )) || [[ "${selected_helper_callers[0]}" != "$physicalizer" ]]; then
    guard_fail "$tag" "selected If helper must have exactly one production caller: physicalizer"
  fi
  if ! rg -q 'physical_input\.into_parts\(' "$physicalizer" || \
     ! rg -q 'Result<CanonicalIfPhysicalSuccessV1' "$physicalizer"; then
    guard_fail "$tag" "If physicalizer must consume the paired payload and return Result-only success"
  fi
  local physical_input_part_callers=()
  mapfile -t physical_input_part_callers < <(
    { rg -l 'physical_input\.into_parts\(' "$root_dir/src/mir" || true; } \
      | awk -v physicalizer="$physicalizer" '$0 != physicalizer && $0 !~ /_tests\.rs$/'
  )
  if (( ${#physical_input_part_callers[@]} != 0 )); then
    guard_fail "$tag" "If physical-input payload must be unpacked only by the named physicalizer"
  fi
  if rg -n 'claim_if\(|Pending\(_physical_input\)|drop\([^)]*physical_input' "$adapter" >/dev/null; then
    guard_fail "$tag" "If selected demand payload was silently dropped or claim-only"
  fi
  local elaborator_callers=()
  mapfile -t elaborator_callers < <(
    { rg -l 'IfJoinSigElaboratorV1::elaborate\(' "$root_dir/src/mir" || true; }
  )
  for file in "${elaborator_callers[@]}"; do
    if [[ "$file" != "$contract_dir"/* ]]; then
      guard_fail "$tag" "If JoinSig elaborator acquired an external caller: ${file#"$root_dir/"}"
    fi
  done
}
